// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Bộ nhận VNI - biến đổi raw input thành đơn vị render có provenance.
//!
//! VNI dùng chữ số làm modifier: `1..=5` dấu thanh, `6` mũ, `7` móc, `8` trăng,
//! `9` đ. Engine này dùng chung `BoDatDau` (nguyên âm chính) và `DonViRender`
//! với Telex; chỉ khác cách diễn giải raw action. Xem RFC 0021.
//!
//! Quy tắc (đã khóa bằng test, xem `tests/vni_*.rs`):
//! * Dấu thanh `1..=5`: áp dụng lên nguyên âm chính (dùng chung
//!   `tim_nguyen_am_chinh`), thay dấu cũ. Không có nguyên âm → literal.
//! * Hình chữ `6/7/8`: áp dụng lên nguyên âm chính nếu tương thích
//!   (`6`→`a/e/o`, `7`→`o/u`, `8`→`a`), thay dấu chữ cũ. Không tương thích →
//!   literal.
//! * `9` (gach): áp dụng lên `d` gần nhất trong âm tiết (chưa có dấu gạch).
//!   Không có → literal.
//! * ươ đặc biệt: `7` lên nguyên âm chính `u` khi đơn vị ngay sau là `o` không
//!   dấu → biến đổi cả `u`→`ư` và `o`→`ơ` (giống `uo`+`w` Telex).
//! * Thay dấu: digit mới thay digit cũ (tone và shape đều thay).
//! * Thứ tự đảo: shape và tone độc lập nên `a16`/`a61` cùng cho `ấ`.
//! * Escape: lặp đúng modifier digit đang hoạt động → hoàn tác modifier đó,
//!   hiện digit đầu thành literal, consume digit thứ hai (giống escape Telex).

use alloc::vec::Vec;

use super::bo_dat_dau::{tim_nguyen_am_chinh, vi_tri_chen};
use super::chu_viet::{ChuCaiViet, ChuGoc, DauChu, DauThanh};
use super::don_vi::{DonViRender, KetQuaDoanChu, NoiDungDonVi};
use super::render;
use crate::cau_hinh::QuyTacDatDau;
use crate::thao_tac::{CachNhap, ThaoTacNhap};

/// Tra dấu thanh từ digit VNI `1..=5`.
fn tu_dau_thanh_digit(d: char) -> Option<DauThanh> {
    match d {
        '1' => Some(DauThanh::Sac),
        '2' => Some(DauThanh::Huyen),
        '3' => Some(DauThanh::Hoi),
        '4' => Some(DauThanh::Nga),
        '5' => Some(DauThanh::Nang),
        _ => None,
    }
}

/// Trả `true` nếu `c` là digit modifier VNI (`1..=9`).
fn la_digit_vni(c: char) -> bool {
    matches!(c, '1'..='9')
}

/// Trả `true` nếu `dau_chu` tương thích với `goc` cho digit `6` (mũ).
fn tuong_thich_mu(goc: ChuGoc) -> bool {
    matches!(goc, ChuGoc::A | ChuGoc::E | ChuGoc::O)
}

/// Trả `true` nếu `dau_chu` tương thích với `digit 7` (móc).
fn tuong_thich_moc(goc: ChuGoc) -> bool {
    matches!(goc, ChuGoc::O | ChuGoc::U)
}

/// Trả `true` nếu `dau_chu` tương thích với `digit 8` (trăng).
fn tuong_thich_trang(goc: ChuGoc) -> bool {
    matches!(goc, ChuGoc::A)
}

/// Trạng thái trước khi áp dụng một modifier, để hoàn tác khi escape.
#[derive(Clone, Copy)]
struct TrangThaiTruoc {
    /// Dấu chữ trước modifier.
    dau_chu: DauChu,
    /// Dấu thanh trước modifier.
    dau_thanh: DauThanh,
}

/// Xử lý một đoạn chữ (liên tục các thao tác raw) qua VNI.
pub(crate) fn xu_ly_doan_chu(cac_thao_tac: &[ThaoTacNhap], quy_tac: QuyTacDatDau) -> KetQuaDoanChu {
    let mut don_vi: Vec<DonViRender> = Vec::new();
    let mut co_escape = false;
    let mut co_escape_hinh_chu = false;
    let n = cac_thao_tac.len();
    let mut i = 0usize;
    // Digit modifier gần nhất đã áp dụng (để escape).
    let mut last_mod: Option<char> = None;
    // Vị trí raw của modifier gần nhất.
    let mut last_mod_pos: Option<usize> = None;
    // Các đơn vị bị modifier gần nhất tác động + trạng thái trước, để hoàn tác.
    let mut last_mod_targets: Vec<usize> = Vec::new();
    let mut last_mod_prev: Vec<TrangThaiTruoc> = Vec::new();
    // Ranh giới đoạn: raw position sau `them_nguyen_ban` gần nhất.
    let mut segment_start: usize = 0;

    while i < n {
        let ky_tu = cac_thao_tac[i].ky_tu;
        let cach_nhap = cac_thao_tac[i].cach_nhap;

        // Ký tự nguyên bản: luôn literal, chặn VNI, reset tracking.
        if cach_nhap == CachNhap::NguyenBan {
            don_vi.push(DonViRender::chuong(ky_tu, i));
            last_mod = None;
            last_mod_pos = None;
            last_mod_targets.clear();
            last_mod_prev.clear();
            segment_start = i + 1;
            i += 1;
            continue;
        }

        // --- Digit modifier VNI ---
        if la_digit_vni(ky_tu) {
            // Escape: lặp đúng modifier digit đang hoạt động.
            if last_mod == Some(ky_tu) {
                if let Some(pos) = last_mod_pos {
                    co_escape = true;
                    co_escape_hinh_chu = !matches!(ky_tu, '1'..='5');
                    // Hoàn tác modifier trên từng target.
                    for (&idx, prev) in last_mod_targets.iter().zip(last_mod_prev.iter()).rev() {
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
                    // Chèn literal cho digit modifier đầu tại đúng vị trí.
                    let vi_tri = vi_tri_chen(&don_vi, pos);
                    don_vi.insert(vi_tri, DonViRender::chuong(cac_thao_tac[pos].ky_tu, pos));
                    last_mod = None;
                    last_mod_pos = None;
                    last_mod_targets.clear();
                    last_mod_prev.clear();
                    i += 1;
                    continue;
                }
            }

            // Thử áp dụng modifier.
            let ap_dung = ap_dung_modifier(
                ky_tu,
                &mut don_vi,
                quy_tac,
                segment_start,
                &mut last_mod_targets,
                &mut last_mod_prev,
            );
            if ap_dung {
                // Mở rộng range / track provenance cho target chính.
                if let Some(&idx) = last_mod_targets.first() {
                    let unit_end = don_vi[idx].raw_ket_thuc;
                    if i == unit_end {
                        don_vi[idx].raw_ket_thuc = i + 1;
                    } else {
                        don_vi[idx].thao_tac_anh_huong.push(i);
                    }
                }
                last_mod = Some(ky_tu);
                last_mod_pos = Some(i);
                i += 1;
                continue;
            }

            // Không áp dụng được → digit literal.
            don_vi.push(DonViRender::chuong(ky_tu, i));
            last_mod = None;
            last_mod_pos = None;
            last_mod_targets.clear();
            last_mod_prev.clear();
            i += 1;
            continue;
        }

        // --- Ký tự chữ (ASCII letter hoặc chữ Việt dựng sẵn) ---
        let chu = match render::phan_tich_ky_tu(ky_tu) {
            Some(c) => c,
            None => ChuCaiViet::thuong(ky_tu),
        };
        if chu.chu_goc.la_nguyen_am() || matches!(chu.chu_goc, ChuGoc::D) {
            don_vi.push(DonViRender::chu(chu, i, i + 1));
        } else {
            don_vi.push(DonViRender::chuong(ky_tu, i));
        }
        // Chữ cái ngắt escape tracking (giống Telex reset tone key).
        last_mod = None;
        last_mod_pos = None;
        last_mod_targets.clear();
        last_mod_prev.clear();
        i += 1;
    }

    KetQuaDoanChu {
        don_vi,
        co_escape,
        co_escape_hinh_chu,
    }
}

/// Tìm nguyên âm cuối cùng tương thích với `dau_chu` (từ `segment_start` trở
/// đi). Dùng cho shape modifier VNI: tìm vowel nhận được shape, không phải
/// "nguyên âm chính" (bán âm rule).
fn tim_nguyen_am_tuong_thich(
    don_vi: &[DonViRender],
    dau_chu: DauChu,
    segment_start: usize,
) -> Option<usize> {
    don_vi
        .iter()
        .enumerate()
        .filter(|(_, u)| u.raw_bat_dau >= segment_start)
        .rev()
        .find(|(_, u)| match &u.noi_dung {
            NoiDungDonVi::Chu(chu) => {
                let goc = chu.chu_goc;
                match dau_chu {
                    DauChu::Mu => tuong_thich_mu(goc),
                    DauChu::Moc => tuong_thich_moc(goc),
                    DauChu::Trang => tuong_thich_trang(goc),
                    _ => false,
                }
            }
            NoiDungDonVi::Chuong(_) => false,
        })
        .map(|(i, _)| i)
}

/// Cập nhật `last_mod_targets`/`last_mod_prev` và áp dụng modifier `digit` tại
/// raw `pos` lên các đơn vị trong `don_vi`. Trả `true` nếu áp dụng được.
///
/// Tìm target theo loại digit:
/// * `1..=5`: nguyên âm chính (BoDatDau).
/// * `6/7/8`: nguyên âm chính nếu tương thích digit.
/// * `9`: `d` gần nhất trong âm tiết (chưa có dấu gạch).
fn ap_dung_modifier(
    digit: char,
    don_vi: &mut [DonViRender],
    quy_tac: QuyTacDatDau,
    segment_start: usize,
    targets: &mut Vec<usize>,
    prev: &mut Vec<TrangThaiTruoc>,
) -> bool {
    targets.clear();
    prev.clear();

    // Dấu thanh `1..=5`.
    if let Some(dau_thanh) = tu_dau_thanh_digit(digit) {
        if let Some(idx) = tim_nguyen_am_chinh(don_vi, quy_tac, segment_start) {
            if let NoiDungDonVi::Chu(ref mut chu) = don_vi[idx].noi_dung {
                prev.push(TrangThaiTruoc {
                    dau_chu: chu.dau_chu,
                    dau_thanh: chu.dau_thanh,
                });
                chu.dau_thanh = dau_thanh;
                targets.push(idx);
                return true;
            }
        }
        return false;
    }

    // `9` gach: `d` gần nhất trong âm tiết chưa có dấu gạch.
    if digit == '9' {
        let mut found: Option<usize> = None;
        for (idx, u) in don_vi.iter().enumerate().rev() {
            if u.raw_bat_dau < segment_start {
                break;
            }
            if let NoiDungDonVi::Chu(chu) = &u.noi_dung {
                if matches!(chu.chu_goc, ChuGoc::D) && matches!(chu.dau_chu, DauChu::Khong) {
                    found = Some(idx);
                    break;
                }
            }
        }
        if let Some(idx) = found {
            if let NoiDungDonVi::Chu(ref mut chu) = don_vi[idx].noi_dung {
                prev.push(TrangThaiTruoc {
                    dau_chu: chu.dau_chu,
                    dau_thanh: chu.dau_thanh,
                });
                chu.dau_chu = DauChu::Gach;
                targets.push(idx);
                return true;
            }
        }
        return false;
    }

    // Hình chữ `6/7/8`: tìm nguyên âm cuối cùng tương thích với digit.
    // Không dùng `tim_nguyen_am_chinh` (bán âm rule) vì shape cần vowel tương
    // thích, không phải "nguyên âm chính" (vd `gio6i` → `6` áp trên `o`, không
    // phải `i` bán âm).
    let dau_chu_moi = match digit {
        '6' => DauChu::Mu,
        '8' => DauChu::Trang,
        '7' => DauChu::Moc,
        _ => return false,
    };
    let idx = match tim_nguyen_am_tuong_thich(don_vi, dau_chu_moi, segment_start) {
        Some(idx) => idx,
        None => return false,
    };
    let goc = match &don_vi[idx].noi_dung {
        NoiDungDonVi::Chu(chu) => chu.chu_goc,
        NoiDungDonVi::Chuong(_) => return false,
    };

    // ươ đặc biệt: `7` trên `u` hoặc `o`, nếu cặp `u`+`o` liền nhau đều chưa
    // có dấu hình chữ → biến đổi cả `u`→`ư` và `o`→`ơ` (giống `uo`+`w` Telex).
    // `tim_nguyen_am_chinh` cho `uo` trả `u` (bán âm `o`); cho `uoi` trả `o`
    // (bán âm `i`). Nên cần kiểm tra cả hai hướng.
    if digit == '7' && matches!(goc, ChuGoc::U | ChuGoc::O) {
        // Tìm cặp u+o liền nhau: idx là `u` và idx+1 là `o`.
        if goc == ChuGoc::U {
            if let Some(sau) = don_vi.get(idx + 1) {
                if let NoiDungDonVi::Chu(chu_sau) = &sau.noi_dung {
                    if matches!(chu_sau.chu_goc, ChuGoc::O)
                        && matches!(chu_sau.dau_chu, DauChu::Khong)
                    {
                        let prev_o = TrangThaiTruoc {
                            dau_chu: chu_sau.dau_chu,
                            dau_thanh: chu_sau.dau_thanh,
                        };
                        if let NoiDungDonVi::Chu(ref mut chu) = don_vi[idx + 1].noi_dung {
                            chu.dau_chu = DauChu::Moc;
                        }
                        prev.push(prev_o);
                        targets.push(idx + 1);
                    }
                }
            }
        }
        // idx là `o` và idx-1 là `u` không dấu.
        if goc == ChuGoc::O && idx > 0 {
            if let Some(truoc) = don_vi.get(idx - 1) {
                if let NoiDungDonVi::Chu(chu_truoc) = &truoc.noi_dung {
                    if matches!(chu_truoc.chu_goc, ChuGoc::U)
                        && matches!(chu_truoc.dau_chu, DauChu::Khong)
                    {
                        let prev_u = TrangThaiTruoc {
                            dau_chu: chu_truoc.dau_chu,
                            dau_thanh: chu_truoc.dau_thanh,
                        };
                        if let NoiDungDonVi::Chu(ref mut chu) = don_vi[idx - 1].noi_dung {
                            chu.dau_chu = DauChu::Moc;
                        }
                        prev.push(prev_u);
                        targets.push(idx - 1);
                    }
                }
            }
        }
    }

    // Biến đổi nguyên âm chính (sau khi đã xử lý `o` phụ nếu ươ).
    if let NoiDungDonVi::Chu(ref mut chu) = don_vi[idx].noi_dung {
        prev.insert(
            0,
            TrangThaiTruoc {
                dau_chu: chu.dau_chu,
                dau_thanh: chu.dau_thanh,
            },
        );
        chu.dau_chu = dau_chu_moi;
        targets.insert(0, idx);
    }

    // Nếu ươ, targets hiện là [idx_u, idx_o] do insert(0,...) rồi push, theo
    // thứ tự raw để hoàn tác nhất quán.
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cau_hinh::QuyTacDatDau;
    use alloc::vec;

    // Các test này chỉ kiểm tra engine ở tầng DonViRender (không qua selection).
    // Selection kỹ thuật + behavior đầu-cuối được khóa trong tests/vni_*.rs.

    /// Dấu thanh đơn nguyên âm.
    #[test]
    fn dau_thanh_don_nguyen_am() {
        let kq = xu_ly_doan_chu(
            &[ThaoTacNhap::tu_dong('a'), ThaoTacNhap::tu_dong('1')],
            QuyTacDatDau::HienDai,
        );
        assert_eq!(kq.don_vi.len(), 1);
        match &kq.don_vi[0].noi_dung {
            NoiDungDonVi::Chu(chu) => {
                assert_eq!(chu.chu_goc, ChuGoc::A);
                assert_eq!(chu.dau_thanh, DauThanh::Sac);
            }
            NoiDungDonVi::Chuong(_) => panic!("a1 phai bien doi"),
        }
    }

    /// Hình chữ mũ/móc/trăng.
    #[test]
    fn hinh_chu_mu_moc_trang() {
        let kq = xu_ly_doan_chu(
            &[ThaoTacNhap::tu_dong('a'), ThaoTacNhap::tu_dong('6')],
            QuyTacDatDau::HienDai,
        );
        assert_eq!(kq.don_vi.len(), 1);
        match &kq.don_vi[0].noi_dung {
            NoiDungDonVi::Chu(chu) => assert_eq!(chu.dau_chu, DauChu::Mu),
            NoiDungDonVi::Chuong(_) => panic!("a6 phai bien doi"),
        }
    }

    /// Kết hợp shape + tone: `a61` → ấ.
    #[test]
    fn ket_hop_shape_tone() {
        let tts = ['a', '6', '1'].map(ThaoTacNhap::tu_dong);
        let kq = xu_ly_doan_chu(&tts, QuyTacDatDau::HienDai);
        assert_eq!(kq.don_vi.len(), 1);
        match &kq.don_vi[0].noi_dung {
            NoiDungDonVi::Chu(chu) => {
                assert_eq!(chu.dau_chu, DauChu::Mu);
                assert_eq!(chu.dau_thanh, DauThanh::Sac);
            }
            NoiDungDonVi::Chuong(_) => panic!("a61 phai bien doi"),
        }
        assert_eq!(kq.don_vi[0].raw_ket_thuc, 3);
    }

    /// Thứ tự đảo `a16` cũng cho ấ.
    #[test]
    fn thu_tu_dao() {
        for raw in ["a61", "a16"] {
            let tts: Vec<ThaoTacNhap> = raw.chars().map(ThaoTacNhap::tu_dong).collect();
            let kq = xu_ly_doan_chu(&tts, QuyTacDatDau::HienDai);
            assert_eq!(kq.don_vi.len(), 1, "{raw}");
            match &kq.don_vi[0].noi_dung {
                NoiDungDonVi::Chu(chu) => {
                    assert_eq!(chu.dau_chu, DauChu::Mu, "{raw}");
                    assert_eq!(chu.dau_thanh, DauThanh::Sac, "{raw}");
                }
                NoiDungDonVi::Chuong(_) => panic!("{raw} phai bien doi"),
            }
        }
    }

    /// Escape `a11` → a1 (hoàn tác sắc, digit đầu literal).
    #[test]
    fn escape_a11() {
        let tts = ['a', '1', '1'].map(ThaoTacNhap::tu_dong);
        let kq = xu_ly_doan_chu(&tts, QuyTacDatDau::HienDai);
        assert!(kq.co_escape);
        assert_eq!(kq.don_vi.len(), 2);
        assert!(matches!(kq.don_vi[0].noi_dung, NoiDungDonVi::Chu(_)));
        assert!(matches!(kq.don_vi[1].noi_dung, NoiDungDonVi::Chuong('1')));
    }

    /// `0` không phải modifier VNI → literal.
    #[test]
    fn khong_la_modifier() {
        let kq = xu_ly_doan_chu(&[ThaoTacNhap::tu_dong('0')], QuyTacDatDau::HienDai);
        assert_eq!(kq.don_vi.len(), 1);
        assert!(matches!(kq.don_vi[0].noi_dung, NoiDungDonVi::Chuong('0')));
    }

    /// `toi6` → tôi (mũ trên `o`, bỏ qua bán âm `i`).
    #[test]
    fn toi6_thanh_toi() {
        let tts: Vec<ThaoTacNhap> = "toi6".chars().map(ThaoTacNhap::tu_dong).collect();
        let kq = xu_ly_doan_chu(&tts, QuyTacDatDau::HienDai);
        // Tìm đơn vị có dấu mũ.
        let co_mu = kq
            .don_vi
            .iter()
            .any(|u| matches!(&u.noi_dung, NoiDungDonVi::Chu(c) if c.dau_chu == DauChu::Mu));
        assert!(co_mu, "toi6 phai co mũ trên o");
    }

    /// ươ đặc biệt: `uo7` → ươ (cả u và o đều có móc).
    #[test]
    fn uo7_thanh_uoua() {
        let tts: Vec<ThaoTacNhap> = "uo7".chars().map(ThaoTacNhap::tu_dong).collect();
        let kq = xu_ly_doan_chu(&tts, QuyTacDatDau::HienDai);
        let nguyen_am: Vec<DauChu> = kq
            .don_vi
            .iter()
            .filter_map(|u| match &u.noi_dung {
                NoiDungDonVi::Chu(c) if c.chu_goc.la_nguyen_am() => Some(c.dau_chu),
                _ => None,
            })
            .collect();
        assert_eq!(nguyen_am, vec![DauChu::Moc, DauChu::Moc], "uo7 -> uoua");
    }
}
