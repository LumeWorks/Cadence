// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Phân loại nội dung snapshot.

/// Loại nội dung mà snapshot đang giữ.
///
/// Phase 1 chỉ có hai trạng thái: rỗng và nguyên bản. Phase 2 sẽ thêm
/// trạng thái Telex khi có hành vi thật.
///
/// Có derive serde khi bật feature `serde` để host có thể lưu/phân loại
/// loại nội dung; đây không phải snapshot nên không vi phạm chính sách
/// không serialize snapshot trong Phase 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoaiNoiDung {
    /// Phiên rỗng, chưa có thao tác nào.
    Trong,
    /// Nội dung nguyên bản người dùng nhập, chưa biến đổi.
    NguyenBan,
}
