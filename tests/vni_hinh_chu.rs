// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test VNI hình chữ: digit `6/7/8/9` và kết hợp với dấu thanh.

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

/// Hình chữ cơ bản: `6` mũ, `7` móc, `8` trăng, `9` đ.
#[test]
fn hinh_chu_co_ban() {
    assert_eq!(go_vni("a6"), "â");
    assert_eq!(go_vni("e6"), "ê");
    assert_eq!(go_vni("o6"), "ô");
    assert_eq!(go_vni("o7"), "ơ");
    assert_eq!(go_vni("u7"), "ư");
    assert_eq!(go_vni("a8"), "ă");
    assert_eq!(go_vni("d9"), "đ");
}

/// Viết hoa hình chữ.
#[test]
fn hinh_chu_hoa() {
    assert_eq!(go_vni("A6"), "Â");
    assert_eq!(go_vni("E6"), "Ê");
    assert_eq!(go_vni("O6"), "Ô");
    assert_eq!(go_vni("O7"), "Ơ");
    assert_eq!(go_vni("U7"), "Ư");
    assert_eq!(go_vni("A8"), "Ă");
    assert_eq!(go_vni("D9"), "Đ");
}

/// Kết hợp shape + tone: `a61` → ấ, `a81` → ắ, `o71` → ớ, `u71` → ứ.
#[test]
fn ket_hop_shape_tone() {
    assert_eq!(go_vni("a61"), "ấ");
    assert_eq!(go_vni("a62"), "ầ");
    assert_eq!(go_vni("a63"), "ẩ");
    assert_eq!(go_vni("a64"), "ẫ");
    assert_eq!(go_vni("a65"), "ậ");
    assert_eq!(go_vni("a81"), "ắ");
    assert_eq!(go_vni("a85"), "ặ");
    assert_eq!(go_vni("o71"), "ớ");
    assert_eq!(go_vni("o75"), "ợ");
    assert_eq!(go_vni("u71"), "ứ");
    assert_eq!(go_vni("u75"), "ự");
}

/// Thứ tự đảo: `a16` cũng cho `ấ`.
#[test]
fn thu_tu_dao() {
    assert_eq!(go_vni("a61"), go_vni("a16"));
    assert_eq!(go_vni("a81"), go_vni("a18"));
    assert_eq!(go_vni("o71"), go_vni("o17"));
    assert_eq!(go_vni("u71"), go_vni("u17"));
}

/// Thay shape: `a68` → ă (trăng thay mũ).
#[test]
fn thay_shape() {
    assert_eq!(go_vni("a68"), "ă");
    assert_eq!(go_vni("a86"), "â");
}

/// `0` không phải modifier → literal.
#[test]
fn so0_khong_modifier() {
    assert_eq!(go_vni("a0"), "a0");
    assert_eq!(go_vni("0a"), "0a");
}

/// Escape: lặp modifier → hoàn tác, hiện digit đầu.
#[test]
fn escape_hinh_chu() {
    assert_eq!(go_vni("a66"), "a6");
    assert_eq!(go_vni("a88"), "a8");
    assert_eq!(go_vni("d99"), "d9");
}

/// Escape tone: `a11` → `a1`.
#[test]
fn escape_dau_thanh() {
    assert_eq!(go_vni("a11"), "a1");
    assert_eq!(go_vni("a22"), "a2");
    assert_eq!(go_vni("a33"), "a3");
}

/// Chuỗi modifier dài không panic. Lặp modifier: apply, escape, apply, escape
/// → cứ 2 digit lặp thì 1 digit thành literal, 1 digit consumed.
#[test]
fn chuoi_modifier_dai_khong_panic() {
    // `a11` → `a1` (apply sắc, escape → literal `1`).
    assert_eq!(go_vni("a11"), "a1");
    // `a1111` → `a11` (apply, escape, apply, escape).
    assert_eq!(go_vni("a1111"), "a11");
    // `a6666` → `a66` (apply mũ, escape, apply, escape).
    assert_eq!(go_vni("a6666"), "a66");
    assert_eq!(go_vni("a8888"), "a88");
    assert_eq!(go_vni("o7777"), "o77");
    assert_eq!(go_vni("u7777"), "u77");
    assert_eq!(go_vni("d9999"), "d99");
}

/// ươ đặc biệt: `uo7` → ươ.
#[test]
fn uoua_dac_biet() {
    assert_eq!(go_vni("uo7"), "ươ");
    assert_eq!(go_vni("nguo7i2"), "người");
    assert_eq!(go_vni("d9uo7ng2"), "đường");
}

/// Digit không tương thích: `i6` → `i6` (i không nhận mũ).
#[test]
fn digit_khong_tuong_thich() {
    assert_eq!(go_vni("i6"), "i6");
    assert_eq!(go_vni("y6"), "y6");
    assert_eq!(go_vni("i7"), "i7");
    assert_eq!(go_vni("e7"), "e7");
    assert_eq!(go_vni("u8"), "u8");
}
