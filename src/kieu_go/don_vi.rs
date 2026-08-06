// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Đơn vị render dùng chung cho mọi kiểu gõ.
//!
//! Cả Telex và VNI đều xuất ra danh sách [`DonViRender`] mang provenance
//! (thao tác raw nào sinh ra đơn vị này). Tầng này trung lập kiểu gõ; chỉ
//! module render Unicode mới biết `ế` ứng với code point nào.

use alloc::vec::Vec;

use super::chu_viet::ChuCaiViet;

/// Nội dung một đơn vị render.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NoiDungDonVi {
    /// Chữ cái Việt đã biến đổi.
    Chu(ChuCaiViet),
    /// Ký tự literal (không phải kiểu gõ modifier, hoặc đã escape).
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
    /// Thao tác raw ảnh hưởng thêm (dấu thanh / modifier số) nhưng không
    /// nằm trong khoảng liên tục.
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

/// Kết quả xử lý đoạn chữ từ một kiểu gõ (Telex hoặc VNI).
pub(crate) struct KetQuaDoanChu {
    /// Danh sách đơn vị render.
    pub(crate) don_vi: Vec<DonViRender>,
    /// Có escape lặp modifier xảy ra.
    pub(crate) co_escape: bool,
    /// Escape là escape hình chữ/modifier (lặp phím modifier đang hoạt động).
    pub(crate) co_escape_hinh_chu: bool,
}
