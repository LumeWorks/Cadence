// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Cadence — lõi gõ tiếng Việt thế hệ mới.
//!
//! Phase 1 chỉ dựng nền móng bất biến: nhận và giữ nguyên mọi ký tự
//! người dùng nhập, duy trì lịch sử thao tác, hỗ trợ con trỏ trong
//! đoạn đang soạn và snapshot trung lập nền tảng. Telex chưa được
//! triển khai trong giai đoạn này.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
