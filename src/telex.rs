// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Bộ nhận Telex — biến đổi raw input thành đơn vị render có provenance.
//!
//! Module này là tầng giữa: nhận các thao tác raw trong một đoạn chữ, áp
//! dụng rule Telex (hình chữ, dấu thanh, escape) và xuất ra `DonViRender`
//! mang theo provenance (thao tác raw nào sinh ra đơn vị này).

use alloc::vec::Vec;

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
///
/// `raw_bat_dau..raw_ket_thuc` là khoảng raw liên tục tạo ra đơn vị này.
/// `thao_tac_anh_huong` là các thao tác raw khác ảnh hưởng (dấu thanh) nhưng
/// không nằm trong khoảng liên tục.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DonViRender {
    /// Nội dung (chữ biến đổi hoặc literal).
    pub(crate) noi_dung: NoiDungDonVi,
    /// Vị trí raw đầu (inclusive).
    pub(crate) raw_bat_dau: usize,
    /// Vị trí raw cuối (exclusive).
    pub(crate) raw_ket_thuc: usize,
    /// Thao tác raw ảnh hưởng thêm (dấu thanh).
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
///
/// Cả hai ký tự được match theo dạng thường. Trả `None` nếu không phải cặp
/// Telex hợp lệ.
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

/// Xử lý một đoạn chữ (liên tục các thao tác raw thuộc đoạn chữ) thành danh
/// sách `DonViRender`.
///
/// Phase 2 bước hình chữ: chỉ áp dụng biến đổi hình chữ (aa/aw/ee/oo/ow/uw/dd)
/// và giữ nguyên ký tự nguyên bản (`CachNhap::NguyenBan`). Dấu thanh và escape
/// sẽ thêm ở commit sau.
pub(crate) fn xu_ly_doan_chu(cac_thao_tac: &[ThaoTacNhap]) -> Vec<DonViRender> {
    let mut don_vi: Vec<DonViRender> = Vec::new();
    let n = cac_thao_tac.len();
    let mut i = 0usize;
    while i < n {
        let ky_tu = cac_thao_tac[i].ky_tu;
        let cach_nhap = cac_thao_tac[i].cach_nhap;
        // Ký tự nguyên bản: không bao giờ làm modifier, luôn literal.
        if cach_nhap == CachNhap::NguyenBan {
            don_vi.push(DonViRender::chuong(ky_tu, i));
            i += 1;
            continue;
        }
        // Ký tự tự động: thử cap hình chữ với ký tự tiếp theo (cũng tự động).
        if i + 1 < n {
            let tiep = cac_thao_tac[i + 1].ky_tu;
            let tiep_cach = cac_thao_tac[i + 1].cach_nhap;
            if tiep_cach == CachNhap::TuDong {
                if let Some((chu_goc, dau_chu)) = cap_hinh_chu(ky_tu, tiep) {
                    let kieu_hoa = KieuHoa::tu_ky_tu(ky_tu);
                    let chu = ChuCaiViet {
                        chu_goc,
                        dau_chu,
                        dau_thanh: DauThanh::Khong,
                        kieu_hoa,
                    };
                    don_vi.push(DonViRender::chu(chu, i, i + 2));
                    i += 2;
                    continue;
                }
            }
        }
        // Không phải cặp Telex: giữ nguyên (phân tích precomposed nếu có).
        let chu = match render::phan_tich_ky_tu(ky_tu) {
            Some(c) => c,
            None => ChuCaiViet::thuong(ky_tu),
        };
        if chu.chu_goc.la_nguyen_am() || matches!(chu.chu_goc, ChuGoc::D) {
            don_vi.push(DonViRender::chu(chu, i, i + 1));
        } else {
            // Phụ âm literal.
            don_vi.push(DonViRender::chuong(ky_tu, i));
        }
        i += 1;
    }
    don_vi
}
