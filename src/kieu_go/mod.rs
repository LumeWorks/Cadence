// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Các kiểu gõ tiếng Việt và hạ tầng ký tự hỗ trợ.
//!
//! Module này gom các kiểu gõ (Telex, VNI) cùng hạ tầng ký tự Việt phục vụ
//! việc gõ: mô hình chữ viết, render Unicode, đơn vị render chung, bộ đặt
//! dấu thanh chung, phân tích âm tiết, và lựa chọn raw/biến đổi. Toàn bộ
//! `pub(crate)`, không lộ ra public API.
//!
//! Kiến trúc nhiều kiểu gõ (RFC 0020): Telex và VNI chỉ khác cách diễn giải
//! raw action thành ý định chữ Việt. Các lớp dùng chung là đơn vị render,
//! bộ đặt dấu thanh, parser âm tiết, render Unicode, selection, phân đoạn,
//! ngữ cảnh, cursor/provenance và trace. Không copy pipeline; không dynamic
//! dispatch.

pub(crate) mod am_tiet;
pub(crate) mod bo_dat_dau;
pub(crate) mod chu_viet;
pub(crate) mod don_vi;
pub(crate) mod lua_chon;
pub(crate) mod render;
pub(crate) mod telex;
pub(crate) mod vni;
