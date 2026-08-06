// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Render chữ Việt ra Unicode.
//!
//! Chỉ module này biết `ế` ứng với code point nào. Nhận `ChuCaiViet` và
//! xuất ra chuỗi theo dạng NFC (dựng sẵn) hoặc NFD (combining mark).

use alloc::string::String;
use alloc::string::ToString;

use super::chu_viet::{ChuCaiViet, ChuGoc, DauChu, DauThanh, KieuHoa};
use crate::cau_hinh::DangUnicode;

/// Trả ký tự NFC (dựng sẵn) của một nguyên âm Việt đầy đủ.
///
/// Chỉ áp dụng cho nguyên âm (`chu_goc` là `A`/`E`/`I`/`O`/`U`/`Y`).
/// Trả `None` nếu tổ hợp `dau_chu` không hợp lệ cho chữ gốc đó.
pub(crate) fn nguyen_am_nfc(chu_goc: ChuGoc, dau_chu: DauChu, dau_thanh: DauThanh) -> Option<char> {
    use ChuGoc::{A, E, I, O, U, Y};
    // Ánh xạ (chữ gốc, dấu chữ, dấu thanh) → ký tự dựng sẵn. Dùng match
    // đầy đủ theo từng nhóm nguyên âm để dễ audit.
    let co_so = match chu_goc {
        A => match (dau_chu, dau_thanh) {
            (DauChu::Khong, DauThanh::Khong) => 'a',
            (DauChu::Khong, DauThanh::Sac) => 'á',
            (DauChu::Khong, DauThanh::Huyen) => 'à',
            (DauChu::Khong, DauThanh::Hoi) => 'ả',
            (DauChu::Khong, DauThanh::Nga) => 'ã',
            (DauChu::Khong, DauThanh::Nang) => 'ạ',
            (DauChu::Trang, DauThanh::Khong) => 'ă',
            (DauChu::Trang, DauThanh::Sac) => 'ắ',
            (DauChu::Trang, DauThanh::Huyen) => 'ằ',
            (DauChu::Trang, DauThanh::Hoi) => 'ẳ',
            (DauChu::Trang, DauThanh::Nga) => 'ẵ',
            (DauChu::Trang, DauThanh::Nang) => 'ặ',
            (DauChu::Mu, DauThanh::Khong) => 'â',
            (DauChu::Mu, DauThanh::Sac) => 'ấ',
            (DauChu::Mu, DauThanh::Huyen) => 'ầ',
            (DauChu::Mu, DauThanh::Hoi) => 'ẩ',
            (DauChu::Mu, DauThanh::Nga) => 'ẫ',
            (DauChu::Mu, DauThanh::Nang) => 'ậ',
            _ => return None,
        },
        E => match (dau_chu, dau_thanh) {
            (DauChu::Khong, DauThanh::Khong) => 'e',
            (DauChu::Khong, DauThanh::Sac) => 'é',
            (DauChu::Khong, DauThanh::Huyen) => 'è',
            (DauChu::Khong, DauThanh::Hoi) => 'ẻ',
            (DauChu::Khong, DauThanh::Nga) => 'ẽ',
            (DauChu::Khong, DauThanh::Nang) => 'ẹ',
            (DauChu::Mu, DauThanh::Khong) => 'ê',
            (DauChu::Mu, DauThanh::Sac) => 'ế',
            (DauChu::Mu, DauThanh::Huyen) => 'ề',
            (DauChu::Mu, DauThanh::Hoi) => 'ể',
            (DauChu::Mu, DauThanh::Nga) => 'ễ',
            (DauChu::Mu, DauThanh::Nang) => 'ệ',
            _ => return None,
        },
        I => match (dau_chu, dau_thanh) {
            (DauChu::Khong, DauThanh::Khong) => 'i',
            (DauChu::Khong, DauThanh::Sac) => 'í',
            (DauChu::Khong, DauThanh::Huyen) => 'ì',
            (DauChu::Khong, DauThanh::Hoi) => 'ỉ',
            (DauChu::Khong, DauThanh::Nga) => 'ĩ',
            (DauChu::Khong, DauThanh::Nang) => 'ị',
            _ => return None,
        },
        O => match (dau_chu, dau_thanh) {
            (DauChu::Khong, DauThanh::Khong) => 'o',
            (DauChu::Khong, DauThanh::Sac) => 'ó',
            (DauChu::Khong, DauThanh::Huyen) => 'ò',
            (DauChu::Khong, DauThanh::Hoi) => 'ỏ',
            (DauChu::Khong, DauThanh::Nga) => 'õ',
            (DauChu::Khong, DauThanh::Nang) => 'ọ',
            (DauChu::Mu, DauThanh::Khong) => 'ô',
            (DauChu::Mu, DauThanh::Sac) => 'ố',
            (DauChu::Mu, DauThanh::Huyen) => 'ồ',
            (DauChu::Mu, DauThanh::Hoi) => 'ổ',
            (DauChu::Mu, DauThanh::Nga) => 'ỗ',
            (DauChu::Mu, DauThanh::Nang) => 'ộ',
            (DauChu::Moc, DauThanh::Khong) => 'ơ',
            (DauChu::Moc, DauThanh::Sac) => 'ớ',
            (DauChu::Moc, DauThanh::Huyen) => 'ờ',
            (DauChu::Moc, DauThanh::Hoi) => 'ở',
            (DauChu::Moc, DauThanh::Nga) => 'ỡ',
            (DauChu::Moc, DauThanh::Nang) => 'ợ',
            _ => return None,
        },
        U => match (dau_chu, dau_thanh) {
            (DauChu::Khong, DauThanh::Khong) => 'u',
            (DauChu::Khong, DauThanh::Sac) => 'ú',
            (DauChu::Khong, DauThanh::Huyen) => 'ù',
            (DauChu::Khong, DauThanh::Hoi) => 'ủ',
            (DauChu::Khong, DauThanh::Nga) => 'ũ',
            (DauChu::Khong, DauThanh::Nang) => 'ụ',
            (DauChu::Moc, DauThanh::Khong) => 'ư',
            (DauChu::Moc, DauThanh::Sac) => 'ứ',
            (DauChu::Moc, DauThanh::Huyen) => 'ừ',
            (DauChu::Moc, DauThanh::Hoi) => 'ử',
            (DauChu::Moc, DauThanh::Nga) => 'ữ',
            (DauChu::Moc, DauThanh::Nang) => 'ự',
            _ => return None,
        },
        Y => match (dau_chu, dau_thanh) {
            (DauChu::Khong, DauThanh::Khong) => 'y',
            (DauChu::Khong, DauThanh::Sac) => 'ý',
            (DauChu::Khong, DauThanh::Huyen) => 'ỳ',
            (DauChu::Khong, DauThanh::Hoi) => 'ỷ',
            (DauChu::Khong, DauThanh::Nga) => 'ỹ',
            (DauChu::Khong, DauThanh::Nang) => 'ỵ',
            _ => return None,
        },
        // Phụ âm không có tổ hợp nguyên âm dựng sẵn.
        _ => return None,
    };
    Some(co_so)
}

/// Render một `ChuCaiViet` ra chuỗi theo dạng Unicode đã chọn.
///
/// Phụ âm (`PhuAm`) và `D` không có tổ hợp dựng sẵn đặc biệt thì được render
/// bằng ký tự gốc, áp kiểu hoa. `đ` (D + Gach) render bằng 'đ'/'Đ'.
pub(crate) fn render_chu(chu: &ChuCaiViet, dang: DangUnicode) -> String {
    let ky_tu_thuong = if chu.chu_goc.la_nguyen_am() {
        match nguyen_am_nfc(chu.chu_goc, chu.dau_chu, chu.dau_thanh) {
            Some(c) => c,
            // Fallback: không nên xảy ra với input hợp lệ; dùng ký tự gốc thường.
            None => chu.chu_goc.ky_tu_thuong(),
        }
    } else {
        match chu.chu_goc {
            ChuGoc::D => {
                // đ không nhận dấu thanh; dấu gạch mới tạo đ.
                if matches!(chu.dau_chu, DauChu::Gach) {
                    'đ'
                } else {
                    'd'
                }
            }
            ChuGoc::PhuAm(c) => c,
            _ => chu.chu_goc.ky_tu_thuong(),
        }
    };
    let ky_tu = chu.kieu_hoa.ap_dung(ky_tu_thuong);
    // Chuẩn hóa dạng output: NFC giữ nguyên, NFD phân rã.
    match dang {
        DangUnicode::Nfc => ky_tu.to_string(),
        DangUnicode::Nfd => {
            use unicode_normalization::UnicodeNormalization;
            ky_tu.nfd().to_string()
        }
    }
}

/// Kiểm tra một ký tự Unicode dựng sẵn tiếng Việt có thể phân tích thành
/// `ChuCaiViet`. Dùng cho input đã có sẵn dấu.
///
/// Trả `None` nếu ký tự không phải chữ Việt dựng sẵn có dấu.
pub(crate) fn phan_tich_ky_tu(c: char) -> Option<ChuCaiViet> {
    let thuong = c.to_ascii_lowercase();
    let kieu_hoa = KieuHoa::tu_ky_tu(c);
    // Bảng tra ngược: ký tự dựng sẵn → (chữ gốc, dấu chữ, dấu thanh).
    let (chu_goc, dau_chu, dau_thanh) = match thuong {
        // a không dấu + tone
        'á' | 'à' | 'ả' | 'ã' | 'ạ' => (ChuGoc::A, DauChu::Khong, tu_dau_thanh(thuong)),
        // ă + tone
        'ă' | 'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' => {
            (ChuGoc::A, DauChu::Trang, tu_dau_thanh(thuong))
        }
        // â + tone
        'â' | 'ấ' | 'ầ' | 'ẩ' | 'ẫ' | 'ậ' => {
            (ChuGoc::A, DauChu::Mu, tu_dau_thanh(thuong))
        }
        // e
        'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' => (ChuGoc::E, DauChu::Khong, tu_dau_thanh(thuong)),
        // ê
        'ê' | 'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' => {
            (ChuGoc::E, DauChu::Mu, tu_dau_thanh(thuong))
        }
        // i
        'í' | 'ì' | 'ỉ' | 'ĩ' | 'ị' => (ChuGoc::I, DauChu::Khong, tu_dau_thanh(thuong)),
        // o
        'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' => (ChuGoc::O, DauChu::Khong, tu_dau_thanh(thuong)),
        // ô
        'ô' | 'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' => {
            (ChuGoc::O, DauChu::Mu, tu_dau_thanh(thuong))
        }
        // ơ
        'ơ' | 'ớ' | 'ờ' | 'ở' | 'ỡ' | 'ợ' => {
            (ChuGoc::O, DauChu::Moc, tu_dau_thanh(thuong))
        }
        // u
        'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' => (ChuGoc::U, DauChu::Khong, tu_dau_thanh(thuong)),
        // ư
        'ư' | 'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' => {
            (ChuGoc::U, DauChu::Moc, tu_dau_thanh(thuong))
        }
        // y
        'ý' | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' => (ChuGoc::Y, DauChu::Khong, tu_dau_thanh(thuong)),
        // đ
        'đ' => (ChuGoc::D, DauChu::Gach, DauThanh::Khong),
        _ => return None,
    };
    Some(ChuCaiViet {
        chu_goc,
        dau_chu,
        dau_thanh,
        kieu_hoa,
    })
}

/// Tra dấu thanh từ ký tự dựng sẵn có dấu (dùng trong `phan_tich_ky_tu`).
fn tu_dau_thanh(c: char) -> DauThanh {
    match c {
        'á' | 'ắ' | 'ấ' | 'é' | 'ế' | 'í' | 'ó' | 'ố' | 'ớ' | 'ú' | 'ứ' | 'ý' => {
            DauThanh::Sac
        }
        'à' | 'ằ' | 'ầ' | 'è' | 'ề' | 'ì' | 'ò' | 'ồ' | 'ờ' | 'ù' | 'ừ' | 'ỳ' => {
            DauThanh::Huyen
        }
        'ả' | 'ẳ' | 'ẩ' | 'ẻ' | 'ể' | 'ỉ' | 'ỏ' | 'ổ' | 'ở' | 'ủ' | 'ử' | 'ỷ' => {
            DauThanh::Hoi
        }
        'ã' | 'ẵ' | 'ẫ' | 'ẽ' | 'ễ' | 'ĩ' | 'õ' | 'ỗ' | 'ỡ' | 'ũ' | 'ữ' | 'ỹ' => {
            DauThanh::Nga
        }
        'ạ' | 'ặ' | 'ậ' | 'ẹ' | 'ệ' | 'ị' | 'ọ' | 'ộ' | 'ợ' | 'ụ' | 'ự' | 'ỵ' => {
            DauThanh::Nang
        }
        _ => DauThanh::Khong,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kieu_go::chu_viet::{ChuGoc, DauChu, DauThanh, KieuHoa};

    /// nguyen_am_nfc: mọi tổ hợp (vowel, Khong, Khong) → ký tự thường gốc.
    #[test]
    fn nguyen_am_nfc_khong_dau() {
        let cases = [
            (ChuGoc::A, 'a'),
            (ChuGoc::E, 'e'),
            (ChuGoc::I, 'i'),
            (ChuGoc::O, 'o'),
            (ChuGoc::U, 'u'),
            (ChuGoc::Y, 'y'),
        ];
        for (goc, exp) in cases {
            assert_eq!(
                nguyen_am_nfc(goc, DauChu::Khong, DauThanh::Khong),
                Some(exp),
                "{goc:?} khong dau"
            );
        }
    }

    /// nguyen_am_nfc: phụ âm trả None (không có tổ hợp dựng sẵn).
    #[test]
    fn nguyen_am_nfc_phu_am_tra_none() {
        assert_eq!(
            nguyen_am_nfc(ChuGoc::D, DauChu::Khong, DauThanh::Khong),
            None
        );
        assert_eq!(
            nguyen_am_nfc(ChuGoc::PhuAm('b'), DauChu::Khong, DauThanh::Khong),
            None
        );
    }

    /// nguyen_am_nfc: tổ hợp dấu không hợp lệ (vd I + Trang) trả None.
    #[test]
    fn nguyen_am_nfc_to_hop_sai_tra_none() {
        // `i` không nhận dấu trăng/mũ/móc.
        assert_eq!(
            nguyen_am_nfc(ChuGoc::I, DauChu::Trang, DauThanh::Khong),
            None
        );
        assert_eq!(nguyen_am_nfc(ChuGoc::I, DauChu::Mu, DauThanh::Khong), None);
        assert_eq!(nguyen_am_nfc(ChuGoc::I, DauChu::Moc, DauThanh::Khong), None);
        // `y` không nhận dấu hình chữ.
        assert_eq!(
            nguyen_am_nfc(ChuGoc::Y, DauChu::Trang, DauThanh::Khong),
            None
        );
    }

    /// phan_tich_ky_tu round-trip: ký tự dựng sẵn → ChuCaiViet → render → gốc.
    #[test]
    fn phan_tich_ky_tu_round_trip() {
        let chars = [
            'á', 'à', 'ả', 'ã', 'ạ', 'ă', 'ắ', 'ằ', 'ẳ', 'ẵ', 'ặ', 'â', 'ấ', 'ầ', 'ẩ', 'ẫ', 'ậ',
            'é', 'è', 'ẻ', 'ẽ', 'ẹ', 'ê', 'ế', 'ề', 'ể', 'ễ', 'ệ', 'í', 'ì', 'ỉ', 'ĩ', 'ị', 'ó',
            'ò', 'ỏ', 'õ', 'ọ', 'ô', 'ố', 'ồ', 'ổ', 'ỗ', 'ộ', 'ơ', 'ớ', 'ờ', 'ở', 'ỡ', 'ợ', 'ú',
            'ù', 'ủ', 'ũ', 'ụ', 'ư', 'ứ', 'ừ', 'ử', 'ữ', 'ự', 'ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ', 'đ',
        ];
        for &c in &chars {
            let chu = phan_tich_ky_tu(c).unwrap_or_else(|| panic!("phan_tich {c} tra None"));
            let rendered = render_chu(&chu, DangUnicode::Nfc);
            assert_eq!(rendered.chars().count(), 1, "{c} render khong phai 1 char");
            assert_eq!(rendered.chars().next(), Some(c), "{c} round-trip");
        }
    }

    /// phan_tich_ky_tu: ký tự không phải chữ Việt → None.
    #[test]
    fn phan_tich_ky_tu_khong_phai_chu_viet() {
        assert_eq!(phan_tich_ky_tu('b'), None);
        assert_eq!(phan_tich_ky_tu('c'), None);
        assert_eq!(phan_tich_ky_tu('1'), None);
        assert_eq!(phan_tich_ky_tu('.'), None);
        assert_eq!(phan_tich_ky_tu('😀'), None);
    }

    /// phan_tich_ky_tu: ký tự Việt THƯỜNG dựng sẵn giữ kiểu thường.
    ///
    /// Lưu ý (giới hạn 0.1.0): ký tự Việt HOA dựng sẵn (vd 'Ế') không được
    /// `phan_tich_ky_tu` nhận diện vì `to_ascii_lowercase` không đổi non-ASCII.
    /// Khi gõ trực tiếp, ký tự đó được giữ raw (an toàn), không parse. Xem
    /// `docs/PHASE4_REPORT.md` known limitations.
    #[test]
    fn phan_tich_ky_tu_giu_kieu_thuong() {
        let chu = phan_tich_ky_tu('ế').expect("e mu sac thuong");
        assert_eq!(chu.kieu_hoa, KieuHoa::Thuong);
        assert_eq!(chu.chu_goc, ChuGoc::E);
        assert_eq!(chu.dau_chu, DauChu::Mu);
        assert_eq!(chu.dau_thanh, DauThanh::Sac);
        // Ký tự Việt HOA dựng sẵn không nhận diện (giới hạn 0.1.0).
        assert_eq!(phan_tich_ky_tu('Ế'), None);
        // Nhưng 'đ' hoa 'Đ' cũng không nhận diện (cùng lý do).
        assert_eq!(phan_tich_ky_tu('Đ'), None);
    }

    /// tu_dau_thanh: mỗi ký tự tone → đúng DauThanh; ký tự không tone → Khong.
    #[test]
    fn tu_dau_thanh_dung() {
        assert_eq!(tu_dau_thanh('á'), DauThanh::Sac);
        assert_eq!(tu_dau_thanh('à'), DauThanh::Huyen);
        assert_eq!(tu_dau_thanh('ả'), DauThanh::Hoi);
        assert_eq!(tu_dau_thanh('ã'), DauThanh::Nga);
        assert_eq!(tu_dau_thanh('ạ'), DauThanh::Nang);
        // Ký tự không có tone → Khong.
        assert_eq!(tu_dau_thanh('a'), DauThanh::Khong);
        assert_eq!(tu_dau_thanh('x'), DauThanh::Khong);
    }

    /// render_chu NFD: tổ hợp có dấu phân rã thành base + combining marks.
    #[test]
    fn render_chu_nfd_phan_ra() {
        let chu = ChuCaiViet {
            chu_goc: ChuGoc::A,
            dau_chu: DauChu::Trang,
            dau_thanh: DauThanh::Sac,
            kieu_hoa: KieuHoa::Thuong,
        };
        let nfd = render_chu(&chu, DangUnicode::Nfd);
        // `ắ` NFD = a + U+0306 (breve) + U+0301 (acute).
        assert_eq!(nfd, "a\u{0306}\u{0301}");
    }

    /// render_chu NFC: `đ` không phân rã (đ không có decomposition).
    #[test]
    fn render_chu_nfc_dd_khong_phan_ra() {
        let chu = ChuCaiViet {
            chu_goc: ChuGoc::D,
            dau_chu: DauChu::Gach,
            dau_thanh: DauThanh::Khong,
            kieu_hoa: KieuHoa::Thuong,
        };
        assert_eq!(render_chu(&chu, DangUnicode::Nfc), "đ");
        assert_eq!(render_chu(&chu, DangUnicode::Nfd), "đ");
    }
}
