// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test đầy đủ 5 dấu thanh + xóa dấu (z) trên các nguyên âm.

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

// --- Sắc (s) ---

#[test]
fn sac_a() {
    assert_eq!(go("as"), "á");
}

#[test]
fn sac_e() {
    assert_eq!(go("es"), "é");
}

#[test]
fn sac_o() {
    assert_eq!(go("os"), "ó");
}

#[test]
fn sac_u() {
    assert_eq!(go("us"), "ú");
}

#[test]
fn sac_i() {
    assert_eq!(go("is"), "í");
}

// --- Huyền (f) ---

#[test]
fn huyen_a() {
    assert_eq!(go("af"), "à");
}

#[test]
fn huyen_o() {
    assert_eq!(go("of"), "ò");
}

#[test]
fn huyen_u() {
    assert_eq!(go("uf"), "ù");
}

// --- Hỏi (r) ---

#[test]
fn hoi_a() {
    assert_eq!(go("ar"), "ả");
}

#[test]
fn hoi_o() {
    assert_eq!(go("or"), "ỏ");
}

#[test]
fn hoi_u() {
    assert_eq!(go("ur"), "ủ");
}

// --- Ngã (x) ---

#[test]
fn nga_a() {
    assert_eq!(go("ax"), "ã");
}

#[test]
fn nga_o() {
    assert_eq!(go("ox"), "õ");
}

#[test]
fn nga_u() {
    assert_eq!(go("ux"), "ũ");
}

// --- Nặng (j) ---

#[test]
fn nang_a() {
    assert_eq!(go("aj"), "ạ");
}

#[test]
fn nang_o() {
    assert_eq!(go("oj"), "ọ");
}

#[test]
fn nang_u() {
    assert_eq!(go("uj"), "ụ");
}

// --- Xóa dấu (z) ---

#[test]
fn xoa_dau_sac() {
    assert_eq!(go("asz"), "a");
}

#[test]
fn xoa_dau_huyen() {
    assert_eq!(go("afz"), "a");
}

#[test]
fn xoa_dau_sau_shape() {
    assert_eq!(go("awfz"), "ă");
}

// --- Thay dấu ---

#[test]
fn thay_sac_bang_huyen() {
    assert_eq!(go("asf"), "à");
}

#[test]
fn thay_huyen_bang_hoi() {
    assert_eq!(go("afr"), "ả");
}

#[test]
fn thay_dau_tren_shape() {
    assert_eq!(go("aws"), "ắ");
    assert_eq!(go("awsf"), "ằ");
    assert_eq!(go("awsr"), "ẳ");
}

// --- z khi không có dấu: literal ---

#[test]
fn z_khong_dau_la_literal() {
    assert_eq!(go("az"), "az");
}

#[test]
fn z_dau_tien_khong_xoa() {
    // `a` không có dấu, `z` là literal.
    assert_eq!(go("az"), "az");
}
