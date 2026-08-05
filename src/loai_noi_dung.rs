// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Phân loại nội dung snapshot.

/// Loại nội dung mà snapshot đang giữ.
///
/// Phase 1 chỉ có hai trạng thái: rỗng và nguyên bản. Phase 2 sẽ thêm
/// trạng thái Telex khi có hành vi thật.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoaiNoiDung {
    /// Phiên rỗng, chưa có thao tác nào.
    Trong,
    /// Nội dung nguyên bản người dùng nhập, chưa biến đổi.
    NguyenBan,
}
