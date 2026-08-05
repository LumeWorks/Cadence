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

/// Kiểu Telex điều khiển hành vi phím `w` và phím gõ nhanh.
///
/// `CanBang` tối ưu cho code/chat hiện đại: `w` chỉ là modifier khi có chữ
/// phù hợp, `w` đơn lẻ giữ nguyên. `DayDu` cho phép `w` đơn lối thành `ư`
/// và `[`/`]` gõ nhanh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KieuTelex {
    /// Cân bằng: `w` đơn lẻ giữ nguyên, không xử lý `[`/`]`.
    CanBang,
    /// Đầy đủ: `w` đơn lẻ thành `ư`, `[`→`ư`, `]`→`ơ`.
    DayDu,
}

/// Quy tắc đặt dấu thanh trên nguyên âm của âm tiết.
///
/// `HienDai` đặt dấu theo quy tắc hiện đại (VD: `hòa`, `hóa`). `TruyenThong`
/// đặt dấu theo quy tắc truyền thống (VD: `hoà`, `hoá`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum QuyTacDatDau {
    /// Quy tắc hiện đại.
    HienDai,
    /// Quy tắc truyền thống.
    TruyenThong,
}

/// Dạng Unicode của output.
///
/// `Nfc` (mặc định) dùng ký tự dựng sẵn. `Nfd` dùng combining mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DangUnicode {
    /// NFC: normalization form composed (dựng sẵn).
    Nfc,
    /// NFD: normalization form decomposed (combining mark).
    Nfd,
}

/// Cấu hình điều khiển hành vi của [`BoGo`](crate::BoGo) và các phiên do nó tạo.
///
/// Field là private; thay đổi thông qua method có kiểm tra hợp lệ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CauHinh {
    /// Số thao tác tối đa một phiên được giữ trước khi từ chối thêm.
    gioi_han_thao_tac: usize,
    /// Kiểu Telex (cân bằng hay đầy đủ).
    kieu_telex: KieuTelex,
    /// Quy tắc đặt dấu thanh (hiện đại hay truyền thống).
    quy_tac_dat_dau: QuyTacDatDau,
    /// Dạng Unicode output (NFC hay NFD).
    dang_unicode: DangUnicode,
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
    /// Tạo cấu hình mặc định (128 thao tác, Telex cân bằng, dấu hiện đại, NFC).
    #[must_use]
    pub fn mac_dinh() -> Self {
        Self {
            gioi_han_thao_tac: GIOI_HAN_MAC_DINH,
            kieu_telex: KieuTelex::CanBang,
            quy_tac_dat_dau: QuyTacDatDau::HienDai,
            dang_unicode: DangUnicode::Nfc,
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

    /// Trả kiểu Telex hiện tại.
    #[must_use]
    pub fn kieu_telex(self) -> KieuTelex {
        self.kieu_telex
    }

    /// Đặt kiểu Telex.
    pub fn dat_kieu_telex(&mut self, kieu_telex: KieuTelex) {
        self.kieu_telex = kieu_telex;
    }

    /// Trả quy tắc đặt dấu hiện tại.
    #[must_use]
    pub fn quy_tac_dat_dau(self) -> QuyTacDatDau {
        self.quy_tac_dat_dau
    }

    /// Đặt quy tắc đặt dấu.
    pub fn dat_quy_tac_dat_dau(&mut self, quy_tac: QuyTacDatDau) {
        self.quy_tac_dat_dau = quy_tac;
    }

    /// Trả dạng Unicode output hiện tại.
    #[must_use]
    pub fn dang_unicode(self) -> DangUnicode {
        self.dang_unicode
    }

    /// Đặt dạng Unicode output.
    pub fn dat_dang_unicode(&mut self, dang_unicode: DangUnicode) {
        self.dang_unicode = dang_unicode;
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
