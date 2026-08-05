// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Phân loại nội dung snapshot.

/// Loại nội dung mà snapshot đang giữ.
///
/// Phase 2 thêm trạng thái Telex và âm tiết tiếng Việt.
///
/// Có derive serde khi bật feature `serde` để host có thể lưu/phân loại
/// loại nội dung; đây không phải snapshot nên không vi phạm chính sách
/// không serialize snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoaiNoiDung {
    /// Phiên rỗng, chưa có thao tác nào.
    Trong,
    /// Nội dung nguyên bản người dùng nhập, chưa biến đổi (output bằng raw).
    NguyenBan,
    /// Có biến đổi Telex, nhưng toàn đoạn chưa được xác nhận là một âm tiết
    /// hoàn chỉnh.
    BienDoiTelex,
    /// Đoạn hiện tại tạo thành một âm tiết tiếng Việt hợp lệ.
    AmTietTiengViet,
}
