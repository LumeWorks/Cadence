// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Kết quả xử lý một thao tác trên phiên.

use alloc::string::String;

/// Kết quả trả về từ các method thao tác trên [`PhienGo`](crate::PhienGo).
///
/// Không trả snapshot clone trong `CapNhat`. Người gọi lấy snapshot qua
/// [`PhienGo::ban_chup`](crate::PhienGo::ban_chup).
///
/// Có derive serde khi bật feature `serde` để host có thể ghi log/lưu kết
/// quả xử lý; đây là kết quả thao tác, không phải snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
