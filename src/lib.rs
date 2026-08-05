// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Cadence — lõi gõ tiếng Việt thế hệ mới.
//!
//! Phase 1 chỉ dựng nền móng bất biến: nhận và giữ nguyên mọi ký tự
//! người dùng nhập, duy trì lịch sử thao tác, hỗ trợ con trỏ trong
//! đoạn đang soạn và snapshot trung lập nền tảng. Telex chưa được
//! triển khai trong giai đoạn này.

//! Cadence mặc định dùng `std`, nhưng vẫn biên dịch được với `no_std + alloc`.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

pub mod cau_hinh;

pub use cau_hinh::{CauHinh, LoiCauHinh};
