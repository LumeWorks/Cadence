// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Bộ gõ — factory bất biến tạo phiên.

use crate::cau_hinh::{CauHinh, LoiCauHinh};
use crate::phien_go::PhienGo;

/// Factory bất biến điều khiển cấu hình chung cho mọi phiên.
///
/// Hai phiên tạo từ cùng [`BoGo`] hoàn toàn độc lập: không có shared
/// mutable state giữa chúng.
#[derive(Debug, Clone)]
pub struct BoGo {
    /// Cấu hình dùng cho mọi phiên do bộ gõ này tạo.
    cau_hinh: CauHinh,
}

impl BoGo {
    /// Tạo bộ gõ từ cấu hình. Trả lỗi nếu cấu hình không hợp lệ.
    pub fn new(cau_hinh: CauHinh) -> Result<Self, LoiCauHinh> {
        // CauHinh chỉ có thể mang giá trị hợp lệ nhờ validation trong
        // dat_gioi_han_thao_tac, nên không cần kiểm tra lại.
        Ok(Self { cau_hinh })
    }

    /// Tạo một phiên mới, độc lập hoàn toàn với các phiên khác.
    #[must_use]
    pub fn tao_phien(&self) -> PhienGo {
        PhienGo::moi(self.cau_hinh)
    }

    /// Trả cấu hình hiện tại của bộ gõ.
    #[must_use]
    pub fn cau_hinh(&self) -> &CauHinh {
        &self.cau_hinh
    }
}
