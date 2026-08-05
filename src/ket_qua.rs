// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Kết quả xử lý một thao tác trên phiên.

use alloc::string::String;

/// Kết quả trả về từ các method thao tác trên [`PhienGo`](crate::PhienGo).
///
/// Không trả snapshot clone trong `CapNhat`. Người gọi lấy snapshot qua
/// [`PhienGo::ban_chup`](crate::PhienGo::ban_chup).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KetQuaXuLy {
    /// Thao tác không làm thay đổi state (ví dụ vượt giới hạn, hoặc xóa
    /// khi phiên rỗng).
    KhongDoi,
    /// State phiên đã thay đổi; snapshot mới có thể lấy qua `ban_chup`.
    CapNhat,
    /// Phiên đã được commit. Trả nội dung đã commit; phiên trở về rỗng.
    ChapNhan {
        /// Nội dung đã commit.
        noi_dung: String,
    },
}
