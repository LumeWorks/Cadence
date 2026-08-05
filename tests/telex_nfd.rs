// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test Unicode NFD output: combining marks thay vì precomposed.

use cadence::{BoGo, CauHinh, DangUnicode};

fn go_nfd(raw: &str) -> String {
    let mut c = CauHinh::mac_dinh();
    c.dat_dang_unicode(DangUnicode::Nfd);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in raw.chars() {
        phien.them_ky_tu(ch);
    }
    phien.ban_chup().noi_dung().to_string()
}

fn go_nfc(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in raw.chars() {
        phien.them_ky_tu(ch);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// NFD: `aws` → `a` + U+0306 (breve) + U+0301 (sắc).
#[test]
fn nfd_aws_phan_ra_combining() {
    let nfd = go_nfd("aws");
    assert_eq!(nfd, "a\u{0306}\u{0301}");
}

/// NFC: `aws` → `ắ` (một codepoint).
#[test]
fn nfc_aws_dung_san() {
    assert_eq!(go_nfc("aws"), "ắ");
}

/// NFD: `ee` → `e` + U+0302 (circumflex).
#[test]
fn nfd_ee_phan_ra() {
    assert_eq!(go_nfd("ee"), "e\u{0302}");
}

/// NFD: `dd` → `đ` (không phân ra, đ không có decomposition).
#[test]
fn nfd_dd_khong_phan_ra() {
    assert_eq!(go_nfd("dd"), "đ");
}

/// NFD: `tieengs` → `ti` + `e` + U+0302 + U+0301 + `ng`.
#[test]
fn nfd_tieengs_phan_ra() {
    let nfd = go_nfd("tieengs");
    assert_eq!(nfd, "tie\u{0302}\u{0301}ng");
}

/// NFC vs NFD: khác byte length nhưng tương đương canonical.
#[test]
fn nfc_vs_nfd_khac_byte_nhung_tuong_duong() {
    let nfc = go_nfc("aws");
    let nfd = go_nfd("aws");
    assert_ne!(nfc.len(), nfd.len());
    // Canonical equivalent: NFD của NFC == NFD.
    use unicode_normalization::UnicodeNormalization;
    assert_eq!(nfc.nfd().collect::<String>(), nfd);
}

/// NFD: `nguowif` → `ngu\u{031b}\u{0301}ơi` — ư có horn + huyền.
#[test]
fn nfd_nguowif_phan_ra() {
    let nfd = go_nfd("nguowif");
    // `ng` + `u` + U+031B (horn) + `o` + U+031B (horn) + U+0301 (sắc? no, huyền=U+0300)
    // Wait: `nguowif` → `người` = ng + ư + ờ + i
    // NFD: ng + u + U+031B + o + U+031B + U+0300 + i
    assert_eq!(nfd, "ngu\u{031b}o\u{031b}\u{0300}i");
}

/// NFD: byte index vẫn là char boundary.
#[test]
fn nfd_byte_index_la_char_boundary() {
    let mut c = CauHinh::mac_dinh();
    c.dat_dang_unicode(DangUnicode::Nfd);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in "aws".chars() {
        phien.them_ky_tu(ch);
    }
    let bc = phien.ban_chup();
    assert!(bc.noi_dung().is_char_boundary(bc.con_tro().chi_so_byte()));
}

/// NFD: grapheme count vẫn đúng (1 grapheme cho `ắ`).
#[test]
fn nfd_grapheme_count_dung() {
    let mut c = CauHinh::mac_dinh();
    c.dat_dang_unicode(DangUnicode::Nfd);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in "aws".chars() {
        phien.them_ky_tu(ch);
    }
    let bc = phien.ban_chup();
    assert_eq!(bc.con_tro().chi_so_grapheme(), 1);
}
