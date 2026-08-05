// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test chữ hoa với Telex: shape, tone, escape ở dạng viết hoa.

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// `AA` → `Â` (shape hoa).
#[test]
fn aa_hoa_thanh_a_circumflex_hoa() {
    assert_eq!(go("AA"), "Â");
}

/// `AW` → `Ă` (shape hoa).
#[test]
fn aw_hoa_thanh_a_breve_hoa() {
    assert_eq!(go("AW"), "Ă");
}

/// `DD` → `Đ` (shape hoa).
#[test]
fn dd_hoa_thanh_d_stroke_hoa() {
    assert_eq!(go("DD"), "Đ");
}

/// `OW` → `Ơ` (shape hoa).
#[test]
fn ow_hoa_thanh_o_horn_hoa() {
    assert_eq!(go("OW"), "Ơ");
}

/// `UW` → `Ư` (shape hoa).
#[test]
fn uw_hoa_thanh_u_horn_hoa() {
    assert_eq!(go("UW"), "Ư");
}

/// `EE` → `Ê` (shape hoa).
#[test]
fn ee_hoa_thanh_e_circumflex_hoa() {
    assert_eq!(go("EE"), "Ê");
}

/// `OO` → `Ô` (shape hoa).
#[test]
fn oo_hoa_thanh_o_circumflex_hoa() {
    assert_eq!(go("OO"), "Ô");
}

/// `AS` → `Á` (tone hoa).
#[test]
fn as_hoa_thanh_a_sac_hoa() {
    assert_eq!(go("AS"), "Á");
}

/// `AF` → `À` (tone hoa).
#[test]
fn af_hoa_thanh_a_huyen_hoa() {
    assert_eq!(go("AF"), "À");
}

/// `AWF` → `Ằ` (shape + tone hoa).
#[test]
fn awf_hoa_shape_tone() {
    assert_eq!(go("AWF"), "Ằ");
}

/// `Vieetj` → `Việt` (mix hoa thường).
#[test]
fn vieetj_mix_hoa_thuong() {
    assert_eq!(go("Vieetj"), "Việt");
}

/// `DDeey` → `Đêy` (mix hoa thường với shape).
#[test]
fn ddeey_mix_hoa_thuong() {
    assert_eq!(go("DDeey"), "Đêy");
}

/// `ASS` → `AS` (escape hoa).
#[test]
fn ass_hoa_escape() {
    assert_eq!(go("ASS"), "AS");
}

/// `DDD` → `DD` (escape shape hoa).
#[test]
fn ddd_hoa_escape() {
    assert_eq!(go("DDD"), "DD");
}

/// `Tieengs` → `Tiếng` (hoa đầu câu).
#[test]
fn tieengs_hoa_dau_cau() {
    assert_eq!(go("Tieengs"), "Tiếng");
}
