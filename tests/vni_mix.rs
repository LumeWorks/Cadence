// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test VNI mixed content: tiếng Việt trộn code, số kỹ thuật.

use cadence::{BoGo, CauHinh, KieuGo};

fn go_vni(raw: &str) -> String {
    let mut c = CauHinh::mac_dinh();
    c.dat_kieu_go(KieuGo::Vni);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in raw.chars() {
        phien.them_ky_tu(ch);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// `toi6_dang_fix_h264` — `toi6` → tôi, `h264` raw.
#[test]
fn toi6_dang_fix_h264() {
    assert_eq!(go_vni("toi6"), "tôi");
    assert_eq!(go_vni("h264"), "h264");
    let result = go_vni("toi6_dang_fix_h264");
    assert!(result.starts_with("tôi"), "result: {result}");
    assert!(result.contains("h264"), "result: {result}");
}

/// `sha256 bi loi64` — sha256 raw, loi64 → lỗ.
#[test]
fn sha256_bi_loi64() {
    assert_eq!(go_vni("sha256"), "sha256");
    let result = go_vni("sha256 bi loi64");
    assert!(result.starts_with("sha256"), "result: {result}");
}

/// `user123 cua toi6` — user123 raw, toi6 → tôi.
#[test]
fn user123_cua_toi6() {
    let result = go_vni("user123 cua toi6");
    assert!(result.starts_with("user123"), "result: {result}");
    assert!(result.ends_with("tôi"), "result: {result}");
}

/// `cargo build loi64` — cargo/build raw.
#[test]
fn cargo_build_loi64() {
    let result = go_vni("cargo build loi64");
    assert!(result.starts_with("cargo build"), "result: {result}");
}

/// Telex mặc định không bị ảnh hưởng bởi VNI.
#[test]
fn telex_mac_dinh_khong_bi_anh_huong() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("mac dinh");
    let mut phien = bo_go.tao_phien();
    for ch in "tieengs".chars() {
        phien.them_ky_tu(ch);
    }
    assert_eq!(phien.ban_chup().noi_dung(), "tiếng");
}

/// Hai BoGo khác kiểu gõ độc lập.
#[test]
fn hai_bogo_doc_lap() {
    let mut c1 = CauHinh::mac_dinh();
    c1.dat_kieu_go(KieuGo::Telex);
    let mut c2 = CauHinh::mac_dinh();
    c2.dat_kieu_go(KieuGo::Vni);

    let bo_tx = BoGo::new(c1).expect("hop le");
    let bo_vni = BoGo::new(c2).expect("hop le");

    let mut p_tx = bo_tx.tao_phien();
    let mut p_vni = bo_vni.tao_phien();

    for ch in "as".chars() {
        p_tx.them_ky_tu(ch);
    }
    for ch in "a1".chars() {
        p_vni.them_ky_tu(ch);
    }
    assert_eq!(p_tx.ban_chup().noi_dung(), "á");
    assert_eq!(p_vni.ban_chup().noi_dung(), "á");
}
