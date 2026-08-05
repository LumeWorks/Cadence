// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Pipeline render + ánh xạ con trỏ từ raw thao tác sang output grapheme.
//!
//! Phase 3: lịch sử raw được phân đoạn theo loại ký tự (`phan_doan`). Mỗi đoạn
//! chữ (Chu) chạy Telex độc lập; mọi đoạn khác render nguyên bản. Việc này
//! ngăn phím dấu thanh/hình chữ xuyên qua ranh giới từ, cho phép code, URL,
//! command và tiếng Việt trộn trong cùng phiên.
//!
//! ```text
//! lịch sử → phan_doan → Vec<Doan>
//!   → mỗi đoạn: (Telex | raw) → substring + local raw_to_byte
//!   → ghép → output toàn cục + raw_to_byte toàn cục → navigable → snapshot
//! ```

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use unicode_segmentation::UnicodeSegmentation;

use crate::cau_hinh::{ChinhSachLuaChon, DangUnicode, KieuTelex, QuyTacDatDau};
use crate::loai_noi_dung::LoaiNoiDung;
use crate::lua_chon;
use crate::ngu_canh;
use crate::phan_doan::{self, Doan, LoaiDoan};
use crate::render;
use crate::telex::{self, DonViRender, NoiDungDonVi};
use crate::thao_tac::ThaoTacNhap;

/// Kết quả của một lần dựng lại snapshot.
pub(crate) struct KetQuaRender {
    /// Chuỗi output đã render (theo dạng Unicode đã chọn).
    pub(crate) noi_dung: String,
    /// Ánh xạ raw position → byte offset trong output (size = n+1).
    pub(crate) raw_to_byte: Vec<usize>,
    /// Các raw position là ranh giới grapheme navigable (sắp xếp tăng).
    pub(crate) navigable: Vec<usize>,
    /// Loại nội dung.
    pub(crate) loai_noi_dung: LoaiNoiDung,
    /// Trace bước quyết định (chỉ khi feature `trace`).
    #[cfg(feature = "trace")]
    pub(crate) trace: alloc::vec::Vec<crate::trace::TraceStep>,
}

/// Kết quả render một đoạn.
struct RenderDoan {
    /// Chuỗi output của đoạn.
    chuoi: String,
    /// Ánh xạ raw position nội bộ (0..=len) → byte offset trong `chuoi`.
    map: Vec<usize>,
    /// Loại nội dung của đoạn.
    loai: LoaiNoiDung,
}

/// Dựng lại toàn bộ snapshot từ lịch sử thao tác.
pub(crate) fn xay_lai(
    thao_tac: &[ThaoTacNhap],
    dang: DangUnicode,
    kieu_telex: KieuTelex,
    quy_tac: QuyTacDatDau,
    chinh_sach: ChinhSachLuaChon,
) -> KetQuaRender {
    let cac_doan = phan_doan::phan_doan(thao_tac, kieu_telex);
    let nhan_dien = ngu_canh::nhan_dien(&cac_doan, thao_tac);

    let mut noi_dung = String::new();
    let mut raw_to_byte = vec![0usize; thao_tac.len() + 1];
    let mut co_bien_doi = false;
    let mut co_am_tiet = false;
    #[cfg(feature = "trace")]
    let mut trace_steps: alloc::vec::Vec<crate::trace::TraceStep> = alloc::vec::Vec::new();

    for (vi_tri, doan) in cac_doan.iter().enumerate() {
        let slice = &thao_tac[doan.bat_dau..doan.ket_thuc];
        let r = render_doan(
            doan,
            slice,
            dang,
            kieu_telex,
            quy_tac,
            chinh_sach,
            nhan_dien[vi_tri].bat_buoc_raw,
        );
        #[cfg(feature = "trace")]
        {
            use crate::trace::TraceKetQua;
            let chuoi_raw: String = slice.iter().map(|t| t.ky_tu).collect();
            let ket_qua = if matches!(
                r.loai,
                LoaiNoiDung::BienDoiTelex | LoaiNoiDung::AmTietTiengViet
            ) {
                TraceKetQua::Telex
            } else {
                TraceKetQua::NguyenBan
            };
            trace_steps.push(crate::trace::TraceStep {
                doan_bat_dau: doan.bat_dau,
                doan_ket_thuc: doan.ket_thuc,
                bang_chung: nhan_dien[vi_tri].bang_chung,
                ket_qua,
                chuoi_raw,
                chuoi_ra: r.chuoi.clone(),
            });
        }
        let bat_dau_byte = noi_dung.len();
        // Điền raw_to_byte toàn cục từ map nội bộ.
        for (i, &local_byte) in r.map.iter().enumerate() {
            raw_to_byte[doan.bat_dau + i] = bat_dau_byte + local_byte;
        }
        noi_dung.push_str(&r.chuoi);
        match r.loai {
            LoaiNoiDung::BienDoiTelex => co_bien_doi = true,
            LoaiNoiDung::AmTietTiengViet => co_am_tiet = true,
            _ => {}
        }
    }

    let loai_noi_dung = if thao_tac.is_empty() {
        LoaiNoiDung::Trong
    } else if co_bien_doi {
        LoaiNoiDung::BienDoiTelex
    } else if co_am_tiet {
        LoaiNoiDung::AmTietTiengViet
    } else {
        LoaiNoiDung::NguyenBan
    };

    let navigable = tinh_navigable(&noi_dung, &raw_to_byte);
    KetQuaRender {
        noi_dung,
        raw_to_byte,
        navigable,
        loai_noi_dung,
        #[cfg(feature = "trace")]
        trace: trace_steps,
    }
}

/// Render một đoạn theo loại. Đoạn `Chu` chạy Telex (trừ teencode lặp hoặc
/// bị buộc raw bởi nhận diện ngữ cảnh); mọi đoạn khác render nguyên bản.
fn render_doan(
    doan: &Doan,
    slice: &[ThaoTacNhap],
    dang: DangUnicode,
    kieu_telex: KieuTelex,
    quy_tac: QuyTacDatDau,
    chinh_sach: ChinhSachLuaChon,
    bat_buoc_raw: bool,
) -> RenderDoan {
    match doan.loai {
        LoaiDoan::Chu => render_chu(slice, dang, kieu_telex, quy_tac, chinh_sach, bat_buoc_raw),
        // NguyenBan/non-Chu: as-is (giữ nguyên, không normalize). Các ký tự
        // này (ASCII, emoji, combining mark) không thay đổi khi normalize.
        _ => render_nguyen_ban(slice),
    }
}

/// Render một đoạn chữ qua Telex, chọn raw/Telex theo `lua_chon`.
///
/// Teencode lặp (3+ chữ cái hình chữ doubled-base có chữ khác trước) hoặc
/// đoạn bị buộc raw bởi nhận diện ngữ cảnh (`::`, `=`, URL, ...) được bảo
/// toàn raw trước khi chạy Telex. Khi fallback raw, dùng `render_chu`
/// (normalize) để NFC/NFD canonical equivalent giữ đúng.
fn render_chu(
    slice: &[ThaoTacNhap],
    dang: DangUnicode,
    kieu_telex: KieuTelex,
    quy_tac: QuyTacDatDau,
    chinh_sach: ChinhSachLuaChon,
    bat_buoc_raw: bool,
) -> RenderDoan {
    if bat_buoc_raw || phan_doan::la_teencode_lap(slice) {
        return render_raw_chu(slice, dang);
    }
    let ket_qua_telex = telex::xu_ly_doan_chu(slice, kieu_telex, quy_tac);
    let don_vi = &ket_qua_telex.don_vi;
    let lua_chon = lua_chon::lua_chon(
        don_vi,
        "",
        ket_qua_telex.co_escape,
        ket_qua_telex.co_escape_hinh_chu,
        false,
        chinh_sach,
    );
    match lua_chon {
        lua_chon::KetQuaLuaChon::Telex => {
            let (chuoi, byte_len) = render_don_vi_list(don_vi, dang);
            let map = tinh_raw_to_byte(don_vi, &byte_len, slice.len());
            let loai = loai_noi_dung_chu(&chuoi, slice, don_vi);
            RenderDoan { chuoi, map, loai }
        }
        lua_chon::KetQuaLuaChon::NguyenBan => render_raw_chu(slice, dang),
    }
}

/// Render raw một đoạn chữ qua `render_chu` (normalize NFC/NFD). Map 1:1
/// theo byte, tính byte offset sau mỗi `render_chu` (có thể nhiều byte
/// trong NFD).
fn render_raw_chu(slice: &[ThaoTacNhap], dang: DangUnicode) -> RenderDoan {
    let mut chuoi = String::new();
    let mut map = Vec::with_capacity(slice.len() + 1);
    for t in slice {
        map.push(chuoi.len());
        let chu = match render::phan_tich_ky_tu(t.ky_tu) {
            Some(c) => c,
            None => crate::chu_viet::ChuCaiViet::thuong(t.ky_tu),
        };
        chuoi.push_str(&render::render_chu(&chu, dang));
    }
    map.push(chuoi.len());
    let raw: String = slice.iter().map(|t| t.ky_tu).collect();
    let loai = loai_noi_dung_cua(&chuoi, &raw);
    RenderDoan { chuoi, map, loai }
}

/// Render nguyên bản as-is: mỗi ký tự push không normalize, map 1:1 theo
/// byte. Dùng cho `them_nguyen_ban` và các đoạn không phải chữ (số, khoảng
/// trắng, dấu câu, kỹ thuật, emoji).
fn render_nguyen_ban(slice: &[ThaoTacNhap]) -> RenderDoan {
    let mut chuoi = String::new();
    let mut map = Vec::with_capacity(slice.len() + 1);
    for t in slice {
        map.push(chuoi.len());
        chuoi.push(t.ky_tu);
    }
    map.push(chuoi.len());
    let raw: String = slice.iter().map(|t| t.ky_tu).collect();
    let loai = loai_noi_dung_cua(&chuoi, &raw);
    RenderDoan { chuoi, map, loai }
}

/// Render danh sách đơn vị Telex ra chuỗi, trả kèm byte length mỗi đơn vị.
fn render_don_vi_list(don_vi: &[DonViRender], dang: DangUnicode) -> (String, Vec<usize>) {
    let mut s = String::new();
    let mut byte_len = Vec::with_capacity(don_vi.len());
    for u in don_vi {
        let r = render_don_vi(u, dang);
        byte_len.push(r.len());
        s.push_str(&r);
    }
    (s, byte_len)
}

/// Render một đơn vị ra chuỗi theo dạng Unicode.
fn render_don_vi(u: &DonViRender, dang: DangUnicode) -> String {
    match &u.noi_dung {
        NoiDungDonVi::Chu(chu) => render::render_chu(chu, dang),
        NoiDungDonVi::Chuong(c) => {
            // Literal: giữ nguyên, không normalize (emoji, dấu câu, ASCII).
            let mut s = String::new();
            s.push(*c);
            s
        }
    }
}

/// Tính `raw_to_byte`: ánh xạ raw position → byte offset.
///
/// Vị trí interior của đơn vị snap về ranh giới gần nhất. Vị trí gap (tone
/// key consumed, không thuộc đơn vị nào) kế thừa byte offset của vị trí
/// trước nó.
fn tinh_raw_to_byte(don_vi: &[DonViRender], byte_len: &[usize], n: usize) -> Vec<usize> {
    // Bước 1: đánh dấu byte offset cho ranh giới đơn vị.
    let mut pos_to_byte: Vec<Option<usize>> = vec![None; n + 1];
    let mut byte_offset = 0usize;
    for (u, &blen) in don_vi.iter().zip(byte_len.iter()) {
        pos_to_byte[u.raw_bat_dau] = Some(byte_offset);
        byte_offset += blen;
        pos_to_byte[u.raw_ket_thuc] = Some(byte_offset);
    }

    // Bước 2: snap interior positions về ranh giới gần nhất.
    for (u, &blen) in don_vi.iter().zip(byte_len.iter()) {
        let start_byte = pos_to_byte[u.raw_bat_dau].unwrap_or(0);
        let end_byte = start_byte + blen;
        for (r, slot) in pos_to_byte.iter_mut().enumerate() {
            if r <= u.raw_bat_dau || r >= u.raw_ket_thuc || slot.is_some() {
                continue;
            }
            let dist_start = r - u.raw_bat_dau;
            let dist_end = u.raw_ket_thuc - r;
            *slot = Some(if dist_end < dist_start {
                end_byte
            } else {
                start_byte
            });
        }
    }

    // Bước 3: điền gap (tone key, escape consumed) bằng byte offset trước.
    let mut raw_to_byte = vec![0usize; n + 1];
    let mut last_byte = 0usize;
    for r in 0..=n {
        match pos_to_byte[r] {
            Some(b) => {
                raw_to_byte[r] = b;
                last_byte = b;
            }
            None => {
                raw_to_byte[r] = last_byte;
            }
        }
    }
    raw_to_byte
}

/// Tính các raw position là ranh giới grapheme navigable.
///
/// Một raw position là navigable nếu byte offset của nó là ranh giới
/// grapheme và khác byte offset của raw position ngay trước nó (mới đạt
/// một grapheme mới).
fn tinh_navigable(noi_dung: &str, raw_to_byte: &[usize]) -> Vec<usize> {
    let mut ranh_gioi_grapheme: Vec<usize> =
        noi_dung.grapheme_indices(true).map(|(i, _)| i).collect();
    ranh_gioi_grapheme.push(noi_dung.len());
    let mut ket_qua = Vec::new();
    let mut byte_truoc: Option<usize> = None;
    for (r, &byte) in raw_to_byte.iter().enumerate() {
        if ranh_gioi_grapheme.contains(&byte) && Some(byte) != byte_truoc {
            ket_qua.push(r);
            byte_truoc = Some(byte);
        }
    }
    ket_qua
}

/// Snap một raw position (có thể interior) về navigable gần nhất.
/// Tie → forward (raw position lớn hơn).
pub(crate) fn snap_raw(r: usize, navigable: &[usize]) -> usize {
    match navigable.binary_search(&r) {
        Ok(_) => r,
        Err(pos) => {
            let truoc = if pos == 0 {
                None
            } else {
                navigable.get(pos - 1).copied()
            };
            let sau = navigable.get(pos).copied();
            match (truoc, sau) {
                (Some(a), Some(b)) => {
                    let da = r - a;
                    let db = b - r;
                    if db < da { b } else { a }
                }
                (Some(a), None) => a,
                (None, Some(b)) => b,
                (None, None) => r,
            }
        }
    }
}

/// Di chuyển con trỏ raw về trái một grapheme (raw position navigable trước).
pub(crate) fn di_trai_raw(r: usize, navigable: &[usize]) -> usize {
    let r_snapped = snap_raw(r, navigable);
    match navigable.binary_search(&r_snapped) {
        Ok(pos) if pos > 0 => navigable[pos - 1],
        _ => r_snapped,
    }
}

/// Di chuyển con trỏ raw về phải một grapheme (raw position navigable sau).
pub(crate) fn di_phai_raw(r: usize, navigable: &[usize]) -> usize {
    let r_snapped = snap_raw(r, navigable);
    match navigable.binary_search(&r_snapped) {
        Ok(pos) => {
            if pos + 1 < navigable.len() {
                navigable[pos + 1]
            } else {
                r_snapped
            }
        }
        Err(pos) => navigable.get(pos).copied().unwrap_or(r_snapped),
    }
}

/// Byte offset trong output cho raw position (đã snap).
pub(crate) fn byte_tai(r: usize, raw_to_byte: &[usize]) -> usize {
    raw_to_byte[r.min(raw_to_byte.len() - 1)]
}

/// Xác định loại nội dung của một đoạn chữ Telex. Nâng cấp lên
/// `AmTietTiengViet` nếu output biến đổi và de-tone base là âm tiết hợp lệ.
fn loai_noi_dung_chu(chuoi: &str, slice: &[ThaoTacNhap], don_vi: &[DonViRender]) -> LoaiNoiDung {
    let raw: String = slice.iter().map(|t| t.ky_tu).collect();
    let loai = loai_noi_dung_cua(chuoi, &raw);
    if loai == LoaiNoiDung::BienDoiTelex {
        let base = lua_chon::render_de_tu_don_vi(don_vi);
        if matches!(
            crate::am_tiet::phan_tich_am_tiet(&base),
            crate::am_tiet::MucHopLe::CoTheTiepTuc
        ) {
            LoaiNoiDung::AmTietTiengViet
        } else {
            loai
        }
    } else {
        loai
    }
}

/// Xác định loại nội dung từ output và raw.
fn loai_noi_dung_cua(noi_dung: &str, raw: &str) -> LoaiNoiDung {
    if raw.is_empty() {
        LoaiNoiDung::Trong
    } else if noi_dung == raw {
        LoaiNoiDung::NguyenBan
    } else {
        LoaiNoiDung::BienDoiTelex
    }
}
