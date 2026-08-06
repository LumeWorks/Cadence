// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test VNI dấu thanh: digit `1..=5` trên mọi nguyên âm.

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

/// Dấu thanh `1..=5` trên nguyên âm đơn `a`.
#[test]
fn dau_thanh_tren_a() {
    assert_eq!(go_vni("a1"), "á");
    assert_eq!(go_vni("a2"), "à");
    assert_eq!(go_vni("a3"), "ả");
    assert_eq!(go_vni("a4"), "ã");
    assert_eq!(go_vni("a5"), "ạ");
}

/// Dấu thanh trên `e`, `i`, `o`, `u`, `y`.
#[test]
fn dau_thanh_tren_cac_nguyen_am() {
    assert_eq!(go_vni("e1"), "é");
    assert_eq!(go_vni("i2"), "ì");
    assert_eq!(go_vni("o3"), "ỏ");
    assert_eq!(go_vni("u4"), "ũ");
    assert_eq!(go_vni("y5"), "ỵ");
}

/// Thay dấu: digit mới thay digit cũ.
#[test]
fn thay_dau() {
    assert_eq!(go_vni("a12"), "à");
    assert_eq!(go_vni("a13"), "ả");
    assert_eq!(go_vni("a14"), "ã");
    assert_eq!(go_vni("a15"), "ạ");
    assert_eq!(go_vni("a21"), "á");
}

/// Dấu ở cuối âm tiết: `tieng61` → `tiếng`, `nguoi72` → `người`.
#[test]
fn dau_cuoi_am_tiet() {
    assert_eq!(go_vni("tieng61"), "tiếng");
    assert_eq!(go_vni("nguoi72"), "người");
    assert_eq!(go_vni("d9uo7ng2"), "đường");
}

/// Viết hoa: `A1` → `Á`, `A6` → `Â`.
#[test]
fn viet_hoa() {
    assert_eq!(go_vni("A1"), "Á");
    assert_eq!(go_vni("A6"), "Â");
    assert_eq!(go_vni("D9"), "Đ");
    assert_eq!(go_vni("O7"), "Ơ");
    assert_eq!(go_vni("U7"), "Ư");
}

/// `0` không phải modifier VNI → literal.
#[test]
fn khong0_literal() {
    assert_eq!(go_vni("a0"), "a0");
    assert_eq!(go_vni("0"), "0");
}

/// Digit mà không có nguyên âm → literal.
#[test]
fn digit_khong_nguyen_am_literal() {
    assert_eq!(go_vni("h264"), "h264");
    assert_eq!(go_vni("h2"), "h2");
    assert_eq!(go_vni("x86"), "x86");
}
