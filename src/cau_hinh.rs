// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Cấu hình của bộ gõ.

use core::fmt;

/// Giới hạn thao tác tối thiểu cho một phiên.
pub(crate) const GIOI_HAN_TOI_THIEU: usize = 1;

/// Giới hạn thao tác tối đa cho một phiên.
pub(crate) const GIOI_HAN_TOI_DA: usize = 4096;

/// Giới hạn thao tác mặc định cho một phiên.
const GIOI_HAN_MAC_DINH: usize = 128;

/// Cấu hình điều khiển hành vi của [`BoGo`](crate::BoGo) và các phiên do nó tạo.
///
/// Field là private; thay đổi thông qua method có kiểm tra hợp lệ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CauHinh {
    /// Số thao tác tối đa một phiên được giữ trước khi từ chối thêm.
    gioi_han_thao_tac: usize,
}

/// Lỗi cấu hình. Dùng enum domain thay vì `String` chung chung để caller
/// có thể match chính xác nguyên nhân.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoiCauHinh {
    /// Giới hạn thao tác nằm ngoài khoảng hợp lệ.
    GioiHanThaoTacKhongHopLe {
        /// Giá trị người dùng truyền vào.
        gioi_han: usize,
        /// Giới hạn tối thiểu hợp lệ.
        toi_thieu: usize,
        /// Giới hạn tối đa hợp lệ.
        toi_da: usize,
    },
}

impl CauHinh {
    /// Tạo cấu hình mặc định (128 thao tác).
    #[must_use]
    pub fn mac_dinh() -> Self {
        Self {
            gioi_han_thao_tac: GIOI_HAN_MAC_DINH,
        }
    }

    /// Trả giới hạn thao tác hiện tại.
    #[must_use]
    pub fn gioi_han_thao_tac(self) -> usize {
        self.gioi_han_thao_tac
    }

    /// Đặt giới hạn thao tác mới. Trả lỗi nếu nằm ngoài `1..=4096`.
    ///
    /// Khi lỗi, giá trị cũ được giữ nguyên.
    pub fn dat_gioi_han_thao_tac(&mut self, gioi_han: usize) -> Result<(), LoiCauHinh> {
        if (GIOI_HAN_TOI_THIEU..=GIOI_HAN_TOI_DA).contains(&gioi_han) {
            self.gioi_han_thao_tac = gioi_han;
            Ok(())
        } else {
            Err(LoiCauHinh::GioiHanThaoTacKhongHopLe {
                gioi_han,
                toi_thieu: GIOI_HAN_TOI_THIEU,
                toi_da: GIOI_HAN_TOI_DA,
            })
        }
    }
}

impl fmt::Display for LoiCauHinh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GioiHanThaoTacKhongHopLe {
                gioi_han,
                toi_thieu,
                toi_da,
            } => {
                write!(
                    f,
                    "gioi han thao tac {gioi_han} khong hop le, phai nam trong {toi_thieu}..={toi_da}"
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LoiCauHinh {}
