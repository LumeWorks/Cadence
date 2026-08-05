// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test round-trip NFC/NFD: render → parse → render cho tất cả tổ hợp.

use cadence::{BoGo, CauHinh, DangUnicode};
use unicode_normalization::UnicodeNormalization;

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

/// NFD của NFC == NFD: canonical equivalence.
#[test]
fn nfc_nfd_tuong_duong() {
    let nfc = go("tieengs", DangUnicode::Nfc);
    let nfd = go("tieengs", DangUnicode::Nfd);
    assert_eq!(nfc.nfd().collect::<String>(), nfd);
}

/// NFC của NFD == NFC: canonical equivalence ngược.
#[test]
fn nfd_nfc_tuong_duong() {
    let nfc = go("tieengs", DangUnicode::Nfc);
    let nfd = go("tieengs", DangUnicode::Nfd);
    assert_eq!(nfd.nfc().collect::<String>(), nfc);
}

/// Round-trip cho mọi shape: NFC output NFD lại vẫn bằng.
#[test]
fn round_trip_shape_nfc_nfd_nfc() {
    for raw in &["aa", "aw", "ee", "oo", "ow", "uw", "dd"] {
        let nfc1 = go(raw, DangUnicode::Nfc);
        let nfd = nfc1.nfd().collect::<String>();
        let nfc2 = nfd.nfc().collect::<String>();
        assert_eq!(nfc1, nfc2, "round-trip cho {raw}");
    }
}

/// Round-trip cho tone: NFC output NFD lại vẫn bằng.
#[test]
fn round_trip_tone_nfc_nfd_nfc() {
    for raw in &["as", "af", "ar", "ax", "aj"] {
        let nfc1 = go(raw, DangUnicode::Nfc);
        let nfd = nfc1.nfd().collect::<String>();
        let nfc2 = nfd.nfc().collect::<String>();
        assert_eq!(nfc1, nfc2, "round-trip cho {raw}");
    }
}

/// Round-trip cho shape+tone.
#[test]
fn round_trip_shape_tone() {
    for raw in &["aws", "awf", "ees", "oos", "ows", "uws"] {
        let nfc1 = go(raw, DangUnicode::Nfc);
        let nfd = nfc1.nfd().collect::<String>();
        let nfc2 = nfd.nfc().collect::<String>();
        assert_eq!(nfc1, nfc2, "round-trip cho {raw}");
    }
}

/// NFD cho `đ`: không phân ra (đ không có decomposition).
#[test]
fn nfd_d_khong_phan_ra() {
    let nfd = go("dd", DangUnicode::Nfd);
    assert_eq!(nfd, "đ");
    let nfc = go("dd", DangUnicode::Nfc);
    assert_eq!(nfd, nfc);
}

/// NFD cho triphthong `ươ`.
#[test]
fn nfd_uo() {
    let nfd = go("uow", DangUnicode::Nfd);
    assert_eq!(nfd, "u\u{031b}o\u{031b}");
}

/// NFD cho `nguowif` → `người`.
#[test]
fn nfd_nguoi_canonical_equivalence() {
    let nfc = go("nguowif", DangUnicode::Nfc);
    let nfd = go("nguowif", DangUnicode::Nfd);
    assert_eq!(nfc.nfd().collect::<String>(), nfd);
    assert_eq!(nfd.nfc().collect::<String>(), nfc);
}

/// NFC và NFD khác byte length nhưng cùng grapheme count.
#[test]
fn nfc_nfd_khac_byte_cung_grapheme() {
    use unicode_segmentation::UnicodeSegmentation;
    let nfc = go("aws", DangUnicode::Nfc);
    let nfd = go("aws", DangUnicode::Nfd);
    assert_ne!(nfc.len(), nfd.len());
    assert_eq!(nfc.graphemes(true).count(), nfd.graphemes(true).count());
}

/// Them_nguyen_ban với ký tự precomposed: `ế` giữ nguyên.
#[test]
fn nguyen_ban_emarkhong_double_tone() {
    let c = CauHinh::mac_dinh();
    let bo_go = BoGo::new(c).expect("ok");
    let mut phien = bo_go.tao_phien();
    phien.them_nguyen_ban('ế');
    phien.them_ky_tu('s');
    // Raw `ế` chặn tone `s` xuyên, `s` là literal.
    let kq = phien.ban_chup().noi_dung();
    assert!(kq.starts_with('ế'), "kỳ vọng ế ở đầu, được {kq}");
}
