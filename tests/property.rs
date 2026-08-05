// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Property test cho các bất biến nền tảng của phiên.

use std::string::String as StdString;

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

/// Mô hình tham chiếu dùng `Vec<char>` và con trỏ, độc lập với PhienGo.
struct MoHinh {
    noi_dung: Vec<char>,
    con_tro: usize,
}

impl MoHinh {
    fn moi() -> Self {
        Self {
            noi_dung: Vec::new(),
            con_tro: 0,
        }
    }

    fn them(&mut self, c: char) {
        self.noi_dung.insert(self.con_tro, c);
        self.con_tro += 1;
    }

    fn xoa_lui(&mut self) {
        if self.con_tro > 0 {
            self.noi_dung.remove(self.con_tro - 1);
            self.con_tro -= 1;
        }
    }

    fn xoa_phia_truoc(&mut self) {
        if self.con_tro < self.noi_dung.len() {
            self.noi_dung.remove(self.con_tro);
        }
    }

    fn di_trai(&mut self) {
        if self.con_tro > 0 {
            self.con_tro -= 1;
        }
    }

    fn di_phai(&mut self) {
        if self.con_tro < self.noi_dung.len() {
            self.con_tro += 1;
        }
    }

    fn ve_dau(&mut self) {
        self.con_tro = 0;
    }

    fn ve_cuoi(&mut self) {
        self.con_tro = self.noi_dung.len();
    }

    fn reset(&mut self) {
        self.noi_dung.clear();
        self.con_tro = 0;
    }

    fn chuoi(&self) -> StdString {
        self.noi_dung.iter().collect()
    }

    /// Byte offset của con trỏ theo mô hình.
    fn byte_con_tro(&self) -> usize {
        self.noi_dung[..self.con_tro]
            .iter()
            .map(|c| c.len_utf8())
            .sum()
    }

    /// Số đơn vị UTF-16 của tiền tố trước con trỏ.
    fn utf16_con_tro(&self) -> usize {
        let mut buf = [0u16; 2];
        self.noi_dung[..self.con_tro]
            .iter()
            .map(|c| c.encode_utf16(&mut buf).len())
            .sum()
    }

    /// Số grapheme của tiền tố trước con trỏ.
    fn grapheme_con_tro(&self) -> usize {
        let tien_to: StdString = self.noi_dung[..self.con_tro].iter().collect();
        tien_to.graphemes(true).count()
    }
}

/// Chiến lược sinh ký tự từ một pool có nghĩa (ASCII, tiếng Việt, emoji).
fn ky_tu_co_nghia() -> impl Strategy<Value = char> {
    prop_oneof![
        Just('a'),
        Just('b'),
        Just('c'),
        Just('d'),
        Just('z'),
        Just('đ'),
        Just('ế'),
        Just('ố'),
        Just('ê'),
        Just('\u{0301}'),
        Just('\u{0302}'),
        Just('😀'),
        Just('\u{1F469}'),
        Just('\u{200D}'),
        Just('\u{FE0F}'),
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

/// Áp dụng một hành động lên cả PhienGo và mô hình tham chiếu.
/// Trả `Some(noi_dung)` nếu là commit thành công.
fn ap_dung(phien: &mut PhienGo, mo_hinh: &mut MoHinh, hd: &HanhDong) -> Option<StdString> {
    match hd {
        HanhDong::Them(c) => {
            phien.them_ky_tu(*c);
            mo_hinh.them(*c);
            None
        }
        HanhDong::ThemNguyenBan(c) => {
            phien.them_nguyen_ban(*c);
            mo_hinh.them(*c);
            None
        }
        HanhDong::XoaLui => {
            phien.xoa_lui();
            mo_hinh.xoa_lui();
            None
        }
        HanhDong::XoaPhiaTruoc => {
            phien.xoa_phia_truoc();
            mo_hinh.xoa_phia_truoc();
            None
        }
        HanhDong::DiTrai => {
            phien.di_trai();
            mo_hinh.di_trai();
            None
        }
        HanhDong::DiPhai => {
            phien.di_phai();
            mo_hinh.di_phai();
            None
        }
        HanhDong::VeDau => {
            phien.ve_dau();
            mo_hinh.ve_dau();
            None
        }
        HanhDong::VeCuoi => {
            phien.ve_cuoi();
            mo_hinh.ve_cuoi();
            None
        }
        HanhDong::Reset => {
            phien.dat_lai();
            mo_hinh.reset();
            None
        }
        HanhDong::Commit => {
            let ket_qua = phien.chap_nhan();
            let tra_ve = match ket_qua {
                KetQuaXuLy::ChapNhan { noi_dung } => Some(noi_dung),
                KetQuaXuLy::KhongDoi => None,
                KetQuaXuLy::CapNhat => None,
            };
            mo_hinh.reset();
            tra_ve
        }
    }
}

fn kiem_tra_bat_bien(phien: &PhienGo, mo_hinh: &MoHinh, gioi_han: usize) {
    let ban_chup = phien.ban_chup();
    // Bất biến 4: noi_dung_goc bằng chuỗi thao tác hiện còn.
    assert_eq!(ban_chup.noi_dung_goc(), mo_hinh.chuoi().as_str());
    // Phase 1: noi_dung == noi_dung_goc.
    assert_eq!(ban_chup.noi_dung(), mo_hinh.chuoi().as_str());

    let con_tro = ban_chup.con_tro();
    // Bất biến 9: byte/utf16 không vượt độ dài tương ứng.
    assert!(con_tro.chi_so_byte() <= ban_chup.noi_dung().len());
    let tong_utf16 = ban_chup.noi_dung().encode_utf16().count();
    assert!(con_tro.chi_so_utf16() <= tong_utf16);
    let tong_grapheme = ban_chup.noi_dung().graphemes(true).count();
    assert!(con_tro.chi_so_grapheme() <= tong_grapheme);

    // So khớp vị trí con trỏ với mô hình.
    assert_eq!(con_tro.chi_so_byte(), mo_hinh.byte_con_tro());
    assert_eq!(con_tro.chi_so_utf16(), mo_hinh.utf16_con_tro());
    assert_eq!(con_tro.chi_so_grapheme(), mo_hinh.grapheme_con_tro());

    // Byte index phải là ranh giới UTF-8.
    assert!(ban_chup
        .noi_dung()
        .is_char_boundary(con_tro.chi_so_byte()));

    // Bất biến 10: số thao tác không vượt giới hạn.
    let so_thao_tac = ban_chup.noi_dung_goc().chars().count();
    assert!(so_thao_tac <= gioi_han);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    /// Bất biến 1, 4, 8, 9, 10: chạy chuỗi hành động bất kỳ và so với mô hình.
    #[test]
    fn dong_bo_voi_mo_hinh(tat_ca_hanh_dong in prop::collection::vec(hanh_dong(), 0..64)) {
        let gioi_han = 32;
        let mut phien = tao_phien(gioi_han);
        let mut mo_hinh = MoHinh::moi();
        for hd in &tat_ca_hanh_dong {
            ap_dung(&mut phien, &mut mo_hinh, hd);
            kiem_tra_bat_bien(&phien, &mo_hinh, gioi_han);
        }
    }

    /// Bất biến 2: thêm ký tự rồi xóa lùi (ở cuối) trả về snapshot cũ.
    #[test]
    fn them_roi_xoa_lui_tra_ve_cu(tat_ca_hanh_dong in prop::collection::vec(hanh_dong(), 0..32),
                                   c in ky_tu_co_nghia()) {
        let mut phien = tao_phien(64);
        let mut mo_hinh = MoHinh::moi();
        for hd in &tat_ca_hanh_dong {
            ap_dung(&mut phien, &mut mo_hinh, hd);
        }
        phien.ve_cuoi();
        mo_hinh.ve_cuoi();
        let ban_chup_cu = phien.ban_chup().clone();

        phien.them_ky_tu(c);
        phien.xoa_lui();

        prop_assert_eq!(phien.ban_chup(), &ban_chup_cu);
    }

    /// Bất biến 3: chèn ký tự rồi xóa đúng ký tự vừa chèn trả về snapshot cũ.
    #[test]
    fn chen_roi_xoa_dung_ky_tu_tra_ve_cu(tat_ca_hanh_dong in prop::collection::vec(hanh_dong(), 0..32),
                                           c in ky_tu_co_nghia()) {
        let mut phien = tao_phien(64);
        let mut mo_hinh = MoHinh::moi();
        for hd in &tat_ca_hanh_dong {
            ap_dung(&mut phien, &mut mo_hinh, hd);
        }
        let ban_chup_cu = phien.ban_chup().clone();

        phien.them_ky_tu(c);
        phien.xoa_lui();

        prop_assert_eq!(phien.ban_chup(), &ban_chup_cu);
    }

    /// Bất biến 5: reset luôn tạo snapshot rỗng.
    #[test]
    fn reset_luon_rong(tat_ca_hanh_dong in prop::collection::vec(hanh_dong(), 0..32)) {
        let mut phien = tao_phien(64);
        for hd in &tat_ca_hanh_dong {
            let mut mo_hinh = MoHinh::moi();
            ap_dung(&mut phien, &mut mo_hinh, hd);
        }
        phien.dat_lai();
        let ban_chup = phien.ban_chup();
        prop_assert!(phien.dang_trong());
        prop_assert_eq!(ban_chup.noi_dung(), "");
        prop_assert_eq!(ban_chup.noi_dung_goc(), "");
        prop_assert_eq!(ban_chup.con_tro().chi_so_byte(), 0);
    }

    /// Bất biến 6: commit rồi reset không để state cũ.
    #[test]
    fn commit_roi_reset_sach(tat_ca_hanh_dong in prop::collection::vec(hanh_dong(), 1..32)) {
        let mut phien = tao_phien(64);
        let mut mo_hinh = MoHinh::moi();
        // Đảm bảo có nội dung để commit.
        phien.them_ky_tu('a');
        mo_hinh.them('a');
        for hd in &tat_ca_hanh_dong {
            ap_dung(&mut phien, &mut mo_hinh, hd);
        }
        phien.ve_cuoi();
        let ban_chup_truoc = phien.ban_chup().noi_dung().to_string();
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
        // Thêm ký tự mới: không được rò state cũ.
        phien.them_ky_tu('z');
        prop_assert_eq!(phien.ban_chup().noi_dung(), "z");
    }

    /// Bất biến 7: hai phiên nhận hai dòng thao tác không ảnh hưởng nhau.
    #[test]
    fn hai_phien_doc_lap(hanh_dong_a in prop::collection::vec(hanh_dong(), 0..32),
                          hanh_dong_b in prop::collection::vec(hanh_dong(), 0..32)) {
        let mut phien_a = tao_phien(64);
        let mut phien_b = tao_phien(64);
        let mut mo_hinh_a = MoHinh::moi();
        let mut mo_hinh_b = MoHinh::moi();
        for hd in &hanh_dong_a {
            ap_dung(&mut phien_a, &mut mo_hinh_a, hd);
        }
        for hd in &hanh_dong_b {
            ap_dung(&mut phien_b, &mut mo_hinh_b, hd);
        }
        prop_assert_eq!(phien_a.ban_chup().noi_dung(), mo_hinh_a.chuoi());
        prop_assert_eq!(phien_b.ban_chup().noi_dung(), mo_hinh_b.chuoi());
    }

    /// Bất biến 8: mọi chuỗi Unicode sinh đều không gây panic.
    #[test]
    fn khong_panic_voi_unicode_bat_ky(tat_ca_hanh_dong in prop::collection::vec(any::<char>(), 0..64)) {
        let mut phien = tao_phien(4096);
        for c in &tat_ca_hanh_dong {
            phien.them_ky_tu(*c);
        }
        // Chỉ cần không panic; snapshot phải nhất quán.
        let ban_chup = phien.ban_chup();
        prop_assert!(ban_chup.con_tro().chi_so_byte() <= ban_chup.noi_dung().len());
    }
}
