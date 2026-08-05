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

    /// Áp kiểu hoa lên một ký tự thường (dùng Unicode uppercase để hỗ trợ đ→Đ).
    pub(crate) fn ap_dung(self, c: char) -> char {
        match self {
            Self::Thuong => c,
            // Vietnamese uppercase luôn là 1 ký tự; fallback an toàn nếu
            // `to_uppercase` rỗng (không xảy ra với char hợp lệ).
            Self::Hoa => c.to_uppercase().next().unwrap_or(c),
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

#[cfg(test)]
mod tests {
    use super::*;

    /// chu_goc_tu_ky_tu: mỗi nguyên âm (thường + hoa) → đúng ChuGoc.
    #[test]
    fn chu_goc_tu_ky_tu_nguyen_am() {
        let cases = [
            ('a', ChuGoc::A),
            ('A', ChuGoc::A),
            ('e', ChuGoc::E),
            ('E', ChuGoc::E),
            ('i', ChuGoc::I),
            ('I', ChuGoc::I),
            ('o', ChuGoc::O),
            ('O', ChuGoc::O),
            ('u', ChuGoc::U),
            ('U', ChuGoc::U),
            ('y', ChuGoc::Y),
            ('Y', ChuGoc::Y),
            ('d', ChuGoc::D),
            ('D', ChuGoc::D),
        ];
        for (c, exp) in cases {
            assert_eq!(chu_goc_tu_ky_tu(c), exp, "chu_goc_tu_ky_tu({c:?}) sai");
        }
    }

    /// chu_goc_tu_ky_tu: phụ âm → PhuAm giữ ký tự thường.
    #[test]
    fn chu_goc_tu_ky_tu_phu_am() {
        assert_eq!(chu_goc_tu_ky_tu('b'), ChuGoc::PhuAm('b'));
        assert_eq!(chu_goc_tu_ky_tu('B'), ChuGoc::PhuAm('b'));
        assert_eq!(chu_goc_tu_ky_tu('c'), ChuGoc::PhuAm('c'));
        assert_eq!(chu_goc_tu_ky_tu('z'), ChuGoc::PhuAm('z'));
    }

    /// ChuGoc::la_nguyen_am: true cho 6 nguyên âm, false cho D và PhuAm.
    #[test]
    fn chu_goc_la_nguyen_am() {
        assert!(ChuGoc::A.la_nguyen_am());
        assert!(ChuGoc::E.la_nguyen_am());
        assert!(ChuGoc::I.la_nguyen_am());
        assert!(ChuGoc::O.la_nguyen_am());
        assert!(ChuGoc::U.la_nguyen_am());
        assert!(ChuGoc::Y.la_nguyen_am());
        assert!(!ChuGoc::D.la_nguyen_am());
        assert!(!ChuGoc::PhuAm('b').la_nguyen_am());
    }

    /// ChuGoc::ky_tu_thuong: trả ký tự thường gốc.
    #[test]
    fn chu_goc_ky_tu_thuong() {
        assert_eq!(ChuGoc::A.ky_tu_thuong(), 'a');
        assert_eq!(ChuGoc::E.ky_tu_thuong(), 'e');
        assert_eq!(ChuGoc::I.ky_tu_thuong(), 'i');
        assert_eq!(ChuGoc::O.ky_tu_thuong(), 'o');
        assert_eq!(ChuGoc::U.ky_tu_thuong(), 'u');
        assert_eq!(ChuGoc::Y.ky_tu_thuong(), 'y');
        assert_eq!(ChuGoc::D.ky_tu_thuong(), 'd');
        assert_eq!(ChuGoc::PhuAm('B').ky_tu_thuong(), 'b');
        assert_eq!(ChuGoc::PhuAm('n').ky_tu_thuong(), 'n');
    }

    /// KieuHoa::tu_ky_tu: ASCII hoa → Hoa, thường → Thuong.
    #[test]
    fn kieu_hoa_tu_ky_tu() {
        assert_eq!(KieuHoa::tu_ky_tu('A'), KieuHoa::Hoa);
        assert_eq!(KieuHoa::tu_ky_tu('Z'), KieuHoa::Hoa);
        assert_eq!(KieuHoa::tu_ky_tu('a'), KieuHoa::Thuong);
        assert_eq!(KieuHoa::tu_ky_tu('z'), KieuHoa::Thuong);
        // Non-ASCII không phải ASCII uppercase → Thuong.
        assert_eq!(KieuHoa::tu_ky_tu('đ'), KieuHoa::Thuong);
    }

    /// KieuHoa::ap_dung: Thuong giữ nguyên, Hoa uppercase.
    #[test]
    fn kieu_hoa_ap_dung() {
        assert_eq!(KieuHoa::Thuong.ap_dung('a'), 'a');
        assert_eq!(KieuHoa::Thuong.ap_dung('Đ'), 'Đ');
        assert_eq!(KieuHoa::Hoa.ap_dung('a'), 'A');
        assert_eq!(KieuHoa::Hoa.ap_dung('e'), 'E');
        // đ → Đ (Unicode uppercase hỗ trợ Vietnamese).
        assert_eq!(KieuHoa::Hoa.ap_dung('đ'), 'Đ');
    }

    /// ChuCaiViet::thuong: tạo chữ thường không dấu.
    #[test]
    fn chu_cai_viet_thuong() {
        let chu = ChuCaiViet::thuong('a');
        assert_eq!(chu.chu_goc, ChuGoc::A);
        assert_eq!(chu.dau_chu, DauChu::Khong);
        assert_eq!(chu.dau_thanh, DauThanh::Khong);
        assert_eq!(chu.kieu_hoa, KieuHoa::Thuong);

        let chu_hoa = ChuCaiViet::thuong('A');
        assert_eq!(chu_hoa.kieu_hoa, KieuHoa::Hoa);
        assert_eq!(chu_hoa.chu_goc, ChuGoc::A);

        // Phụ âm.
        let chu_ph = ChuCaiViet::thuong('b');
        assert_eq!(chu_ph.chu_goc, ChuGoc::PhuAm('b'));
    }

    /// DauChu có 5 biến thể, DauThanh có 6 biến thể — kiểm tra tất cả phân biệt.
    #[test]
    fn dau_chu_va_dau_thanh_phan_biet() {
        let dau_chu = [
            DauChu::Khong,
            DauChu::Trang,
            DauChu::Mu,
            DauChu::Moc,
            DauChu::Gach,
        ];
        for (i, &a) in dau_chu.iter().enumerate() {
            for &b in dau_chu.iter().skip(i + 1) {
                assert_ne!(a, b, "DauChu trùng");
            }
        }

        let dau_thanh = [
            DauThanh::Khong,
            DauThanh::Sac,
            DauThanh::Huyen,
            DauThanh::Hoi,
            DauThanh::Nga,
            DauThanh::Nang,
        ];
        for (i, &a) in dau_thanh.iter().enumerate() {
            for &b in dau_thanh.iter().skip(i + 1) {
                assert_ne!(a, b, "DauThanh trùng");
            }
        }
    }
}
