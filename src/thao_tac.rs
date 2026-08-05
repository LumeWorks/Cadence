// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Mô hình thao tác nhập nội bộ — nguồn sự thật của phiên.

/// Cách nhập của một ký tự.
///
/// Phase 1 chưa có Telex nên `TuDong` và `NguyenBan` cho cùng kết quả
/// hiển thị. Hai giá trị vẫn được phân biệt để Phase 2 dùng lịch sử quyết
/// định Telex mà không phá public API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CachNhap {
    /// Nhập tự động (sẽ do Telex sinh ra trong Phase 2).
    TuDong,
    /// Nhập nguyên bản, giữ đúng ký tự người dùng bấm.
    NguyenBan,
}

/// Một thao tác nhập ký tự trong lịch sử phiên.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ThaoTacNhap {
    /// Ký tự người dùng nhập.
    pub(crate) ky_tu: char,
    /// Cách nhập ký tự này.
    pub(crate) cach_nhap: CachNhap,
}

impl ThaoTacNhap {
    /// Tạo thao tác nhập nguyên bản.
    pub(crate) fn nguyen_ban(ky_tu: char) -> Self {
        Self {
            ky_tu,
            cach_nhap: CachNhap::NguyenBan,
        }
    }

    /// Tạo thao tác nhập tự động.
    pub(crate) fn tu_dong(ky_tu: char) -> Self {
        Self {
            ky_tu,
            cach_nhap: CachNhap::TuDong,
        }
    }
}
