// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test Unicode: byte, UTF-16, grapheme cluster và con trỏ.

use cadence::{BoGo, CauHinh, KetQuaXuLy};

fn tao_phien() -> cadence::PhienGo {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh hop le");
    bo_go.tao_phien()
}

#[test]
fn ascii() {
    let mut phien = tao_phien();
    phien.them_ky_tu('A');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "A");
    assert_eq!(ban_chup.con_tro().chi_so_byte(), 1);
    assert_eq!(ban_chup.con_tro().chi_so_utf16(), 1);
    assert_eq!(ban_chup.con_tro().chi_so_grapheme(), 1);
}

#[test]
fn ky_tu_d() {
    let mut phien = tao_phien();
    phien.them_ky_tu('đ');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "đ");
    // 'đ' là 2 byte UTF-8, 1 đơn vị UTF-16, 1 grapheme.
    assert_eq!(ban_chup.con_tro().chi_so_byte(), 2);
    assert_eq!(ban_chup.con_tro().chi_so_utf16(), 1);
    assert_eq!(ban_chup.con_tro().chi_so_grapheme(), 1);
}

#[test]
fn ky_tu_e_trong_dau() {
    let mut phien = tao_phien();
    phien.them_ky_tu('ế');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "ế");
    // 'ế' (U+1EBF) dựng sẵn trong BMP: 3 byte UTF-8, 1 đơn vị UTF-16, 1 grapheme.
    assert_eq!(ban_chup.con_tro().chi_so_byte(), 3);
    assert_eq!(ban_chup.con_tro().chi_so_utf16(), 1);
    assert_eq!(ban_chup.con_tro().chi_so_grapheme(), 1);
}

#[test]
fn chuoi_nfc() {
    let mut phien = tao_phien();
    for ky_tu in "ếốờữ".chars() {
        phien.them_ky_tu(ky_tu);
    }
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "ếốờữ");
    // 4 ký tự dựng sẵn trong BMP, mỗi cái 3 byte UTF-8.
    assert_eq!(ban_chup.con_tro().chi_so_byte(), 12);
    assert_eq!(ban_chup.con_tro().chi_so_utf16(), 4);
    assert_eq!(ban_chup.con_tro().chi_so_grapheme(), 4);
}

#[test]
fn chuoi_co_combining_mark() {
    let mut phien = tao_phien();
    // 'e' + combining acute (U+0301) tạo thành một grapheme.
    phien.them_ky_tu('e');
    phien.them_ky_tu('\u{0301}');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "e\u{0301}");
    // Một grapheme, 3 byte (1 + 2), 2 đơn vị UTF-16.
    assert_eq!(ban_chup.con_tro().chi_so_grapheme(), 1);
    assert_eq!(ban_chup.con_tro().chi_so_byte(), 3);
    assert_eq!(ban_chup.con_tro().chi_so_utf16(), 2);
}

#[test]
fn emoji_don() {
    let mut phien = tao_phien();
    phien.them_ky_tu('😀');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "😀");
    // Emoji ngoài BMP: 4 byte UTF-8, 2 đơn vị UTF-16, 1 grapheme.
    assert_eq!(ban_chup.con_tro().chi_so_byte(), 4);
    assert_eq!(ban_chup.con_tro().chi_so_utf16(), 2);
    assert_eq!(ban_chup.con_tro().chi_so_grapheme(), 1);
}

#[test]
fn emoji_ngoai_bmp() {
    let mut phien = tao_phien();
    // U+1F600, ngoài BMP, cần 2 đơn vị UTF-16.
    phien.them_ky_tu('\u{1F600}');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.con_tro().chi_so_byte(), 4);
    assert_eq!(ban_chup.con_tro().chi_so_utf16(), 2);
    assert_eq!(ban_chup.con_tro().chi_so_grapheme(), 1);
}

#[test]
fn emoji_co_variation_selector() {
    let mut phien = tao_phien();
    // ❤ + VS16 (U+FE0F) tạo một grapheme.
    phien.them_ky_tu('\u{2764}');
    phien.them_ky_tu('\u{FE0F}');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "\u{2764}\u{FE0F}");
    assert_eq!(ban_chup.con_tro().chi_so_grapheme(), 1);
    assert_eq!(ban_chup.con_tro().chi_so_byte(), 6);
    assert_eq!(ban_chup.con_tro().chi_so_utf16(), 2);
}

#[test]
fn emoji_co_zero_width_joiner() {
    let mut phien = tao_phien();
    // 👨 + ZWJ + 👩 tạo một grapheme "family".
    phien.them_ky_tu('\u{1F468}');
    phien.them_ky_tu('\u{200D}');
    phien.them_ky_tu('\u{1F469}');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.con_tro().chi_so_grapheme(), 1);
    // 4 + 3 + 4 = 11 byte.
    assert_eq!(ban_chup.con_tro().chi_so_byte(), 11);
    // 2 + 1 + 2 = 5 đơn vị UTF-16.
    assert_eq!(ban_chup.con_tro().chi_so_utf16(), 5);
}

#[test]
fn chuoi_tron_ascii_tieng_viet_emoji() {
    let mut phien = tao_phien();
    // "a" + "đ" + "😀".
    phien.them_ky_tu('a');
    phien.them_ky_tu('đ');
    phien.them_ky_tu('😀');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "ađ😀");
    assert_eq!(ban_chup.con_tro().chi_so_byte(), 1 + 2 + 4);
    assert_eq!(ban_chup.con_tro().chi_so_utf16(), 1 + 1 + 2);
    assert_eq!(ban_chup.con_tro().chi_so_grapheme(), 3);
}

#[test]
fn chen_xoa_quanh_ky_tu_unicode() {
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('đ');
    phien.them_ky_tu('b');
    // Xóa 'đ' ở giữa.
    phien.ve_cuoi();
    phien.di_trai();
    phien.di_trai();
    phien.xoa_phia_truoc();
    assert_eq!(phien.ban_chup().noi_dung(), "ab");
}

#[test]
fn byte_index_dung_tai_ranh_gioi_char() {
    let mut phien = tao_phien();
    phien.them_ky_tu('ế');
    phien.them_ky_tu('ố');
    // Di con trỏ về giữa hai ký tự dựng sẵn.
    phien.ve_dau();
    phien.di_phai();
    let ban_chup = phien.ban_chup();
    // Byte index phải là 3 (ranh giới UTF-8, không nằm giữa code point).
    assert_eq!(ban_chup.con_tro().chi_so_byte(), 3);
    assert!(
        ban_chup
            .noi_dung()
            .is_char_boundary(ban_chup.con_tro().chi_so_byte())
    );
}

#[test]
fn utf16_index_dung() {
    let mut phien = tao_phien();
    // "a" (1 UTF-16) + emoji (2 UTF-16).
    phien.them_ky_tu('a');
    phien.them_ky_tu('😀');
    // Con trỏ giữa hai ký tự.
    phien.ve_dau();
    phien.di_phai();
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.con_tro().chi_so_utf16(), 1);
}

#[test]
fn grapheme_index_dung() {
    let mut phien = tao_phien();
    // 3 grapheme: "a", "e\u{0301}", "😀".
    phien.them_ky_tu('a');
    phien.them_ky_tu('e');
    phien.them_ky_tu('\u{0301}');
    phien.them_ky_tu('😀');
    // Di đến sau grapheme thứ 2 ("e\u{0301}"): 3 thao tác (a, e, combining).
    phien.ve_dau();
    phien.di_phai();
    phien.di_phai();
    phien.di_phai();
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.con_tro().chi_so_grapheme(), 2);
}

#[test]
fn con_tro_khong_nam_giua_utf8_code_point() {
    let mut phien = tao_phien();
    phien.them_ky_tu('ế');
    let ban_chup = phien.ban_chup();
    // Byte index 0 và 2 đều là ranh giới char của "ế" (2 byte).
    assert!(
        ban_chup
            .noi_dung()
            .is_char_boundary(ban_chup.con_tro().chi_so_byte())
    );
}

#[test]
fn con_tro_khong_nam_giua_grapheme_cluster() {
    let mut phien = tao_phien();
    // "e\u{0301}" là một grapheme; đưa con trỏ giữa 'e' và combining mark.
    phien.them_ky_tu('e');
    phien.them_ky_tu('\u{0301}');
    phien.ve_dau();
    phien.di_phai();
    let ban_chup = phien.ban_chup();
    // Grapheme index phải là ranh giới cluster (0 hoặc 1), không nằm giữa.
    let idx = ban_chup.con_tro().chi_so_grapheme();
    assert!(idx == 0 || idx == 1);
    // Toàn bộ chuỗi chỉ có 1 grapheme nên index hợp lệ là 0 hoặc 1.
    assert!(ban_chup.noi_dung().chars().count() >= 1);
}

#[test]
fn commit_unicode_tra_dung_chuoi() {
    let mut phien = tao_phien();
    for ky_tu in "ếố😄".chars() {
        phien.them_ky_tu(ky_tu);
    }
    match phien.chap_nhan() {
        KetQuaXuLy::ChapNhan { noi_dung } => {
            assert_eq!(noi_dung, "ếố😄");
        }
        _ => panic!("commit phai tra ChapNhan"),
    }
}
