// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Pipeline render + ánh xạ con trỏ từ raw thao tác sang output grapheme.
//!
//! Sau khi Telex biến đổi raw history thành `DonViRender`, module này:
//!
//! ```text
//! don_vi → render chuỗi output → raw_to_byte → navigable → con trỏ công khai
//! ```

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use unicode_segmentation::UnicodeSegmentation;

use crate::cau_hinh::{DangUnicode, KieuTelex, QuyTacDatDau};
use crate::loai_noi_dung::LoaiNoiDung;
use crate::lua_chon;
use crate::render;
use crate::telex::{DonViRender, NoiDungDonVi};
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
}

/// Dựng lại toàn bộ snapshot từ lịch sử thao tác.
pub(crate) fn xay_lai(
    thao_tac: &[ThaoTacNhap],
    dang: DangUnicode,
    kieu_telex: KieuTelex,
    quy_tac: QuyTacDatDau,
) -> KetQuaRender {
    let ket_qua_telex = crate::telex::xu_ly_doan_chu(thao_tac, kieu_telex, quy_tac);
    let don_vi = ket_qua_telex.don_vi;
    let co_escape = ket_qua_telex.co_escape;
    let co_escape_hinh_chu = ket_qua_telex.co_escape_hinh_chu;
    let noi_dung_goc: String = thao_tac.iter().map(|t| t.ky_tu).collect();

    // Lựa chọn raw vs Telex cho toàn đoạn.
    let lua_chon = lua_chon::lua_chon(&don_vi, &noi_dung_goc, co_escape, co_escape_hinh_chu);
    let (noi_dung, raw_to_byte): (String, Vec<usize>) = match lua_chon {
        lua_chon::KetQuaLuaChon::Telex => {
            let mut s = String::new();
            let mut byte_len = Vec::with_capacity(don_vi.len());
            for u in &don_vi {
                let r = render_don_vi(u, dang);
                byte_len.push(r.len());
                s.push_str(&r);
            }
            let n = thao_tac.len();
            (s, tinh_raw_to_byte(&don_vi, &byte_len, n))
        }
        lua_chon::KetQuaLuaChon::NguyenBan => {
            // Render raw từng ký tự (chỉ normalize dạng Unicode). Mỗi raw
            // position maps trực tiếp: r → byte offset của ký tự r.
            let mut s = String::new();
            let mut map = vec![0usize; thao_tac.len() + 1];
            for (i, t) in thao_tac.iter().enumerate() {
                map[i] = s.len();
                let chu = match render::phan_tich_ky_tu(t.ky_tu) {
                    Some(c) => c,
                    None => crate::chu_viet::ChuCaiViet::thuong(t.ky_tu),
                };
                s.push_str(&render::render_chu(&chu, dang));
            }
            map[thao_tac.len()] = s.len();
            (s, map)
        }
    };
    let navigable = tinh_navigable(&noi_dung, &raw_to_byte);
    let loai_noi_dung = loai_noi_dung_cua(&noi_dung, &noi_dung_goc);
    // Nâng cấp lên `AmTietTiengViet` nếu output là âm tiết hợp lệ.
    let loai_noi_dung = if loai_noi_dung == LoaiNoiDung::BienDoiTelex {
        let base = lua_chon::render_de_tu_don_vi(&don_vi);
        if matches!(
            crate::am_tiet::phan_tich_am_tiet(&base),
            crate::am_tiet::MucHopLe::CoTheTiepTuc
        ) {
            LoaiNoiDung::AmTietTiengViet
        } else {
            loai_noi_dung
        }
    } else {
        loai_noi_dung
    };
    KetQuaRender {
        noi_dung,
        raw_to_byte,
        navigable,
        loai_noi_dung,
    }
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

/// Xác định loại nội dung từ output và raw.
fn loai_noi_dung_cua(noi_dung: &str, noi_dung_goc: &str) -> LoaiNoiDung {
    if noi_dung_goc.is_empty() {
        LoaiNoiDung::Trong
    } else if noi_dung == noi_dung_goc {
        LoaiNoiDung::NguyenBan
    } else {
        LoaiNoiDung::BienDoiTelex
    }
}
