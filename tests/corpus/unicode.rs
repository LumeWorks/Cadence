// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus Unicode - NFC/NFD idempotent, combining mark, ZWJ, boundary.
//! Liên kết branch: `render.rs`, `vi_tri.rs::tai_byte`, `phan_doan.rs::LoaiDoan::Emoji`.

use cadence::{BoGo, CauHinh, DangUnicode};
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

fn go(raw: &str, dang: DangUnicode) -> String {
    let mut c = CauHinh::mac_dinh();
    c.dat_dang_unicode(dang);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in raw.chars() {
        phien.them_ky_tu(ch);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// NFC output idempotent dưới NFC; NFD idempotent dưới NFD.
#[test]
fn nfc_nfd_idempotent() {
    for raw in ["aws", "tieengs", "nguowif", "dduwowngf"] {
        let nfc = go(raw, DangUnicode::Nfc);
        let nfd = go(raw, DangUnicode::Nfd);
        assert_eq!(nfc.nfc().collect::<String>(), nfc, "NFC idempotent {raw}");
        assert_eq!(nfd.nfd().collect::<String>(), nfd, "NFD idempotent {raw}");
    }
}

/// NFC và NFD canonical equivalent cho cùng input.
#[test]
fn nfc_nfd_canonical_equivalent() {
    for raw in ["aws", "tieengs", "nguowif", "ees", "ows"] {
        let nfc = go(raw, DangUnicode::Nfc);
        let nfd = go(raw, DangUnicode::Nfd);
        assert_eq!(nfc.nfd().collect::<String>(), nfd, "equiv {raw}");
        assert_eq!(nfd.nfc().collect::<String>(), nfc, "equiv nguoc {raw}");
    }
}

/// Byte index luôn là UTF-8 boundary cho mọi tổ hợp.
#[test]
fn byte_index_char_boundary() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    for raw in ["ếốờữ", "ađ😀", "e\u{0301}😀", "tiếng"] {
        let mut phien = bo_go.tao_phien();
        for c in raw.chars() {
            phien.them_ky_tu(c);
        }
        let bc = phien.ban_chup();
        assert!(
            bc.noi_dung().is_char_boundary(bc.con_tro().chi_so_byte()),
            "char boundary cho {raw}"
        );
    }
}

/// Combining mark không bị tách khỏi base trong grapheme.
#[test]
fn combining_mark_mot_grapheme() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    phien.them_ky_tu('e');
    phien.them_ky_tu('\u{0301}');
    let bc = phien.ban_chup();
    assert_eq!(bc.noi_dung().graphemes(true).count(), 1);
    assert_eq!(bc.con_tro().chi_so_grapheme(), 1);
}

/// ZWJ sequence không bị tách tại public cursor.
#[test]
fn zwj_khong_tach() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in ['\u{1F468}', '\u{200D}', '\u{1F469}'] {
        phien.them_ky_tu(c);
    }
    // Di chuyển con trỏ - không kẹt giữa ZWJ.
    phien.ve_dau();
    phien.di_phai();
    let bc = phien.ban_chup();
    // Toàn bộ chuỗi 1 grapheme; cursor ở 0 hoặc 1.
    let g = bc.con_tro().chi_so_grapheme();
    assert!(g == 0 || g == 1, "grapheme {g}");
}

/// NFD output: grapheme count đúng (combining mark thuộc base).
#[test]
fn nfd_grapheme_count_dung() {
    let nfd = go("aws", DangUnicode::Nfd);
    let nfc = go("aws", DangUnicode::Nfc);
    assert_ne!(nfd.len(), nfc.len());
    assert_eq!(nfd.graphemes(true).count(), nfc.graphemes(true).count());
}

/// Ký tự lặp preservation: `aaa` → `aa` (escape), không mất.
#[test]
fn ky_tu_lap_preservation() {
    assert_eq!(go("aaa", DangUnicode::Nfc), "aa");
    // Raw giữ `aaa`.
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in "aaa".chars() {
        phien.them_ky_tu(c);
    }
    assert_eq!(phien.ban_chup().noi_dung_goc(), "aaa");
}
