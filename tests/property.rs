// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Property test cho các bất biến nền tảng của phiên (Phase 2).
//!
//! Phase 2 thay đổi hai bất biến so với Phase 1: output có thể khác raw
//! (Telex biến đổi) và con trỏ di theo grapheme (không theo raw action).
//! Các property dưới đây kiểm tra các bất biến mới này.

use cadence::{BoGo, CauHinh, KetQuaXuLy, PhienGo};
use proptest::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

/// Một hành động có thể áp dụng lên phiên.
#[derive(Debug, Clone)]
enum HanhDong {
    Them(char),
    ThemNguyenBan(char),
    XoaLui,
    XoaPhiaTruoc,
    DiTrai,
    DiPhai,
    VeDau,
    VeCuoi,
    Reset,
    Commit,
}

/// Chiến lược sinh ký tự từ pool có nghĩa (ASCII, tiếng Việt, emoji).
fn ky_tu_co_nghia() -> impl Strategy<Value = char> {
    prop_oneof![
        Just('a'),
        Just('b'),
        Just('c'),
        Just('d'),
        Just('e'),
        Just('o'),
        Just('u'),
        Just('w'),
        Just('s'),
        Just('f'),
        Just('z'),
        Just('đ'),
        Just('ế'),
        Just('\u{0301}'),
        Just('😀'),
        Just(' '),
    ]
}

/// Chiến lược sinh một hành động.
fn hanh_dong() -> impl Strategy<Value = HanhDong> {
    prop_oneof![
        ky_tu_co_nghia().prop_map(HanhDong::Them),
        ky_tu_co_nghia().prop_map(HanhDong::ThemNguyenBan),
        Just(HanhDong::XoaLui),
        Just(HanhDong::XoaPhiaTruoc),
        Just(HanhDong::DiTrai),
        Just(HanhDong::DiPhai),
        Just(HanhDong::VeDau),
        Just(HanhDong::VeCuoi),
        Just(HanhDong::Reset),
        Just(HanhDong::Commit),
    ]
}

fn tao_phien(gioi_han: usize) -> PhienGo {
    let mut cau_hinh = CauHinh::mac_dinh();
    cau_hinh
        .dat_gioi_han_thao_tac(gioi_han)
        .expect("gioi han hop le");
    let bo_go = BoGo::new(cau_hinh).expect("cau hinh hop le");
    bo_go.tao_phien()
}

/// Áp dụng một hành động lên PhienGo.
fn ap_dung(phien: &mut PhienGo, hd: &HanhDong) {
    match hd {
        HanhDong::Them(c) => {
            phien.them_ky_tu(*c);
        }
        HanhDong::ThemNguyenBan(c) => {
            phien.them_nguyen_ban(*c);
        }
        HanhDong::XoaLui => {
            phien.xoa_lui();
        }
        HanhDong::XoaPhiaTruoc => {
            phien.xoa_phia_truoc();
        }
        HanhDong::DiTrai => {
            phien.di_trai();
        }
        HanhDong::DiPhai => {
            phien.di_phai();
        }
        HanhDong::VeDau => {
            phien.ve_dau();
        }
        HanhDong::VeCuoi => {
            phien.ve_cuoi();
        }
        HanhDong::Reset => {
            phien.dat_lai();
        }
        HanhDong::Commit => {
            let _ = phien.chap_nhan();
        }
    }
}

/// Áp dụng chuỗi hành động lên một phiên.
fn ap_dung_day(phien: &mut PhienGo, cac_hanh_dong: &[HanhDong]) {
    for hd in cac_hanh_dong {
        ap_dung(phien, hd);
    }
}

/// Kiểm tra các bất biến Phase 2 sau mỗi thao tác.
fn kiem_tra_bat_bien(phien: &PhienGo, gioi_han: usize) -> Result<(), TestCaseError> {
    let ban_chup = phien.ban_chup();
    let con_tro = ban_chup.con_tro();
    let noi_dung = ban_chup.noi_dung();
    // Con trỏ trong khoảng hợp lệ.
    prop_assert!(con_tro.chi_so_byte() <= noi_dung.len());
    let tong_utf16 = noi_dung.encode_utf16().count();
    prop_assert!(con_tro.chi_so_utf16() <= tong_utf16);
    let tong_grapheme = noi_dung.graphemes(true).count();
    prop_assert!(con_tro.chi_so_grapheme() <= tong_grapheme);
    // Byte index phải là ranh giới UTF-8 (không nằm giữa code point).
    prop_assert!(noi_dung.is_char_boundary(con_tro.chi_so_byte()));
    // Grapheme index phải là ranh giới grapheme (public cursor không nằm
    // giữa cluster).
    let cac_byte_grapheme: Vec<usize> = noi_dung.grapheme_indices(true).map(|(i, _)| i).collect();
    prop_assert!(
        cac_byte_grapheme.contains(&con_tro.chi_so_byte())
            || con_tro.chi_so_byte() == noi_dung.len()
    );
    // Số thao tác raw không vượt giới hạn.
    let so_thao_tac = ban_chup.noi_dung_goc().chars().count();
    prop_assert!(so_thao_tac <= gioi_han);
    Ok(())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    /// Bất biến: cursor hợp lệ + limit sau chuỗi hành động bất kỳ.
    #[test]
    fn bat_bien_sau_hanh_dong(tat_ca_hanh_dong in prop::collection::vec(hanh_dong(), 0..64)) {
        let gioi_han = 32;
        let mut phien = tao_phien(gioi_han);
        for hd in &tat_ca_hanh_dong {
            ap_dung(&mut phien, hd);
            kiem_tra_bat_bien(&phien, gioi_han)?;
        }
    }

    /// Bất biến: thêm ký tự rồi xóa lùi (ở cuối) trả về snapshot cũ.
    #[test]
    fn them_roi_xoa_lui_tra_ve_cu(tat_ca_hanh_dong in prop::collection::vec(hanh_dong(), 0..32),
                                   c in ky_tu_co_nghia()) {
        let mut phien = tao_phien(64);
        ap_dung_day(&mut phien, &tat_ca_hanh_dong);
        phien.ve_cuoi();
        let ban_chup_cu = phien.ban_chup().clone();

        phien.them_ky_tu(c);
        phien.xoa_lui();

        prop_assert_eq!(phien.ban_chup(), &ban_chup_cu);
    }

    /// Bất biến: reset luôn tạo snapshot rỗng.
    #[test]
    fn reset_luon_rong(tat_ca_hanh_dong in prop::collection::vec(hanh_dong(), 0..32)) {
        let mut phien = tao_phien(64);
        ap_dung_day(&mut phien, &tat_ca_hanh_dong);
        phien.dat_lai();
        let ban_chup = phien.ban_chup();
        prop_assert!(phien.dang_trong());
        prop_assert_eq!(ban_chup.noi_dung(), "");
        prop_assert_eq!(ban_chup.noi_dung_goc(), "");
        prop_assert_eq!(ban_chup.con_tro().chi_so_byte(), 0);
    }

    /// Bất biến: commit rồi reset không để state cũ.
    #[test]
    fn commit_roi_reset_sach(tat_ca_hanh_dong in prop::collection::vec(hanh_dong(), 1..32)) {
        let mut phien = tao_phien(64);
        phien.them_ky_tu('a');
        ap_dung_day(&mut phien, &tat_ca_hanh_dong);
        phien.ve_cuoi();
        let ban_chup_truoc = String::from(phien.ban_chup().noi_dung());
        let dang_trong_truoc = phien.dang_trong();
        match phien.chap_nhan() {
            KetQuaXuLy::ChapNhan { noi_dung } => {
                prop_assert!(!dang_trong_truoc);
                prop_assert_eq!(noi_dung, ban_chup_truoc);
            }
            KetQuaXuLy::KhongDoi => {
                prop_assert!(dang_trong_truoc);
            }
            KetQuaXuLy::CapNhat => panic!("chap_nhan khong tra CapNhat"),
        }
        phien.dat_lai();
        prop_assert!(phien.dang_trong());
        phien.them_ky_tu('z');
        prop_assert_eq!(phien.ban_chup().noi_dung(), "z");
    }

    /// Bất biến: hai phiên độc lập (replay cùng actions → identical).
    #[test]
    fn hai_phien_doc_lap(hanh_dong_a in prop::collection::vec(hanh_dong(), 0..32),
                          hanh_dong_b in prop::collection::vec(hanh_dong(), 0..32)) {
        let mut phien_a = tao_phien(64);
        let mut phien_b = tao_phien(64);
        ap_dung_day(&mut phien_a, &hanh_dong_a);
        ap_dung_day(&mut phien_b, &hanh_dong_b);
        // Replay determinism: phiên khác chạy cùng actions_a phải giống phien_a.
        let mut phien_a_lai = tao_phien(64);
        ap_dung_day(&mut phien_a_lai, &hanh_dong_a);
        prop_assert_eq!(phien_a.ban_chup(), phien_a_lai.ban_chup());
        // Tương tự cho phien_b.
        let mut phien_b_lai = tao_phien(64);
        ap_dung_day(&mut phien_b_lai, &hanh_dong_b);
        prop_assert_eq!(phien_b.ban_chup(), phien_b_lai.ban_chup());
    }

    /// Bất biến: mọi chuỗi Unicode không gây panic.
    #[test]
    fn khong_panic_voi_unicode_bat_ky(tat_ca_hanh_dong in prop::collection::vec(any::<char>(), 0..64)) {
        let mut phien = tao_phien(4096);
        for c in &tat_ca_hanh_dong {
            phien.them_ky_tu(*c);
        }
        let ban_chup = phien.ban_chup();
        prop_assert!(ban_chup.con_tro().chi_so_byte() <= ban_chup.noi_dung().len());
        prop_assert!(ban_chup.noi_dung().is_char_boundary(ban_chup.con_tro().chi_so_byte()));
    }

    /// Bất biến: them_nguyen_ban không kích hoạt Telex (output == raw).
    #[test]
    fn them_nguyen_ban_khong_bien_doi(cac_ky_tu in prop::collection::vec(ky_tu_co_nghia(), 0..32)) {
        let mut phien = tao_phien(4096);
        for c in &cac_ky_tu {
            phien.them_nguyen_ban(*c);
        }
        let ban_chup = phien.ban_chup();
        prop_assert_eq!(ban_chup.noi_dung(), ban_chup.noi_dung_goc());
    }

    /// Bất biến: NFC output canonical equivalent với NFD output của cùng raw.
    #[test]
    fn nfc_nfd_canonical_equivalent(cac_ky_tu in prop::collection::vec(ky_tu_co_nghia(), 0..32)) {
        use unicode_normalization::UnicodeNormalization;
        let mut cau_hinh_nfc = CauHinh::mac_dinh();
        cau_hinh_nfc.dat_gioi_han_thao_tac(4096).expect("ok");
        let mut cau_hinh_nfd = CauHinh::mac_dinh();
        cau_hinh_nfd.dat_dang_unicode(cadence::DangUnicode::Nfd);
        cau_hinh_nfd.dat_gioi_han_thao_tac(4096).expect("ok");
        let mut phien_nfc = BoGo::new(cau_hinh_nfc).expect("ok").tao_phien();
        let mut phien_nfd = BoGo::new(cau_hinh_nfd).expect("ok").tao_phien();
        for c in &cac_ky_tu {
            phien_nfc.them_ky_tu(*c);
            phien_nfd.them_ky_tu(*c);
        }
        let nfc = phien_nfc.ban_chup().noi_dung().to_string();
        let nfd = phien_nfd.ban_chup().noi_dung().to_string();
        prop_assert_eq!(nfc.nfd().collect::<String>(), nfd);
    }

    /// Bất biến: byte index luôn là char boundary sau di_trai/di_phai.
    #[test]
    fn byte_index_char_boundary_sau_di_chuyen(cac_hanh_dong in prop::collection::vec(hanh_dong(), 0..32)) {
        let mut phien = tao_phien(4096);
        ap_dung_day(&mut phien, &cac_hanh_dong);
        for _ in 0..10 {
            phien.di_trai();
            let bc = phien.ban_chup();
            prop_assert!(bc.noi_dung().is_char_boundary(bc.con_tro().chi_so_byte()));
            phien.di_phai();
            let bc = phien.ban_chup();
            prop_assert!(bc.noi_dung().is_char_boundary(bc.con_tro().chi_so_byte()));
        }
    }

    /// Bất biến: navigation không thay đổi nội dung (raw và rendered).
    #[test]
    fn navigation_khong_doi_noi_dung(cac_hanh_dong in prop::collection::vec(hanh_dong(), 0..32)) {
        let mut phien = tao_phien(4096);
        ap_dung_day(&mut phien, &cac_hanh_dong);
        let noi_dung = phien.ban_chup().noi_dung().to_string();
        let noi_dung_goc = phien.ban_chup().noi_dung_goc().to_string();
        // Navigation không thay đổi nội dung.
        for _ in 0..5 {
            phien.di_trai();
        }
        prop_assert_eq!(phien.ban_chup().noi_dung(), &noi_dung[..]);
        prop_assert_eq!(phien.ban_chup().noi_dung_goc(), &noi_dung_goc[..]);
        for _ in 0..5 {
            phien.di_phai();
        }
        prop_assert_eq!(phien.ban_chup().noi_dung(), &noi_dung[..]);
        prop_assert_eq!(phien.ban_chup().noi_dung_goc(), &noi_dung_goc[..]);
        phien.ve_dau();
        prop_assert_eq!(phien.ban_chup().noi_dung(), &noi_dung[..]);
        prop_assert_eq!(phien.ban_chup().noi_dung_goc(), &noi_dung_goc[..]);
        phien.ve_cuoi();
        prop_assert_eq!(phien.ban_chup().noi_dung(), &noi_dung[..]);
        prop_assert_eq!(phien.ban_chup().noi_dung_goc(), &noi_dung_goc[..]);
    }

    /// Bất biến: cursor round-trip — ve_dau rồi ve_cuoi về cuối; ve_cuoi rồi ve_dau về đầu.
    #[test]
    fn cursor_round_trip(cac_hanh_dong in prop::collection::vec(hanh_dong(), 0..32)) {
        let mut phien = tao_phien(4096);
        ap_dung_day(&mut phien, &cac_hanh_dong);
        phien.ve_cuoi();
        let g_cuoi = phien.ban_chup().con_tro().chi_so_grapheme();
        phien.ve_dau();
        let g_dau = phien.ban_chup().con_tro().chi_so_grapheme();
        prop_assert_eq!(g_dau, 0);
        // Ve_cuoi từ đầu → về cuối.
        phien.ve_cuoi();
        prop_assert_eq!(phien.ban_chup().con_tro().chi_so_grapheme(), g_cuoi);
        // Ve_dau từ cuối → về đầu.
        phien.ve_dau();
        prop_assert_eq!(phien.ban_chup().con_tro().chi_so_grapheme(), 0);
    }

    /// Bất biến: di_trai ở đầu luôn KhongDoi; di_phai ở cuối luôn KhongDoi.
    #[test]
    fn boundary_navigation_khong_doi(cac_hanh_dong in prop::collection::vec(hanh_dong(), 0..32)) {
        let mut phien = tao_phien(4096);
        ap_dung_day(&mut phien, &cac_hanh_dong);
        phien.ve_dau();
        prop_assert!(matches!(phien.di_trai(), KetQuaXuLy::KhongDoi));
        phien.ve_cuoi();
        prop_assert!(matches!(phien.di_phai(), KetQuaXuLy::KhongDoi));
    }

    /// Bất biến: loai_noi_dung không đổi dưới navigation (content không đổi).
    #[test]
    fn loai_noi_dung_on_dinh_navigation(cac_hanh_dong in prop::collection::vec(hanh_dong(), 0..32)) {
        let mut phien = tao_phien(4096);
        ap_dung_day(&mut phien, &cac_hanh_dong);
        let loai = phien.ban_chup().loai_noi_dung();
        phien.ve_dau();
        prop_assert_eq!(phien.ban_chup().loai_noi_dung(), loai);
        phien.ve_cuoi();
        prop_assert_eq!(phien.ban_chup().loai_noi_dung(), loai);
        for _ in 0..3 {
            phien.di_trai();
            prop_assert_eq!(phien.ban_chup().loai_noi_dung(), loai);
        }
    }

    /// Bất biến: chap_nhan trả nội dung đúng bằng ban_chup().noi_dung().
    #[test]
    fn chap_nhan_tra_noi_dung_dung(cac_hanh_dong in prop::collection::vec(hanh_dong(), 1..32)) {
        let mut phien = tao_phien(64);
        phien.them_ky_tu('a');
        ap_dung_day(&mut phien, &cac_hanh_dong);
        let noi_dung = phien.ban_chup().noi_dung().to_string();
        let kq = phien.chap_nhan();
        match kq {
            KetQuaXuLy::ChapNhan { noi_dung: n } => {
                prop_assert_eq!(n, noi_dung);
            }
            KetQuaXuLy::KhongDoi => {
                prop_assert!(noi_dung.is_empty());
            }
            KetQuaXuLy::CapNhat => panic!("chap_nhan khong tra CapNhat"),
        }
    }

    /// Bất biến: hai phiên cùng actions → cùng loai_noi_dung (determinism).
    #[test]
    fn hai_phien_cung_loai_noi_dung(cac_hanh_dong in prop::collection::vec(hanh_dong(), 0..32)) {
        let mut phien_a = tao_phien(64);
        let mut phien_b = tao_phien(64);
        ap_dung_day(&mut phien_a, &cac_hanh_dong);
        ap_dung_day(&mut phien_b, &cac_hanh_dong);
        prop_assert_eq!(
            phien_a.ban_chup().loai_noi_dung(),
            phien_b.ban_chup().loai_noi_dung()
        );
    }

    /// Bất biến: xoa_lui ở đầu luôn KhongDoi bất kể nội dung.
    #[test]
    fn xoa_lui_o_dau_khong_doi(cac_hanh_dong in prop::collection::vec(hanh_dong(), 0..32)) {
        let mut phien = tao_phien(4096);
        ap_dung_day(&mut phien, &cac_hanh_dong);
        phien.ve_dau();
        let noi_dung = phien.ban_chup().noi_dung().to_string();
        prop_assert!(matches!(phien.xoa_lui(), KetQuaXuLy::KhongDoi));
        prop_assert_eq!(phien.ban_chup().noi_dung(), &noi_dung[..]);
    }

    /// Bất biến: xoa_phia_truoc ở cuối luôn KhongDoi bất kể nội dung.
    #[test]
    fn xoa_phia_truoc_o_cuoi_khong_doi(cac_hanh_dong in prop::collection::vec(hanh_dong(), 0..32)) {
        let mut phien = tao_phien(4096);
        ap_dung_day(&mut phien, &cac_hanh_dong);
        phien.ve_cuoi();
        let noi_dung = phien.ban_chup().noi_dung().to_string();
        prop_assert!(matches!(phien.xoa_phia_truoc(), KetQuaXuLy::KhongDoi));
        prop_assert_eq!(phien.ban_chup().noi_dung(), &noi_dung[..]);
    }
}
