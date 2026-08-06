// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Bộ đặt dấu thanh dùng chung cho mọi kiểu gõ.
//!
//! Cả Telex và VNI đều dùng cùng quy tắc chọn nguyên âm chính (nguyên âm mang
//! dấu thanh) trên một cụm nguyên âm. Việc này đảm bảo `hoa2`/`hoa.f`,
//! `thuy3`/`thuy.r`, `quy1`/`quy.s` cho cùng vị trí dấu bất kể kiểu gõ.
//!
//! Quy tắc nguyên âm chính (2026.1):
//! 1. Bán âm cuối `i`/`u`/`o`/`y` không dấu hình chữ → dấu trên nguyên âm trước.
//!    Ngoại lệ: `y` sau `qu` (onset `qu`) là nucleus, không phải bán âm → dấu
//!    trên `y` (vd `quý`). Điều này sửa `thủy`/`tùy`/`huỷ` (dấu trên `u`) mà
//!    không phá `quý`.
//! 2. On-glide `o`+`a`/`e`: `HienDai` đặt trên `o`, `TruyenThong` đặt trên
//!    `a`/`e`.
//! 3. Ngược lại, đặt trên nguyên âm cuối.

use super::chu_viet::{ChuGoc, DauChu};
use super::don_vi::{DonViRender, NoiDungDonVi};
use crate::cau_hinh::QuyTacDatDau;
use alloc::vec::Vec;

/// Tìm index của đơn vị nguyên âm cuối cùng trong `don_vi` (từ `min_raw` trở đi).
pub(crate) fn tim_nguyen_am_cuoi(don_vi: &[DonViRender], min_raw: usize) -> Option<usize> {
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

/// Trả `true` nếu đơn vị tại `idx` là nguyên âm `chu_goc` không dấu hình chữ.
fn la_nguyen_am_khong_dau_chu(don_vi: &[DonViRender], idx: usize, goc: ChuGoc) -> bool {
    matches!(&don_vi[idx].noi_dung, NoiDungDonVi::Chu(chu) if chu.chu_goc == goc
        && matches!(chu.dau_chu, DauChu::Khong))
}

/// Tìm index của nguyên âm chính (nguyên âm mang dấu thanh).
pub(crate) fn tim_nguyen_am_chinh(
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
        // Bán âm cuối: `i`, `u`, `o`, `y` (không dấu hình chữ) → tone trên
        // nguyên âm trước. `y` chỉ là bán âm khi KHÔNG theo sau `qu` (nếu theo
        // sau `qu` thì `y` là nucleus, vd `quý`).
        if matches!(don_vi[cuoi].noi_dung, NoiDungDonVi::Chu(chu) if matches!(
            chu.chu_goc,
            ChuGoc::I | ChuGoc::U | ChuGoc::O | ChuGoc::Y
        ) && matches!(chu.dau_chu, DauChu::Khong))
        {
            let truoc = cac_nguyen_am[cac_nguyen_am.len() - 2];
            // Ngoại lệ `qu` + `y`: `y` là nucleus → giữ `y` (cuoi).
            // `q` được lưu là `Chuong('q')` (phụ âm không phải nguyên âm/D),
            // nên phải kiểm tra cả `Chu(PhuAm('q'))` lẫn `Chuong`.
            if matches!(don_vi[cuoi].noi_dung, NoiDungDonVi::Chu(chu) if chu.chu_goc == ChuGoc::Y)
                && la_nguyen_am_khong_dau_chu(don_vi, truoc, ChuGoc::U)
                && truoc > 0
                && matches!(
                    don_vi[truoc - 1].noi_dung,
                    NoiDungDonVi::Chu(chu) if matches!(chu.chu_goc, ChuGoc::PhuAm('q'))
                )
                || matches!(don_vi[cuoi].noi_dung, NoiDungDonVi::Chu(chu) if chu.chu_goc == ChuGoc::Y)
                    && la_nguyen_am_khong_dau_chu(don_vi, truoc, ChuGoc::U)
                    && truoc > 0
                    && matches!(
                        don_vi[truoc - 1].noi_dung,
                        NoiDungDonVi::Chuong('q') | NoiDungDonVi::Chuong('Q')
                    )
            {
                return Some(cuoi);
            }
            return Some(truoc);
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

/// Tìm vị trí insert trong `don_vi` sao cho đơn vị mới chiếm raw `pos` nằm
/// đúng thứ tự raw.
pub(crate) fn vi_tri_chen(don_vi: &[DonViRender], pos: usize) -> usize {
    don_vi
        .iter()
        .position(|u| u.raw_bat_dau > pos)
        .unwrap_or(don_vi.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cau_hinh::QuyTacDatDau;
    use crate::kieu_go::chu_viet::{ChuCaiViet, ChuGoc, DauChu, DauThanh, KieuHoa};

    fn chu_v(goc: ChuGoc, raw: usize) -> DonViRender {
        DonViRender::chu(
            ChuCaiViet {
                chu_goc: goc,
                dau_chu: DauChu::Khong,
                dau_thanh: DauThanh::Khong,
                kieu_hoa: KieuHoa::Thuong,
            },
            raw,
            raw + 1,
        )
    }

    fn chu_phu(c: char, raw: usize) -> DonViRender {
        DonViRender::chu(ChuCaiViet::thuong(c), raw, raw + 1)
    }

    /// `thuy` + tone → nguyên âm chính là `u` (bán âm `y`), tone trên `u`.
    #[test]
    fn thuy_nguyen_am_chinh_la_u() {
        let dv = [
            chu_phu('t', 0),
            chu_phu('h', 1),
            chu_v(ChuGoc::U, 2),
            chu_v(ChuGoc::Y, 3),
        ];
        let idx = tim_nguyen_am_chinh(&dv, QuyTacDatDau::HienDai, 0).expect("thuy co nguyen am");
        assert_eq!(idx, 2, "tone phai dat tren u (thuy -> thuy)");
    }

    /// `quy` + tone → nguyên âm chính là `y` (nucleus sau onset `qu`), tone trên `y`.
    #[test]
    fn quy_nguyen_am_chinh_la_y() {
        let dv = [chu_phu('q', 0), chu_v(ChuGoc::U, 1), chu_v(ChuGoc::Y, 2)];
        let idx = tim_nguyen_am_chinh(&dv, QuyTacDatDau::HienDai, 0).expect("quy co nguyen am");
        assert_eq!(idx, 2, "tone phai dat tren y (quy -> quy)");
    }

    /// `ay` + tone → bán âm `y`, tone trên `a` (`ay` -> `ay`).
    #[test]
    fn ay_nguyen_am_chinh_la_a() {
        let dv = [chu_v(ChuGoc::A, 0), chu_v(ChuGoc::Y, 1)];
        let idx = tim_nguyen_am_chinh(&dv, QuyTacDatDau::HienDai, 0).expect("ay co nguyen am");
        assert_eq!(idx, 0);
    }

    /// `quyen` (3 nguyên âm) → vẫn tone trên `e` (không bị quy tắc `y` ảnh hưởng).
    #[test]
    fn quyen_nguyen_am_chinh_la_e() {
        let dv = [
            chu_phu('q', 0),
            chu_v(ChuGoc::U, 1),
            chu_v(ChuGoc::Y, 2),
            chu_v(ChuGoc::E, 3),
            chu_phu('n', 4),
        ];
        let idx = tim_nguyen_am_chinh(&dv, QuyTacDatDau::HienDai, 0).expect("quyen co nguyen am");
        assert_eq!(idx, 3);
    }

    /// `khuỷu`: 3 nguyên âm `u y u`, bán âm cuối `u` → tone trên `y`.
    #[test]
    fn khuyu_nguyen_am_chinh_la_y() {
        let dv = [
            chu_phu('k', 0),
            chu_phu('h', 1),
            chu_v(ChuGoc::U, 2),
            chu_v(ChuGoc::Y, 3),
            chu_v(ChuGoc::U, 4),
        ];
        let idx = tim_nguyen_am_chinh(&dv, QuyTacDatDau::HienDai, 0).expect("khuyu co nguyen am");
        assert_eq!(idx, 3);
    }
}
