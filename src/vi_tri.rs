// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Vị trí con trỏ trong văn bản theo ba đơn vị trung lập nền tảng.

use unicode_segmentation::UnicodeSegmentation;

/// Vị trí con trỏ tính theo byte, đơn vị UTF-16 và grapheme cluster.
///
/// Snapshot công bố vị trí theo cả ba đơn vị để host application trên
/// các nền tảng khác nhau (dùng UTF-16 như Windows TSF, hoặc grapheme
/// như trình soạn thảo) không phải tự tính lại.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViTriVanBan {
    /// Vị trí tính theo byte trong chuỗi UTF-8.
    chi_so_byte: usize,
    /// Vị trí tính theo đơn vị UTF-16.
    chi_so_utf16: usize,
    /// Vị trí tính theo grapheme cluster.
    chi_so_grapheme: usize,
}

impl ViTriVanBan {
    /// Tạo vị trí đầu (tọa độ 0/0/0).
    #[must_use]
    pub(crate) fn dau() -> Self {
        Self {
            chi_so_byte: 0,
            chi_so_utf16: 0,
            chi_so_grapheme: 0,
        }
    }

    /// Tính vị trí con trỏ tại ranh giới byte `chi_so_byte` trong `van_ban`.
    ///
    /// `chi_so_byte` phải là ranh giới ký tự UTF-8 hợp lệ (luôn đúng khi
    /// con trỏ nội bộ nằm giữa hai `char`). Hàm tính đơn vị UTF-16 và
    /// grapheme cluster tương ứng từ tiền tố `van_ban[..chi_so_byte]`.
    #[must_use]
    pub(crate) fn tai_byte(van_ban: &str, chi_so_byte: usize) -> Self {
        debug_assert!(
            van_ban.is_char_boundary(chi_so_byte),
            "chi_so_byte phai la ranh gioi UTF-8"
        );
        let tien_to = &van_ban[..chi_so_byte];
        Self {
            chi_so_byte,
            chi_so_utf16: tien_to.encode_utf16().count(),
            chi_so_grapheme: tien_to.graphemes(true).count(),
        }
    }

    /// Trả vị trí theo byte UTF-8.
    #[must_use]
    pub fn chi_so_byte(self) -> usize {
        self.chi_so_byte
    }

    /// Trả vị trí theo đơn vị UTF-16.
    #[must_use]
    pub fn chi_so_utf16(self) -> usize {
        self.chi_so_utf16
    }

    /// Trả vị trí theo grapheme cluster.
    #[must_use]
    pub fn chi_so_grapheme(self) -> usize {
        self.chi_so_grapheme
    }
}
