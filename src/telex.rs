// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Bộ nhận Telex — biến đổi raw input thành đơn vị render có provenance.
//!
//! Module này là tầng giữa: nhận các thao tác raw trong một đoạn chữ, áp
//! dụng rule Telex (hình chữ, dấu thanh, escape) và xuất ra `DonViRender`
//! mang theo provenance (thao tác raw nào sinh ra đơn vị này).

use alloc::vec::Vec;

use crate::cau_hinh::{KieuTelex, QuyTacDatDau};
use crate::chu_viet::{ChuCaiViet, ChuGoc, DauChu, DauThanh, KieuHoa};
use crate::render;
use crate::thao_tac::{CachNhap, ThaoTacNhap};

/// Nội dung một đơn vị render.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NoiDungDonVi {
    /// Chữ cái Việt đã biến đổi.
    Chu(ChuCaiViet),
    /// Ký tự literal (không phải Telex, hoặc đã escape).
    Chuong(char),
}

/// Một đơn vị render: một grapheme trong output kèm provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DonViRender {
    /// Nội dung (chữ biến đổi hoặc literal).
    pub(crate) noi_dung: NoiDungDonVi,
    /// Vị trí raw đầu (inclusive).
    pub(crate) raw_bat_dau: usize,
    /// Vị trí raw cuối (exclusive).
    pub(crate) raw_ket_thuc: usize,
    /// Thao tác raw ảnh hưởng thêm (dấu thanh) nhưng không nằm trong
    /// khoảng liên tục.
    pub(crate) thao_tac_anh_huong: Vec<usize>,
}

impl DonViRender {
    /// Tạo đơn vị literal từ một ký tự, chiếm đúng một thao tác raw.
    pub(crate) fn chuong(ky_tu: char, raw: usize) -> Self {
        Self {
            noi_dung: NoiDungDonVi::Chuong(ky_tu),
            raw_bat_dau: raw,
            raw_ket_thuc: raw + 1,
            thao_tac_anh_huong: Vec::new(),
        }
    }

    /// Tạo đơn vị chữ biến đổi, chiếm khoảng raw.
    pub(crate) fn chu(chu: ChuCaiViet, bat_dau: usize, ket_thuc: usize) -> Self {
        Self {
            noi_dung: NoiDungDonVi::Chu(chu),
            raw_bat_dau: bat_dau,
            raw_ket_thuc: ket_thuc,
            thao_tac_anh_huong: Vec::new(),
        }
    }
}

/// Bảng cap hình chữ: tra (chữ gốc, dấu chữ) từ cặp (base, modifier).
fn cap_hinh_chu(base: char, modifier: char) -> Option<(ChuGoc, DauChu)> {
    let b = base.to_ascii_lowercase();
    let m = modifier.to_ascii_lowercase();
    use ChuGoc::{A, D, E, O, U};
    use DauChu::{Gach, Moc, Mu, Trang};
    match (b, m) {
        ('a', 'a') => Some((A, Mu)),
        ('a', 'w') => Some((A, Trang)),
        ('e', 'e') => Some((E, Mu)),
        ('o', 'o') => Some((O, Mu)),
        ('o', 'w') => Some((O, Moc)),
        ('u', 'w') => Some((U, Moc)),
        ('d', 'd') => Some((D, Gach)),
        _ => None,
    }
}

/// Tra dấu thanh từ phím Telex. `z` trả `Khong` (xóa dấu).
fn tu_dau_thanh_key(c: char) -> Option<DauThanh> {
    match c.to_ascii_lowercase() {
        's' => Some(DauThanh::Sac),
        'f' => Some(DauThanh::Huyen),
        'r' => Some(DauThanh::Hoi),
        'x' => Some(DauThanh::Nga),
        'j' => Some(DauThanh::Nang),
        'z' => Some(DauThanh::Khong),
        _ => None,
    }
}

/// Trả `true` nếu `c` là phím dấu thanh (s/f/r/x/j/z).
fn la_phim_dau_thanh(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 's' | 'f' | 'r' | 'x' | 'j' | 'z')
}

/// Tìm index của đơn vị nguyên âm cuối cùng trong `don_vi` (từ `min_raw` trở đi).
fn tim_nguyen_am_cuoi(don_vi: &[DonViRender], min_raw: usize) -> Option<usize> {
    don_vi
        .iter()
        .enumerate()
        .filter(|(_, u)| u.raw_bat_dau >= min_raw)
        .rev()
        .find(|(_, u)| match &u.noi_dung {
            NoiDungDonVi::Chu(chu) => chu.chu_goc.la_nguyen_am(),
            NoiDungDonVi::Chuong(_) => false,
        })
        .map(|(i, _)| i)
}

/// Tìm index của nguyên âm chính (nguyên âm mang dấu thanh).
///
/// Quy tắc Phase 2:
/// 1. Bán âm cuối: nếu nguyên âm cuối là `i`/`u` không dấu hình chữ và có
///    nguyên âm trước → dấu trên nguyên âm trước.
/// 2. On-glide `o`+`a`/`e`: `HienDai` đặt trên `o`, `TruyenThong` đặt trên
///    `a`/`e`.
/// 3. Ngược lại, đặt trên nguyên âm cuối.
fn tim_nguyen_am_chinh(
    don_vi: &[DonViRender],
    quy_tac: QuyTacDatDau,
    min_raw: usize,
) -> Option<usize> {
    let cac_nguyen_am: Vec<usize> = don_vi
        .iter()
        .enumerate()
        .filter(|(_, u)| u.raw_bat_dau >= min_raw)
        .filter(|(_, u)| match &u.noi_dung {
            NoiDungDonVi::Chu(chu) => chu.chu_goc.la_nguyen_am(),
            NoiDungDonVi::Chuong(_) => false,
        })
        .map(|(i, _)| i)
        .collect();
    if cac_nguyen_am.is_empty() {
        return None;
    }
    let cuoi = *cac_nguyen_am.last().unwrap_or(&0);
    if cac_nguyen_am.len() >= 2 {
        if let NoiDungDonVi::Chu(chu) = &don_vi[cuoi].noi_dung {
            // Bán âm cuối: `i`, `u`, `o` (không dấu hình chữ) → tone trên
            // nguyên âm trước. `o` base là off-glide trong `ao`, `eo`, `ưo`.
            if matches!(chu.chu_goc, ChuGoc::I | ChuGoc::U | ChuGoc::O)
                && matches!(chu.dau_chu, DauChu::Khong)
            {
                return Some(cac_nguyen_am[cac_nguyen_am.len() - 2]);
            }
        }
        // On-glide `o`+`a`/`e`: HienDai trên `o`, TruyenThong trên `a`/`e`.
        if cac_nguyen_am.len() == 2 {
            let truoc = cac_nguyen_am[0];
            if let (NoiDungDonVi::Chu(chu_truoc), NoiDungDonVi::Chu(chu_sau)) =
                (&don_vi[truoc].noi_dung, &don_vi[cuoi].noi_dung)
            {
                if matches!(chu_truoc.chu_goc, ChuGoc::O)
                    && matches!(chu_sau.chu_goc, ChuGoc::A | ChuGoc::E)
                {
                    return match quy_tac {
                        QuyTacDatDau::HienDai => Some(truoc),
                        QuyTacDatDau::TruyenThong => Some(cuoi),
                    };
                }
            }
        }
    }
    Some(cuoi)
}

/// Tìm vị trí insert trong `don_vi` sao cho đơn vị mới chiếm raw `pos`
/// nằm đúng thứ tự raw.
fn vi_tri_chen(don_vi: &[DonViRender], pos: usize) -> usize {
    don_vi
        .iter()
        .position(|u| u.raw_bat_dau > pos)
        .unwrap_or(don_vi.len())
}

/// Kết quả xử lý đoạn chữ từ Telex.
pub(crate) struct KetQuaTelex {
    /// Danh sách đơn vị render.
    pub(crate) don_vi: Vec<DonViRender>,
    /// Có escape lặp phím xảy ra.
    pub(crate) co_escape: bool,
    /// Escape là escape hình chữ (aa/aw/ee/oo/ow/uw/dd lặp).
    pub(crate) co_escape_hinh_chu: bool,
}

/// Xử lý một đoạn chữ (liên tục các thao tác raw) thành `DonViRender`.
///
/// Pipeline:
/// 1. Biến đổi hình chữ (aa/aw/ee/oo/ow/uw/dd) với escape.
/// 2. Dấu thanh (s/f/r/x/j/z) áp dụng lên nguyên âm cuối, thay dấu, escape.
/// 3. Ký tự nguyên bản luôn literal, chặn Telex nối xuyên.
pub(crate) fn xu_ly_doan_chu(
    cac_thao_tac: &[ThaoTacNhap],
    kieu_telex: KieuTelex,
    quy_tac: QuyTacDatDau,
) -> KetQuaTelex {
    let mut don_vi: Vec<DonViRender> = Vec::new();
    let mut co_escape = false;
    let mut co_escape_hinh_chu = false;
    let n = cac_thao_tac.len();
    let mut i = 0usize;
    // Track phím dấu thanh gần nhất (lowercase) để escape.
    let mut tone_key_cuoi: Option<char> = None;
    // Track vị trí raw của phím dấu thanh gần nhất đã consume.
    let mut tone_pos_cuoi: Option<usize> = None;
    // Track ranh giới đoạn: raw position sau `them_nguyen_ban` gần nhất.
    let mut segment_start: usize = 0;

    while i < n {
        let ky_tu = cac_thao_tac[i].ky_tu;
        let cach_nhap = cac_thao_tac[i].cach_nhap;

        // Ký tự nguyên bản: luôn literal, chặn Telex, reset tone tracking.
        if cach_nhap == CachNhap::NguyenBan {
            don_vi.push(DonViRender::chuong(ky_tu, i));
            tone_key_cuoi = None;
            tone_pos_cuoi = None;
            segment_start = i + 1;
            i += 1;
            continue;
        }

        // --- Thử biến đổi hình chữ ---
        if i + 1 < n && cac_thao_tac[i + 1].cach_nhap == CachNhap::TuDong {
            if let Some((chu_goc, dau_chu)) = cap_hinh_chu(ky_tu, cac_thao_tac[i + 1].ky_tu) {
                let modifier_key = cac_thao_tac[i + 1].ky_tu.to_ascii_lowercase();
                // Escape: lặp đúng modifier đang hoạt động.
                if i + 2 < n
                    && cac_thao_tac[i + 2].cach_nhap == CachNhap::TuDong
                    && cac_thao_tac[i + 2].ky_tu.to_ascii_lowercase() == modifier_key
                {
                    // Escape: hiện raw, bỏ biến đổi, consume escape key.
                    co_escape = true;
                    co_escape_hinh_chu = true;
                    don_vi.push(DonViRender::chuong(ky_tu, i));
                    don_vi.push(DonViRender::chuong(cac_thao_tac[i + 1].ky_tu, i + 1));
                    tone_key_cuoi = None;
                    tone_pos_cuoi = None;
                    i += 3;
                    continue;
                }
                // Áp dụng biến đổi hình chữ.
                // Trường hợp đặc biệt: `uo` + `w` → `ươ` (w biến đổi cả u→ư
                // và o→ơ, vì ươ là tam nguyên âm).
                if (chu_goc, dau_chu) == (ChuGoc::O, DauChu::Moc) {
                    if let Some(idx) = tim_nguyen_am_cuoi(&don_vi, segment_start) {
                        if let NoiDungDonVi::Chu(prev_chu) = &don_vi[idx].noi_dung {
                            if prev_chu.chu_goc == ChuGoc::U
                                && matches!(prev_chu.dau_chu, DauChu::Khong)
                            {
                                let mut u_moi = *prev_chu;
                                u_moi.dau_chu = DauChu::Moc;
                                don_vi[idx].noi_dung = NoiDungDonVi::Chu(u_moi);
                            }
                        }
                    }
                }
                let kieu_hoa = KieuHoa::tu_ky_tu(ky_tu);
                let chu = ChuCaiViet {
                    chu_goc,
                    dau_chu,
                    dau_thanh: DauThanh::Khong,
                    kieu_hoa,
                };
                don_vi.push(DonViRender::chu(chu, i, i + 2));
                tone_key_cuoi = None;
                tone_pos_cuoi = None;
                i += 2;
                continue;
            }
        }

        // --- Thử dấu thanh ---
        if la_phim_dau_thanh(ky_tu) {
            let key_lower = ky_tu.to_ascii_lowercase();
            let dau_thanh_moi = tu_dau_thanh_key(ky_tu).unwrap_or(DauThanh::Khong);

            // Escape: lặp đúng phím dấu thanh đang hoạt động.
            if tone_key_cuoi == Some(key_lower) {
                if let Some(tone_pos) = tone_pos_cuoi {
                    co_escape = true;
                    // Hoàn tác dấu trên nguyên âm chính.
                    if let Some(idx) = tim_nguyen_am_chinh(&don_vi, quy_tac, segment_start) {
                        if let NoiDungDonVi::Chu(ref mut chu) = don_vi[idx].noi_dung {
                            chu.dau_thanh = DauThanh::Khong;
                        }
                        // Rút lại raw_ket_thuc nếu tone key đã mở rộng range.
                        if don_vi[idx].raw_ket_thuc == tone_pos + 1 {
                            don_vi[idx].raw_ket_thuc = tone_pos;
                        } else {
                            // Tone key ở xa — xóa khỏi thao_tac_anh_huong.
                            don_vi[idx].thao_tac_anh_huong.retain(|&p| p != tone_pos);
                        }
                        // Chèn literal cho tone key cũ tại đúng vị trí.
                        let vi_tri = vi_tri_chen(&don_vi, tone_pos);
                        let ky_tu_tone = cac_thao_tac[tone_pos].ky_tu;
                        don_vi.insert(vi_tri, DonViRender::chuong(ky_tu_tone, tone_pos));
                    }
                    // Escape trigger (position i) consumed — không hiện literal.
                    tone_key_cuoi = None;
                    tone_pos_cuoi = None;
                    i += 1;
                    continue;
                }
            }

            // Áp dụng / thay / xóa dấu thanh.
            if let Some(idx) = tim_nguyen_am_chinh(&don_vi, quy_tac, segment_start) {
                // z (xóa dấu): chỉ consume khi có dấu để xóa.
                let dau_hien_tai = match &don_vi[idx].noi_dung {
                    NoiDungDonVi::Chu(chu) => chu.dau_thanh,
                    NoiDungDonVi::Chuong(_) => DauThanh::Khong,
                };
                if key_lower == 'z' && dau_hien_tai == DauThanh::Khong {
                    // Không có dấu để xóa — z là literal.
                    don_vi.push(DonViRender::chuong(ky_tu, i));
                    tone_key_cuoi = None;
                    tone_pos_cuoi = None;
                    i += 1;
                    continue;
                }
                // Áp dụng dấu mới.
                if let NoiDungDonVi::Chu(ref mut chu) = don_vi[idx].noi_dung {
                    chu.dau_thanh = dau_thanh_moi;
                }
                // Mở rộng range nếu tone key nằm ngay sau unit.
                let unit_end = don_vi[idx].raw_ket_thuc;
                if i == unit_end {
                    don_vi[idx].raw_ket_thuc = i + 1;
                } else {
                    // Tone key ở xa (sau other units) — chỉ track provenance.
                    don_vi[idx].thao_tac_anh_huong.push(i);
                }
                // z (xóa dấu) không track escape.
                if key_lower == 'z' {
                    tone_key_cuoi = None;
                    tone_pos_cuoi = None;
                } else {
                    tone_key_cuoi = Some(key_lower);
                    tone_pos_cuoi = Some(i);
                }
                // Tone key consumed — không tạo đơn vị mới.
                i += 1;
                continue;
            }
            // Không có nguyên âm để đặt dấu — literal.
            don_vi.push(DonViRender::chuong(ky_tu, i));
            tone_key_cuoi = None;
            tone_pos_cuoi = None;
            i += 1;
            continue;
        }

        // --- DayDu: phím nhanh `w` đơn lẻ, `[`, `]` ---
        if kieu_telex == KieuTelex::DayDu {
            let key_lower = ky_tu.to_ascii_lowercase();
            if key_lower == 'w' || key_lower == '[' || key_lower == ']' {
                let kieu_hoa = KieuHoa::tu_ky_tu(ky_tu);
                let (chu_goc, dau_chu) = if key_lower == ']' {
                    (ChuGoc::O, DauChu::Moc)
                } else {
                    (ChuGoc::U, DauChu::Moc)
                };
                let chu = ChuCaiViet {
                    chu_goc,
                    dau_chu,
                    dau_thanh: DauThanh::Khong,
                    kieu_hoa,
                };
                don_vi.push(DonViRender::chu(chu, i, i + 1));
                tone_key_cuoi = None;
                tone_pos_cuoi = None;
                i += 1;
                continue;
            }
        }

        // --- Ký tự thường (không phải Telex modifier) ---
        let chu = match render::phan_tich_ky_tu(ky_tu) {
            Some(c) => c,
            None => ChuCaiViet::thuong(ky_tu),
        };
        if chu.chu_goc.la_nguyen_am() || matches!(chu.chu_goc, ChuGoc::D) {
            don_vi.push(DonViRender::chu(chu, i, i + 1));
        } else {
            don_vi.push(DonViRender::chuong(ky_tu, i));
        }
        tone_key_cuoi = None;
        tone_pos_cuoi = None;
        i += 1;
    }
    KetQuaTelex {
        don_vi,
        co_escape,
        co_escape_hinh_chu,
    }
}
