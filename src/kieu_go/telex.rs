// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Bộ nhận Telex - biến đổi raw input thành đơn vị render có provenance.
//!
//! Module này là tầng giữa: nhận các thao tác raw trong một đoạn chữ, áp
//! dụng rule Telex (hình chữ, dấu thanh, escape) và xuất ra `DonViRender`
//! mang theo provenance (thao tác raw nào sinh ra đơn vị này).
//!
//! Thứ tự linh hoạt (parity VNI, RFC 0021): phím hình chữ (`w`/`a`/`e`/`o`/`d`)
//! reach back tới base trần gần nhất trong đoạn, không nhất thiết ngay sau base.
//! Do đó `oiw`→`ơi`, `voiws`→`với`, `khongo`→`không` đều biến đổi. Xem RFC 0006.

use alloc::vec::Vec;

use super::bo_dat_dau::{tim_nguyen_am_chinh, vi_tri_chen};
use super::chu_viet::{ChuCaiViet, ChuGoc, DauChu, DauThanh, KieuHoa};
use super::render;
use crate::cau_hinh::{KieuTelex, QuyTacDatDau};
use crate::thao_tac::{CachNhap, ThaoTacNhap};

pub(crate) use super::don_vi::{DonViRender, KetQuaDoanChu, NoiDungDonVi};

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

/// Trả `true` nếu `c` là phím hình chữ ứng viên (`w`/`a`/`e`/`o`/`d`).
///
/// Các phím này vừa là chữ cái nền vừa là modifier hình chữ. Khi không tìm
/// được base trần tương thích trong đoạn, chúng rơi xuống đường "ký tự
/// thường" và được push như chữ cái thường.
fn la_phim_hinh_chu(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'w' | 'a' | 'e' | 'o' | 'd')
}

/// Trạng thái trước khi áp dụng một phím hình chữ, để hoàn tác khi escape.
#[derive(Clone, Copy)]
struct TrangThaiTruoc {
    /// Dấu chữ trước modifier.
    dau_chu: DauChu,
    /// Dấu thanh trước modifier.
    dau_thanh: DauThanh,
}

/// Tìm ngược base trần (chưa có dấu hình chữ) gần nhất tương thích với phím
/// hình chữ `modifier`, từ `segment_start` trở đi. Trả `(idx, dau_chu_moi)`.
///
/// Khác VNI (`tim_nguyen_am_tuong_thich`), hàm này yêu cầu `dau_chu == Khong`
/// để tránh restroke chữ đã có dấu hình chữ (vd `â` + `w` không thành `ă`).
/// Điều này giữ escape Telex (`aaw`→`âw`, không phải `ă`).
fn tim_base_hinh_chu(
    don_vi: &[DonViRender],
    modifier: char,
    segment_start: usize,
) -> Option<(usize, DauChu)> {
    let m = modifier.to_ascii_lowercase();
    don_vi
        .iter()
        .enumerate()
        .filter(|(_, u)| u.raw_bat_dau >= segment_start)
        .rev()
        .find_map(|(idx, u)| match &u.noi_dung {
            NoiDungDonVi::Chu(chu) if matches!(chu.dau_chu, DauChu::Khong) => {
                cap_hinh_chu(chu.chu_goc.ky_tu_thuong(), m).map(|(_, dau)| (idx, dau))
            }
            _ => None,
        })
}

/// Xử lý một đoạn chữ (liên tục các thao tác raw) thành `DonViRender`.
///
/// Pipeline:
/// 1. Ký tự nguyên bản luôn literal, chặn Telex nối xuyên, reset tracking.
/// 2. Dấu thanh (s/f/r/x/j/z) áp dụng lên nguyên âm chính, thay dấu, escape.
/// 3. Hình chữ (w/a/e/o/d) reach back tới base trần trong đoạn, thay dấu
///    chữ, escape. ươ đặc biệt: `w` horn cả `u` và `o` khi cặp liền nhau.
/// 4. DayDu: phím nhanh `w` đơn lẻ, `[`, `]`.
/// 5. Ký tự thường push như chữ cái (vowel/D) hoặc literal (phụ âm khác).
pub(crate) fn xu_ly_doan_chu(
    cac_thao_tac: &[ThaoTacNhap],
    kieu_telex: KieuTelex,
    quy_tac: QuyTacDatDau,
) -> KetQuaDoanChu {
    let mut don_vi: Vec<DonViRender> = Vec::new();
    let mut co_escape = false;
    let mut co_escape_hinh_chu = false;
    // Đã áp dụng shape "ở xa" (modifier reach back qua ký tự khác) ít nhất
    // một lần. `lua_chon` dùng để chặn reshape tiếng Anh/kỹ thuật.
    let mut co_hinh_xa = false;
    let n = cac_thao_tac.len();
    let mut i = 0usize;
    // Track phím dấu thanh gần nhất (lowercase) để escape.
    let mut tone_key_cuoi: Option<char> = None;
    // Track vị trí raw của phím dấu thanh gần nhất đã consume.
    let mut tone_pos_cuoi: Option<usize> = None;
    // Track phím hình chữ gần nhất (lowercase) để escape.
    let mut hinh_mod: Option<char> = None;
    // Vị trí raw của phím hình chữ gần nhất.
    let mut hinh_mod_pos: Option<usize> = None;
    // Các đơn vị bị phím hình chữ gần nhất tác động + trạng thái trước, để
    // hoàn tác khi escape (ươ có thể tác động 2 đơn vị: `u` và `o`).
    let mut hinh_mod_targets: Vec<usize> = Vec::new();
    let mut hinh_mod_prev: Vec<TrangThaiTruoc> = Vec::new();
    // Ranh giới đoạn: raw position sau `them_nguyen_ban` gần nhất.
    let mut segment_start: usize = 0;

    // Xóa tracking hình chữ (sau khi shape áp dụng/escape, hoặc khi ký tự
    // khác ngắt chuỗi escape hình chữ).
    macro_rules! xoa_hinh {
        () => {{
            hinh_mod = None;
            hinh_mod_pos = None;
            hinh_mod_targets.clear();
            hinh_mod_prev.clear();
        }};
    }
    // Xóa tracking dấu thanh (khi shape áp dụng/escape, hoặc ký tự khác ngắt).
    macro_rules! xoa_tone {
        () => {{
            tone_key_cuoi = None;
            tone_pos_cuoi = None;
        }};
    }

    while i < n {
        let ky_tu = cac_thao_tac[i].ky_tu;
        let cach_nhap = cac_thao_tac[i].cach_nhap;

        // Ký tự nguyên bản: luôn literal, chặn Telex, reset tracking.
        if cach_nhap == CachNhap::NguyenBan {
            don_vi.push(DonViRender::chuong(ky_tu, i));
            xoa_tone!();
            xoa_hinh!();
            segment_start = i + 1;
            i += 1;
            continue;
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
                            // Tone key ở xa - xóa khỏi thao_tac_anh_huong.
                            don_vi[idx].thao_tac_anh_huong.retain(|&p| p != tone_pos);
                        }
                        // Chèn literal cho tone key cũ tại đúng vị trí.
                        let vi_tri = vi_tri_chen(&don_vi, tone_pos);
                        let ky_tu_tone = cac_thao_tac[tone_pos].ky_tu;
                        don_vi.insert(vi_tri, DonViRender::chuong(ky_tu_tone, tone_pos));
                    }
                    // Escape trigger (position i) consumed - không hiện literal.
                    xoa_tone!();
                    xoa_hinh!();
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
                    // Không có dấu để xóa - z là literal.
                    don_vi.push(DonViRender::chuong(ky_tu, i));
                    xoa_tone!();
                    xoa_hinh!();
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
                    // Tone key ở xa (sau other units) - chỉ track provenance.
                    don_vi[idx].thao_tac_anh_huong.push(i);
                }
                // z (xóa dấu) không track escape.
                if key_lower == 'z' {
                    xoa_tone!();
                } else {
                    tone_key_cuoi = Some(key_lower);
                    tone_pos_cuoi = Some(i);
                }
                // Phím dấu thanh ngắt chuỗi escape hình chữ.
                xoa_hinh!();
                // Tone key consumed - không tạo đơn vị mới.
                i += 1;
                continue;
            }
            // Không có nguyên âm để đặt dấu - literal.
            don_vi.push(DonViRender::chuong(ky_tu, i));
            xoa_tone!();
            xoa_hinh!();
            i += 1;
            continue;
        }

        // --- Thử biến đổi hình chữ (backward search, thứ tự linh hoạt) ---
        if la_phim_hinh_chu(ky_tu) {
            let key_lower = ky_tu.to_ascii_lowercase();

            // Escape: lặp đúng phím hình chữ đang hoạt động.
            if hinh_mod == Some(key_lower) {
                if let Some(pos) = hinh_mod_pos {
                    co_escape = true;
                    co_escape_hinh_chu = true;
                    // Hoàn tác shape trên từng target (ươ có thể 2 target).
                    for (&idx, prev) in hinh_mod_targets.iter().zip(hinh_mod_prev.iter()).rev() {
                        if let NoiDungDonVi::Chu(ref mut chu) = don_vi[idx].noi_dung {
                            chu.dau_chu = prev.dau_chu;
                            chu.dau_thanh = prev.dau_thanh;
                        }
                        // Rút lại raw_ket_thuc nếu modifier đã mở rộng range.
                        if don_vi[idx].raw_ket_thuc == pos + 1 {
                            don_vi[idx].raw_ket_thuc = pos;
                        } else {
                            don_vi[idx].thao_tac_anh_huong.retain(|&p| p != pos);
                        }
                    }
                    // Chèn literal cho modifier cũ tại đúng vị trí.
                    let vi_tri = vi_tri_chen(&don_vi, pos);
                    don_vi.insert(vi_tri, DonViRender::chuong(cac_thao_tac[pos].ky_tu, pos));
                    xoa_tone!();
                    xoa_hinh!();
                    i += 1;
                    continue;
                }
            }

            // Tìm base trần ngược trong đoạn.
            if let Some((idx, dau_chu_moi)) = tim_base_hinh_chu(&don_vi, key_lower, segment_start) {
                hinh_mod_targets.clear();
                hinh_mod_prev.clear();

                // ươ đặc biệt: Moc trên O hoặc U, nếu cặp u+o liền nhau đều
                // trần → horn cả hai (giống `uo`+`w` / VNI digit 7).
                if dau_chu_moi == DauChu::Moc {
                    let goc = match &don_vi[idx].noi_dung {
                        NoiDungDonVi::Chu(chu) => chu.chu_goc,
                        NoiDungDonVi::Chuong(_) => ChuGoc::A, // không xảy ra
                    };
                    // base=O, partner U ngay trước → horn U.
                    if goc == ChuGoc::O && idx > 0 {
                        if let NoiDungDonVi::Chu(partner) = &don_vi[idx - 1].noi_dung {
                            if partner.chu_goc == ChuGoc::U
                                && matches!(partner.dau_chu, DauChu::Khong)
                            {
                                let prev_u = TrangThaiTruoc {
                                    dau_chu: partner.dau_chu,
                                    dau_thanh: partner.dau_thanh,
                                };
                                if let NoiDungDonVi::Chu(p) = &mut don_vi[idx - 1].noi_dung {
                                    p.dau_chu = DauChu::Moc;
                                }
                                hinh_mod_prev.push(prev_u);
                                hinh_mod_targets.push(idx - 1);
                            }
                        }
                    }
                    // base=U, partner O ngay sau → horn O.
                    if goc == ChuGoc::U {
                        if let Some(sau) = don_vi.get(idx + 1) {
                            if let NoiDungDonVi::Chu(partner) = &sau.noi_dung {
                                if partner.chu_goc == ChuGoc::O
                                    && matches!(partner.dau_chu, DauChu::Khong)
                                {
                                    let prev_o = TrangThaiTruoc {
                                        dau_chu: partner.dau_chu,
                                        dau_thanh: partner.dau_thanh,
                                    };
                                    if let NoiDungDonVi::Chu(p) = &mut don_vi[idx + 1].noi_dung {
                                        p.dau_chu = DauChu::Moc;
                                    }
                                    hinh_mod_prev.push(prev_o);
                                    hinh_mod_targets.push(idx + 1);
                                }
                            }
                        }
                    }
                }

                // Biến đổi base chính (sau khi đã xử lý partner ươ nếu có).
                if let NoiDungDonVi::Chu(ref mut chu) = don_vi[idx].noi_dung {
                    hinh_mod_prev.insert(
                        0,
                        TrangThaiTruoc {
                            dau_chu: chu.dau_chu,
                            dau_thanh: chu.dau_thanh,
                        },
                    );
                    chu.dau_chu = dau_chu_moi;
                }
                hinh_mod_targets.insert(0, idx);

                // Mở rộng range nếu modifier nằm ngay sau unit, còn không thì
                // track provenance.
                let unit_end = don_vi[idx].raw_ket_thuc;
                if i == unit_end {
                    don_vi[idx].raw_ket_thuc = i + 1;
                } else {
                    // Modifier reach back qua ký tự khác → shape "ở xa".
                    don_vi[idx].thao_tac_anh_huong.push(i);
                    co_hinh_xa = true;
                }

                hinh_mod = Some(key_lower);
                hinh_mod_pos = Some(i);
                // Phím hình chữ ngắt chuỗi escape dấu thanh.
                xoa_tone!();
                i += 1;
                continue;
            }
            // Không tìm base → fall through (DayDu single-w hoặc ký tự thường).
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
                xoa_tone!();
                xoa_hinh!();
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
        xoa_tone!();
        xoa_hinh!();
        i += 1;
    }
    KetQuaDoanChu {
        don_vi,
        co_escape,
        co_escape_hinh_chu,
        co_hinh_xa,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cau_hinh::KieuTelex;
    use crate::kieu_go::chu_viet::{ChuGoc, DauChu};
    use alloc::{string::String, vec};

    /// Bảng cap_hinh_chu: mỗi cặp (base, modifier) hợp lệ trả đúng (ChuGoc, DauChu).
    #[test]
    fn cap_hinh_chu_dung_cho_moi_cap() {
        assert_eq!(cap_hinh_chu('a', 'a'), Some((ChuGoc::A, DauChu::Mu)));
        assert_eq!(cap_hinh_chu('a', 'w'), Some((ChuGoc::A, DauChu::Trang)));
        assert_eq!(cap_hinh_chu('e', 'e'), Some((ChuGoc::E, DauChu::Mu)));
        assert_eq!(cap_hinh_chu('o', 'o'), Some((ChuGoc::O, DauChu::Mu)));
        assert_eq!(cap_hinh_chu('o', 'w'), Some((ChuGoc::O, DauChu::Moc)));
        assert_eq!(cap_hinh_chu('u', 'w'), Some((ChuGoc::U, DauChu::Moc)));
        assert_eq!(cap_hinh_chu('d', 'd'), Some((ChuGoc::D, DauChu::Gach)));
    }

    /// cap_hinh_chu không nhận base sai (b, c, f, ...).
    #[test]
    fn cap_hinh_chu_base_sai_tra_none() {
        assert_eq!(cap_hinh_chu('b', 'w'), None);
        assert_eq!(cap_hinh_chu('c', 'w'), None);
        assert_eq!(cap_hinh_chu('f', 'a'), None);
        assert_eq!(cap_hinh_chu('i', 'w'), None);
        assert_eq!(cap_hinh_chu('y', 'w'), None);
    }

    /// cap_hinh_chu case-insensitive (hoa/thường cho cùng kết quả).
    #[test]
    fn cap_hinh_chu_case_insensitive() {
        assert_eq!(cap_hinh_chu('A', 'A'), Some((ChuGoc::A, DauChu::Mu)));
        assert_eq!(cap_hinh_chu('A', 'W'), Some((ChuGoc::A, DauChu::Trang)));
        assert_eq!(cap_hinh_chu('D', 'D'), Some((ChuGoc::D, DauChu::Gach)));
    }

    /// tu_dau_thanh_key: mỗi phím tone → đúng DauThanh.
    #[test]
    fn tu_dau_thanh_key_dung() {
        assert_eq!(tu_dau_thanh_key('s'), Some(DauThanh::Sac));
        assert_eq!(tu_dau_thanh_key('f'), Some(DauThanh::Huyen));
        assert_eq!(tu_dau_thanh_key('r'), Some(DauThanh::Hoi));
        assert_eq!(tu_dau_thanh_key('x'), Some(DauThanh::Nga));
        assert_eq!(tu_dau_thanh_key('j'), Some(DauThanh::Nang));
        assert_eq!(tu_dau_thanh_key('z'), Some(DauThanh::Khong));
    }

    /// tu_dau_thanh_key không nhận phím không phải tone.
    #[test]
    fn tu_dau_thanh_key_sai_tra_none() {
        assert_eq!(tu_dau_thanh_key('a'), None);
        assert_eq!(tu_dau_thanh_key('w'), None);
        assert_eq!(tu_dau_thanh_key('d'), None);
    }

    /// la_phim_dau_thanh nhất quán với tu_dau_thanh_key (trừ `z` xóa dấu).
    #[test]
    fn la_phim_dau_thanh_nhat_quan() {
        for c in ['s', 'f', 'r', 'x', 'j', 'z'] {
            assert!(la_phim_dau_thanh(c), "{c} phai la phim tone");
            assert!(
                la_phim_dau_thanh(c.to_ascii_uppercase()),
                "{} phai la phim tone",
                c
            );
        }
        for c in ['a', 'e', 'i', 'o', 'u', 'y', 'w', 'd', 'b'] {
            assert!(!la_phim_dau_thanh(c), "{c} khong phai phim tone");
        }
    }

    /// la_phim_hinh_chu nhận w/a/e/o/d, không nhận tone hay phụ âm khác.
    #[test]
    fn la_phim_hinh_chu_nhat_quan() {
        for c in ['w', 'a', 'e', 'o', 'd'] {
            assert!(la_phim_hinh_chu(c), "{c} phai la phim hinh chu");
            assert!(la_phim_hinh_chu(c.to_ascii_uppercase()));
        }
        for c in ['s', 'f', 'r', 'x', 'j', 'z', 'b', 'i', 'u', 'y'] {
            assert!(!la_phim_hinh_chu(c), "{c} khong phai phim hinh chu");
        }
    }

    /// xu_ly_doan_chu: DayDu `w` đơn lẻ → `ư` (ChuGoc::U, DauChu::Moc).
    #[test]
    fn day_du_w_don_le_thanh_u_horn() {
        let tts = [ThaoTacNhap::tu_dong('w')];
        let kq = xu_ly_doan_chu(&tts, KieuTelex::DayDu, QuyTacDatDau::HienDai);
        assert_eq!(kq.don_vi.len(), 1);
        match &kq.don_vi[0].noi_dung {
            NoiDungDonVi::Chu(chu) => {
                assert_eq!(chu.chu_goc, ChuGoc::U);
                assert_eq!(chu.dau_chu, DauChu::Moc);
            }
            NoiDungDonVi::Chuong(_) => panic!("w don le phai bien doi"),
        }
    }

    /// xu_ly_doan_chu: CanBang `w` đơn lẻ → literal `w`.
    #[test]
    fn can_bang_w_don_le_literal() {
        let tts = [ThaoTacNhap::tu_dong('w')];
        let kq = xu_ly_doan_chu(&tts, KieuTelex::CanBang, QuyTacDatDau::HienDai);
        assert_eq!(kq.don_vi.len(), 1);
        assert!(matches!(kq.don_vi[0].noi_dung, NoiDungDonVi::Chuong('w')));
    }

    /// xu_ly_doan_chu: ký tự nguyên bản luôn literal, không biến đổi Telex.
    ///
    /// DayDu: `w` tự động → `ư` (shape Chu). `w` nguyên bản → literal `w`
    /// (Chuong), không biến đổi.
    #[test]
    fn ky_tu_nguyen_ban_luon_literal() {
        // `w` tự động trong DayDu → shape `ư` (Chu).
        let tts_auto = [ThaoTacNhap::tu_dong('w')];
        let kq_auto = xu_ly_doan_chu(&tts_auto, KieuTelex::DayDu, QuyTacDatDau::HienDai);
        assert_eq!(kq_auto.don_vi.len(), 1);
        assert!(matches!(kq_auto.don_vi[0].noi_dung, NoiDungDonVi::Chu(_)));

        // `w` nguyên bản trong DayDu → literal `w` (Chuong), không shape.
        let tts_raw = [ThaoTacNhap::nguyen_ban('w')];
        let kq_raw = xu_ly_doan_chu(&tts_raw, KieuTelex::DayDu, QuyTacDatDau::HienDai);
        assert_eq!(kq_raw.don_vi.len(), 1);
        assert!(matches!(
            kq_raw.don_vi[0].noi_dung,
            NoiDungDonVi::Chuong('w')
        ));
    }

    // --- Thứ tự linh hoạt: shape modifier reach back trong đoạn ---

    /// `ow` liền → ơ (baseline adjacency, vẫn hoạt động).
    #[test]
    fn ow_lien_thanh_o_moc() {
        let tts: Vec<ThaoTacNhap> = "ow".chars().map(ThaoTacNhap::tu_dong).collect();
        let kq = xu_ly_doan_chu(&tts, KieuTelex::CanBang, QuyTacDatDau::HienDai);
        assert_eq!(kq.don_vi.len(), 1);
        match &kq.don_vi[0].noi_dung {
            NoiDungDonVi::Chu(chu) => {
                assert_eq!(chu.chu_goc, ChuGoc::O);
                assert_eq!(chu.dau_chu, DauChu::Moc);
            }
            NoiDungDonVi::Chuong(_) => panic!("ow phai biendoi"),
        }
    }

    /// `oiw` (w cách base `o` qua bán âm `i`) → ơ + i = ơi.
    #[test]
    fn oiw_w_cach_base_qua_ban_am_thanh_oi() {
        let tts: Vec<ThaoTacNhap> = "oiw".chars().map(ThaoTacNhap::tu_dong).collect();
        let kq = xu_ly_doan_chu(&tts, KieuTelex::CanBang, QuyTacDatDau::HienDai);
        let nguyen_am: Vec<DauChu> = kq
            .don_vi
            .iter()
            .filter_map(|u| match &u.noi_dung {
                NoiDungDonVi::Chu(c) if c.chu_goc.la_nguyen_am() => Some(c.dau_chu),
                _ => None,
            })
            .collect();
        // ơ (Moc) + i (Khong).
        assert_eq!(nguyen_am, vec![DauChu::Moc, DauChu::Khong]);
        assert!(
            kq.don_vi
                .iter()
                .all(|u| !matches!(u.noi_dung, NoiDungDonVi::Chuong('w')))
        );
    }

    /// `uoiw` (w cách cặp `uo` qua bán âm `i`) → ươ + i = ươi.
    #[test]
    fn uoiw_w_cach_qua_ban_am_thanh_uouoi() {
        let tts: Vec<ThaoTacNhap> = "uoiw".chars().map(ThaoTacNhap::tu_dong).collect();
        let kq = xu_ly_doan_chu(&tts, KieuTelex::CanBang, QuyTacDatDau::HienDai);
        let dau: Vec<DauChu> = kq
            .don_vi
            .iter()
            .filter_map(|u| match &u.noi_dung {
                NoiDungDonVi::Chu(c) if c.chu_goc.la_nguyen_am() => Some(c.dau_chu),
                _ => None,
            })
            .collect();
        assert_eq!(dau, vec![DauChu::Moc, DauChu::Moc, DauChu::Khong]);
    }

    /// `khongo` (oo restroke `o` đầu qua phụ âm `ng`) → ô + ng.
    #[test]
    fn khongo_oo_restroke_qua_phu_am_thanh_ong() {
        let tts: Vec<ThaoTacNhap> = "khongo".chars().map(ThaoTacNhap::tu_dong).collect();
        let kq = xu_ly_doan_chu(&tts, KieuTelex::CanBang, QuyTacDatDau::HienDai);
        let co_mu = kq
            .don_vi
            .iter()
            .any(|u| matches!(&u.noi_dung, NoiDungDonVi::Chu(c) if c.dau_chu == DauChu::Mu));
        assert!(co_mu, "khongo phai co mũ trên o đầu");
        // Chỉ một ô (o đầu), o cuối là modifier consumed.
        let so_mu = kq
            .don_vi
            .iter()
            .filter(|u| matches!(&u.noi_dung, NoiDungDonVi::Chu(c) if c.dau_chu == DauChu::Mu))
            .count();
        assert_eq!(so_mu, 1);
    }

    /// `uongw` (w cách cặp `uo` qua phụ âm `ng`) → ươ + ng = ương.
    #[test]
    fn uongw_w_cach_qua_phu_am_thanh_uouong() {
        let tts: Vec<ThaoTacNhap> = "uongw".chars().map(ThaoTacNhap::tu_dong).collect();
        let kq = xu_ly_doan_chu(&tts, KieuTelex::CanBang, QuyTacDatDau::HienDai);
        let dau: Vec<DauChu> = kq
            .don_vi
            .iter()
            .filter_map(|u| match &u.noi_dung {
                NoiDungDonVi::Chu(c) if c.chu_goc.la_nguyen_am() => Some(c.dau_chu),
                _ => None,
            })
            .collect();
        assert_eq!(dau, vec![DauChu::Moc, DauChu::Moc]);
    }

    /// Escape backward: `oww` → o + w (hoàn tác ơ).
    #[test]
    fn oww_escape_thanh_ow() {
        let tts: Vec<ThaoTacNhap> = "oww".chars().map(ThaoTacNhap::tu_dong).collect();
        let kq = xu_ly_doan_chu(&tts, KieuTelex::CanBang, QuyTacDatDau::HienDai);
        assert!(kq.co_escape);
        assert!(kq.co_escape_hinh_chu);
        // o (Khong) + w literal.
        match &kq.don_vi[0].noi_dung {
            NoiDungDonVi::Chu(chu) => {
                assert_eq!(chu.chu_goc, ChuGoc::O);
                assert_eq!(chu.dau_chu, DauChu::Khong);
            }
            NoiDungDonVi::Chuong(_) => panic!("o phai la Chu"),
        }
        assert!(matches!(kq.don_vi[1].noi_dung, NoiDungDonVi::Chuong('w')));
    }

    /// Escape backward `oiww` → oiw (hoàn tác ơ, giữ bán âm i).
    #[test]
    fn oiww_escape_thanh_oiw() {
        let tts: Vec<ThaoTacNhap> = "oiww".chars().map(ThaoTacNhap::tu_dong).collect();
        let kq = xu_ly_doan_chu(&tts, KieuTelex::CanBang, QuyTacDatDau::HienDai);
        assert!(kq.co_escape);
        let chuoi: String = kq
            .don_vi
            .iter()
            .map(|u| match &u.noi_dung {
                NoiDungDonVi::Chu(c) => c.chu_goc.ky_tu_thuong(),
                NoiDungDonVi::Chuong(c) => *c,
            })
            .collect();
        assert_eq!(chuoi, "oiw");
    }

    /// Case preservation: `OW` → Ơ (hoa theo base).
    #[test]
    fn ow_hoa_thanh_o_moc_hoa() {
        let tts: Vec<ThaoTacNhap> = "OW".chars().map(ThaoTacNhap::tu_dong).collect();
        let kq = xu_ly_doan_chu(&tts, KieuTelex::CanBang, QuyTacDatDau::HienDai);
        match &kq.don_vi[0].noi_dung {
            NoiDungDonVi::Chu(chu) => {
                assert_eq!(chu.chu_goc, ChuGoc::O);
                assert_eq!(chu.dau_chu, DauChu::Moc);
                assert_eq!(chu.kieu_hoa, crate::kieu_go::chu_viet::KieuHoa::Hoa);
            }
            NoiDungDonVi::Chuong(_) => panic!("OW phai bien doi"),
        }
    }

    /// `aaw` → â + w (â có Mu, w không restroke được → w literal). Không thành ă.
    #[test]
    fn aaw_khong_restroke_da_co_dau() {
        let tts: Vec<ThaoTacNhap> = "aaw".chars().map(ThaoTacNhap::tu_dong).collect();
        let kq = xu_ly_doan_chu(&tts, KieuTelex::CanBang, QuyTacDatDau::HienDai);
        match &kq.don_vi[0].noi_dung {
            NoiDungDonVi::Chu(chu) => {
                assert_eq!(chu.dau_chu, DauChu::Mu, "aaw -> â (Mu), khong phai ă");
            }
            NoiDungDonVi::Chuong(_) => panic!("aaw phai co â"),
        }
        assert!(matches!(kq.don_vi[1].noi_dung, NoiDungDonVi::Chuong('w')));
    }
}
