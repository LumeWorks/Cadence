// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Snapshot trung lập nền tảng của đoạn đang soạn.

use alloc::string::String;

use crate::loai_noi_dung::LoaiNoiDung;
use crate::vi_tri::ViTriVanBan;

/// Snapshot không đổi của đoạn đang soạn tại một thời điểm.
///
/// Phase 1 luôn có `noi_dung == noi_dung_goc` vì chưa có Telex. Hai khái
/// niệm vẫn được giữ riêng để Phase 2 biến đổi `noi_dung` mà không phá
/// raw input trong `noi_dung_goc`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BanChupSoan {
    /// Nội dung hiển thị (Phase 1: bằng `noi_dung_goc`).
    noi_dung: String,
    /// Nội dung gốc người dùng nhập, chưa biến đổi.
    noi_dung_goc: String,
    /// Vị trí con trỏ theo byte/UTF-16/grapheme.
    con_tro: ViTriVanBan,
    /// Loại nội dung hiện tại.
    loai_noi_dung: LoaiNoiDung,
}

impl BanChupSoan {
    /// Tạo snapshot rỗng.
    #[must_use]
    pub(crate) fn rong() -> Self {
        Self {
            noi_dung: String::new(),
            noi_dung_goc: String::new(),
            con_tro: ViTriVanBan::dau(),
            loai_noi_dung: LoaiNoiDung::Trong,
        }
    }

    /// Dựng snapshot từ nội dung gốc đã render và vị trí byte con trỏ.
    #[must_use]
    pub(crate) fn dung(noi_dung_goc: String, chi_so_byte: usize) -> Self {
        let loai_noi_dung = if noi_dung_goc.is_empty() {
            LoaiNoiDung::Trong
        } else {
            LoaiNoiDung::NguyenBan
        };
        let con_tro = ViTriVanBan::tai_byte(&noi_dung_goc, chi_so_byte);
        // Phase 1: hiển thị bằng nội dung gốc.
        let noi_dung = noi_dung_goc.clone();
        Self {
            noi_dung,
            noi_dung_goc,
            con_tro,
            loai_noi_dung,
        }
    }

    /// Trả nội dung hiển thị.
    #[must_use]
    pub fn noi_dung(&self) -> &str {
        &self.noi_dung
    }

    /// Trả nội dung gốc, chưa biến đổi.
    #[must_use]
    pub fn noi_dung_goc(&self) -> &str {
        &self.noi_dung_goc
    }

    /// Trả vị trí con trỏ.
    #[must_use]
    pub fn con_tro(&self) -> ViTriVanBan {
        self.con_tro
    }

    /// Trả loại nội dung.
    #[must_use]
    pub fn loai_noi_dung(&self) -> LoaiNoiDung {
        self.loai_noi_dung
    }

    /// Trả `true` nếu snapshot rỗng.
    #[must_use]
    pub fn dang_trong(&self) -> bool {
        self.noi_dung_goc.is_empty()
    }
}
