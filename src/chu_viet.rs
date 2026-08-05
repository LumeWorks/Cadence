// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Mô hình domain chữ Việt.
//!
//! Mô hình tách bốn khía cạnh của một chữ cái tiếng Việt:
//!
//! ```text
//! chữ gốc + dấu hình chữ + dấu thanh + kiểu hoa
//! ```
//!
//! Chỉ module render mới biết `ế` ứng với code point nào; module này
//! chỉ giữ cấu trúc domain, không phụ thuộc Unicode cụ thể.

/// Chữ gốc (vowel nền hoặc D đặc biệt).
///
/// `A`/`E`/`I`/`O`/`U`/`Y` là nguyên âm nền có thể nhận dấu hình chữ và
/// dấu thanh. `D` là phụ âm đặc biệt có thể thành `đ`. Các phụ âm khác
/// được giữ qua `PhuAm`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChuGoc {
    /// Nguyên âm a.
    A,
    /// Nguyên âm e.
    E,
    /// Nguyên âm i.
    I,
    /// Nguyên âm o.
    O,
    /// Nguyên âm u.
    U,
    /// Nguyên âm y.
    Y,
    /// Phụ âm d (có thể thành đ).
    D,
    /// Phụ âm khác, giữ nguyên ký tự gốc.
    PhuAm(char),
}

impl ChuGoc {
    /// Trả ký tự gốc thường tương ứng (không dấu, không biến đổi).
    pub(crate) fn ky_tu_thuong(self) -> char {
        match self {
            Self::A => 'a',
            Self::E => 'e',
            Self::I => 'i',
            Self::O => 'o',
            Self::U => 'u',
            Self::Y => 'y',
            Self::D => 'd',
            Self::PhuAm(c) => c.to_ascii_lowercase(),
        }
    }

    /// Trả `true` nếu đây là nguyên âm nền có thể nhận dấu hình chữ.
    pub(crate) fn la_nguyen_am(self) -> bool {
        matches!(
            self,
            Self::A | Self::E | Self::I | Self::O | Self::U | Self::Y
        )
    }
}

/// Dấu hình chữ (diacritic của chữ cái, không phải dấu thanh).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DauChu {
    /// Không có dấu hình chữ.
    Khong,
    /// Dấu trăng (breve): `a` → `ă`.
    Trang,
    /// Dấu mũ (circumflex): `a`→`â`, `e`→`ê`, `o`→`ô`.
    Mu,
    /// Dấu móc (horn): `o`→`ơ`, `u`→`ư`.
    Moc,
    /// Dấu gạch (stroke): `d`→`đ`.
    Gach,
}

/// Dấu thanh (tone).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DauThanh {
    /// Không dấu (ngang).
    Khong,
    /// Sắc.
    Sac,
    /// Huyền.
    Huyen,
    /// Hỏi.
    Hoi,
    /// Ngã.
    Nga,
    /// Nặng.
    Nang,
}

/// Kiểu hoa/thường của chữ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum KieuHoa {
    /// Chữ thường.
    Thuong,
    /// Chữ hoa.
    Hoa,
}

impl KieuHoa {
    /// Tạo từ một ký tự: hoa nếu `c` là ASCII uppercase, ngược lại thường.
    pub(crate) fn tu_ky_tu(c: char) -> Self {
        if c.is_ascii_uppercase() {
            Self::Hoa
        } else {
            Self::Thuong
        }
    }

    /// Áp kiểu hoa lên một ký tự thường.
    pub(crate) fn ap_dung(self, c: char) -> char {
        match self {
            Self::Thuong => c,
            Self::Hoa => c.to_ascii_uppercase(),
        }
    }
}

/// Một chữ cái Việt đầy đủ: chữ gốc + dấu hình chữ + dấu thanh + kiểu hoa.
///
/// Đây là đơn vị nhỏ nhất của kết quả biến đổi Telex. Provenance (thao tác
/// raw nào sinh ra đơn vị này) được giữ ở lớp [`DonViRender`](crate::telex)
/// chứ không nằm trong chính `ChuCaiViet`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ChuCaiViet {
    /// Chữ gốc.
    pub(crate) chu_goc: ChuGoc,
    /// Dấu hình chữ.
    pub(crate) dau_chu: DauChu,
    /// Dấu thanh.
    pub(crate) dau_thanh: DauThanh,
    /// Kiểu hoa.
    pub(crate) kieu_hoa: KieuHoa,
}

impl ChuCaiViet {
    /// Tạo chữ cái thường không dấu từ chữ gốc.
    pub(crate) fn thuong(ky_tu: char) -> Self {
        Self {
            chu_goc: chu_goc_tu_ky_tu(ky_tu),
            dau_chu: DauChu::Khong,
            dau_thanh: DauThanh::Khong,
            kieu_hoa: KieuHoa::tu_ky_tu(ky_tu),
        }
    }
}

/// Tạo `ChuGoc` từ một ký tự chữ cái Latin.
///
/// Ký tự có thể hoa hoặc thường; phân biệt kiểu hoa nằm ở `KieuHoa`.
pub(crate) fn chu_goc_tu_ky_tu(c: char) -> ChuGoc {
    match c.to_ascii_lowercase() {
        'a' => ChuGoc::A,
        'e' => ChuGoc::E,
        'i' => ChuGoc::I,
        'o' => ChuGoc::O,
        'u' => ChuGoc::U,
        'y' => ChuGoc::Y,
        'd' => ChuGoc::D,
        other => ChuGoc::PhuAm(other),
    }
}
