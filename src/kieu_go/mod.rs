// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Các kiểu gõ tiếng Việt và hạ tầng ký tự hỗ trợ.
//!
//! Module này gom các kiểu gõ (hiện có Telex) cùng hạ tầng ký tự Việt
//! phục vụ việc gõ: mô hình chữ viết, render Unicode, phân tích âm tiết,
//! và lựa chọn raw/Telex. Toàn bộ `pub(crate)`, không lộ ra public API.

pub(crate) mod am_tiet;
pub(crate) mod chu_viet;
pub(crate) mod lua_chon;
pub(crate) mod render;
pub(crate) mod telex;
