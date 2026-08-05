// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Trace có cấu trúc cho quyết định lựa chọn raw/Telex.
//!
//! Chỉ tồn tại khi feature `trace` bật. Không overhead khi tắt - toàn bộ
//! module bị `cfg` ẩn.

use alloc::string::String;

use crate::ngu_canh::BangChungLuaChon;

/// Kết quả lựa chọn cho một đoạn: Telex hay nguyên bản.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceKetQua {
    /// Đoạn được biến đổi Telex.
    Telex,
    /// Đoạn giữ nguyên bản (raw).
    NguyenBan,
}

/// Một bước trace: quyết định cho một đoạn raw.
#[derive(Debug, Clone)]
pub struct TraceStep {
    /// Vị trí raw đầu (inclusive) trong lịch sử.
    pub doan_bat_dau: usize,
    /// Vị trí raw cuối (exclusive).
    pub doan_ket_thuc: usize,
    /// Bằng chứng quyết định.
    pub bang_chung: BangChungLuaChon,
    /// Kết quả: Telex hay nguyên bản.
    pub ket_qua: TraceKetQua,
    /// Chuỗi raw của đoạn.
    pub chuoi_raw: String,
    /// Chuỗi output của đoạn.
    pub chuoi_ra: String,
}
