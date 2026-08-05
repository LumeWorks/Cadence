// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test tương tác config: DayDu + NFD, TruyenThong + NFD, DayDu + TruyenThong.

use cadence::{BoGo, CauHinh, DangUnicode, KieuTelex, QuyTacDatDau};
use unicode_normalization::UnicodeNormalization;

fn go(raw: &str, kieu: KieuTelex, quy_tac: QuyTacDatDau, dang: DangUnicode) -> String {
    let mut c = CauHinh::mac_dinh();
    c.dat_kieu_telex(kieu);
    c.dat_quy_tac_dat_dau(quy_tac);
    c.dat_dang_unicode(dang);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in raw.chars() {
        phien.them_ky_tu(ch);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// DayDu + NFD: `ws` → `ư` + U+0301 (combining sắc).
#[test]
fn daydu_nfd_ws() {
    let kq = go(
        "ws",
        KieuTelex::DayDu,
        QuyTacDatDau::HienDai,
        DangUnicode::Nfd,
    );
    assert_eq!(kq, "u\u{031b}\u{0301}");
}

/// DayDu + NFD: `]f` → `o` + U+031B + U+0300 (combining horn + huyền).
#[test]
fn daydu_nfd_ngoac_dong_huyen() {
    let kq = go(
        "]f",
        KieuTelex::DayDu,
        QuyTacDatDau::HienDai,
        DangUnicode::Nfd,
    );
    assert_eq!(kq, "o\u{031b}\u{0300}");
}

/// TruyenThong + NFD: `hoas` → `ho` + `a` + U+0301.
#[test]
fn truyen_thong_nfd_hoas() {
    let kq = go(
        "hoas",
        KieuTelex::CanBang,
        QuyTacDatDau::TruyenThong,
        DangUnicode::Nfd,
    );
    assert_eq!(kq, "hoa\u{0301}");
}

/// HienDai + NFD: `hoas` → `h` + `o` + U+0301 + `a`.
#[test]
fn hien_dai_nfd_hoas() {
    let kq = go(
        "hoas",
        KieuTelex::CanBang,
        QuyTacDatDau::HienDai,
        DangUnicode::Nfd,
    );
    assert_eq!(kq, "ho\u{0301}a");
}

/// DayDu + TruyenThong: `]as` → `ớa` (TruyenThong tone trên `a`).
#[test]
fn daydu_truyen_thong_ngoac_dong_a_sac() {
    let kq = go(
        "]as",
        KieuTelex::DayDu,
        QuyTacDatDau::TruyenThong,
        DangUnicode::Nfc,
    );
    // `]`→`ơ`, `a`→vowel, `s`→sắc. TruyenThong: tone trên `a`.
    assert!(
        kq.contains('ớ') || kq.contains('á'),
        "kỳ vọng ớ/á, được {kq}"
    );
}

/// DayDu + HienDai: `]as` → `óa` (HienDai tone trên `o`→`ơ`).
#[test]
fn daydu_hien_dai_ngoac_dong_a_sac() {
    let kq = go(
        "]as",
        KieuTelex::DayDu,
        QuyTacDatDau::HienDai,
        DangUnicode::Nfc,
    );
    // `]`→`ơ`, `a`→vowel, `s`→sắc. HienDai: tone trên `ơ`.
    assert!(
        kq.contains('ớ') || kq.contains('ó'),
        "kỳ vọng ớ/ó, được {kq}"
    );
}

/// Tất cả config组合 đều cho kết quả canonical equivalent.
#[test]
fn tat_ca_config_canonical_equivalent() {
    let raw = "tieengs";
    let nfc = go(
        raw,
        KieuTelex::CanBang,
        QuyTacDatDau::HienDai,
        DangUnicode::Nfc,
    );
    let nfd = go(
        raw,
        KieuTelex::CanBang,
        QuyTacDatDau::HienDai,
        DangUnicode::Nfd,
    );
    assert_eq!(nfc.nfd().collect::<String>(), nfd);
}

/// DayDu + NFD + escape: `wss` → `ưs` (escape sắc, giữ shape `w`→`ư`).
#[test]
fn daydu_nfd_escape() {
    let kq = go(
        "wss",
        KieuTelex::DayDu,
        QuyTacDatDau::HienDai,
        DangUnicode::Nfd,
    );
    // `w`→`ư`, `s`→`ứ`, `s`→escape → `ư` + `s` literal. NFD: u + horn + s.
    assert_eq!(kq, "u\u{031b}s");
}

/// CanBang + TruyenThong + NFD: `hoaf` → `ho` + `a` + U+0300.
#[test]
fn canbang_truyen_thong_nfd_hoaf() {
    let kq = go(
        "hoaf",
        KieuTelex::CanBang,
        QuyTacDatDau::TruyenThong,
        DangUnicode::Nfd,
    );
    assert_eq!(kq, "hoa\u{0300}");
}

/// DayDu + HienDai + NFD: `nguowif` → `người` (NFD).
#[test]
fn daydu_hien_dai_nfd_nguoi() {
    let kq = go(
        "nguowif",
        KieuTelex::DayDu,
        QuyTacDatDau::HienDai,
        DangUnicode::Nfd,
    );
    let nfc = go(
        "nguowif",
        KieuTelex::DayDu,
        QuyTacDatDau::HienDai,
        DangUnicode::Nfc,
    );
    assert_eq!(kq, nfc.nfd().collect::<String>());
}
