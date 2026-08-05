// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Soak test - kiểm tra chịu tải với chuỗi đầu vào dài và đa dạng.
//!
//! Mục tiêu: đảm bảo engine không panic, không treo, và giữ bất biến
//! nền tảng dưới tải nặng (nhiều thao tác, nhiều loại ký tự, nhiều
//! cấu hình). Khác với property test (ngẫu nhiên, proptest), soak test
//! dùng chuỗi xác định dài để có coverage lặp lại.

use cadence::{BoGo, CauHinh, ChinhSachLuaChon, DangUnicode, KetQuaXuLy, KieuTelex, QuyTacDatDau};
use unicode_segmentation::UnicodeSegmentation;

fn phien() -> cadence::PhienGo {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    bo_go.tao_phien()
}

fn phien_voi(cau_hinh: CauHinh) -> cadence::PhienGo {
    let bo_go = BoGo::new(cau_hinh).expect("cau hinh hop le");
    bo_go.tao_phien()
}

/// Kiểm tra bất biến cursor sau mỗi thao tác.
fn kiem_tra_bat_bien(phien: &cadence::PhienGo) {
    let bc = phien.ban_chup();
    let noi_dung = bc.noi_dung();
    let con_tro = bc.con_tro();
    assert!(
        con_tro.chi_so_byte() <= noi_dung.len(),
        "byte {} > len {}",
        con_tro.chi_so_byte(),
        noi_dung.len()
    );
    assert!(
        noi_dung.is_char_boundary(con_tro.chi_so_byte()),
        "byte {} khong phai char boundary",
        con_tro.chi_so_byte()
    );
    let tong_grapheme = noi_dung.graphemes(true).count();
    assert!(
        con_tro.chi_so_grapheme() <= tong_grapheme,
        "grapheme {} > total {}",
        con_tro.chi_so_grapheme(),
        tong_grapheme
    );
}

/// Soak: 1000 ký tự Telex ngẫu nhiên — không panic, cursor hợp lệ.
#[test]
fn soak_1000_ky_tu_telex() {
    let mut p = phien();
    let pool = "abcdefghijklmnopqrstuvwxyzswfxrjz ";
    for (i, c) in pool.chars().cycle().take(1000).enumerate() {
        p.them_ky_tu(c);
        if i % 100 == 0 {
            kiem_tra_bat_bien(&p);
        }
    }
    kiem_tra_bat_bien(&p);
    assert!(!p.dang_trong());
}

/// Soak: xen kẽ them_ky_tu và them_nguyen_ban — không panic.
#[test]
fn soak_xen_ke_nguyen_ban() {
    let mut c = CauHinh::mac_dinh();
    c.dat_gioi_han_thao_tac(4096).expect("hop le");
    let mut p = phien_voi(c);
    let pool = "aewsfd";
    for (i, c) in pool.chars().cycle().take(500).enumerate() {
        if i % 3 == 0 {
            p.them_nguyen_ban(c);
        } else {
            p.them_ky_tu(c);
        }
    }
    kiem_tra_bat_bien(&p);
    // Raw phải giữ chính xác.
    let raw: String = pool.chars().cycle().take(500).collect();
    assert_eq!(p.ban_chup().noi_dung_goc(), &raw[..]);
}

/// Soak: navigation liên tục qua chuỗi dài — không treo, đạt boundary.
#[test]
fn soak_navigation_lien_tuc() {
    let mut p = phien();
    for c in "tieengs nguowif dduwowngf".chars() {
        p.them_ky_tu(c);
    }
    // Di trái 200 lần — phải đạt đầu và KhongDoi.
    let mut khong_doi_count = 0;
    for _ in 0..200 {
        if matches!(p.di_trai(), KetQuaXuLy::KhongDoi) {
            khong_doi_count += 1;
        }
    }
    assert!(khong_doi_count > 0, "phai dat dau va KhongDoi");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 0);

    // Di phải 200 lần — phải đạt cuối.
    khong_doi_count = 0;
    for _ in 0..200 {
        if matches!(p.di_phai(), KetQuaXuLy::KhongDoi) {
            khong_doi_count += 1;
        }
    }
    assert!(khong_doi_count > 0, "phai dat cuoi va KhongDoi");
}

/// Soak: chèn/xóa lùi lặp lại — không panic, cursor hợp lệ.
#[test]
fn soak_chen_xoa_lap() {
    let mut p = phien();
    for i in 0..200 {
        let c = char::from_digit((i % 26) + 10, 36).unwrap_or('a');
        p.them_ky_tu(c);
        if i % 5 == 0 {
            p.xoa_lui();
        }
        if i % 7 == 0 {
            p.ve_dau();
            p.di_phai();
        }
    }
    kiem_tra_bat_bien(&p);
}

/// Soak: mọi tổ hợp cấu hình — engine ổn định.
#[test]
fn soak_moi_cau_hinh() {
    let input = "tieengs https://x.com brooooo :D";
    for kieu in [KieuTelex::CanBang, KieuTelex::DayDu] {
        for quy_tac in [QuyTacDatDau::HienDai, QuyTacDatDau::TruyenThong] {
            for dang in [DangUnicode::Nfc, DangUnicode::Nfd] {
                for cs in [
                    ChinhSachLuaChon::TuNhien,
                    ChinhSachLuaChon::UuTienTiengViet,
                    ChinhSachLuaChon::UuTienNguyenBan,
                ] {
                    let mut c = CauHinh::mac_dinh();
                    c.dat_kieu_telex(kieu);
                    c.dat_quy_tac_dat_dau(quy_tac);
                    c.dat_dang_unicode(dang);
                    c.dat_chinh_sach_lua_chon(cs);
                    let mut p = phien_voi(c);
                    for ch in input.chars() {
                        p.them_ky_tu(ch);
                    }
                    kiem_tra_bat_bien(&p);
                }
            }
        }
    }
}

/// Soak: commit/reset lặp 100 lần — không rò state.
#[test]
fn soak_commit_reset_lap() {
    let mut p = phien();
    for i in 0..100 {
        p.them_ky_tu('a');
        p.them_ky_tu('s');
        let _ = p.chap_nhan();
        assert!(p.dang_trong());
        p.dat_lai();
        assert!(p.dang_trong());
        let _ = i;
    }
}

/// Soak: emoji + combining mark + Telex trộn — không panic, grapheme đúng.
#[test]
fn soak_tron_emoji_combining_telex() {
    let mut p = phien();
    let input = [
        't',
        'i',
        'e',
        'e',
        'n',
        'g',
        's',
        ' ',
        '\u{1F600}',
        '\u{1F3FB}',
        ' ',
        'a',
        'w',
        's',
        'e',
        '\u{0301}',
        ' ',
        '\u{1F468}',
        '\u{200D}',
        '\u{1F469}',
    ];
    for c in input {
        p.them_ky_tu(c);
    }
    let bc = p.ban_chup();
    kiem_tra_bat_bien(&p);
    assert!(bc.noi_dung().is_char_boundary(bc.con_tro().chi_so_byte()));
    // Raw giữ byte-for-byte.
    let raw: String = input.iter().collect();
    assert_eq!(bc.noi_dung_goc(), &raw[..]);
}

/// Soak: giới hạn thao tác thấp (10) — không vượt, không panic.
#[test]
fn soak_gioi_han_thap() {
    let mut c = CauHinh::mac_dinh();
    c.dat_gioi_han_thao_tac(10).expect("hop le");
    let mut p = phien_voi(c);
    for i in 0..50 {
        let c = char::from_digit((i % 26) + 10, 36).unwrap_or('a');
        p.them_ky_tu(c);
    }
    // Raw không vượt 10.
    assert!(p.ban_chup().noi_dung_goc().chars().count() <= 10);
    kiem_tra_bat_bien(&p);
}

/// Soak: xoa_lui từ cuối đến khi rỗng — không underflow.
#[test]
fn soak_di_trai_xoa_lui_den_rong() {
    let mut p = phien();
    for c in "tieengs".chars() {
        p.them_ky_tu(c);
    }
    // Xóa lùi từ cuối cho đến khi rỗng.
    p.ve_cuoi();
    loop {
        if matches!(p.xoa_lui(), KetQuaXuLy::KhongDoi) {
            break;
        }
    }
    assert!(p.dang_trong());
    // Xóa lùi thêm nữa vẫn không panic.
    for _ in 0..10 {
        assert!(matches!(p.xoa_lui(), KetQuaXuLy::KhongDoi));
    }
    assert!(p.dang_trong());
}

/// Soak: di_phai/xoa_phia_truoc xen kẽ đến khi rỗng — không panic.
#[test]
fn soak_xoa_phia_truoc_den_rong() {
    let mut p = phien();
    for c in "tieengs".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    loop {
        if matches!(p.xoa_phia_truoc(), KetQuaXuLy::KhongDoi) {
            break;
        }
    }
    assert!(p.dang_trong());
    // Xóa phía trước thêm nữa vẫn không panic.
    for _ in 0..10 {
        assert!(matches!(p.xoa_phia_truoc(), KetQuaXuLy::KhongDoi));
    }
    assert!(p.dang_trong());
}
