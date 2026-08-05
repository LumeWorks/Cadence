// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Definition-of-Done acceptance cases cho Phase 2 Telex engine.
//!
//! Mỗi test tại đây là một case đầu cuối: gõ raw → so sánh output.
//! Các case này phải pass trước khi Phase 2 được đóng.

use cadence::{BoGo, CauHinh};

fn go(phien: &mut cadence::PhienGo, raw: &str) -> String {
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

fn phien() -> cadence::PhienGo {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh hop le");
    bo_go.tao_phien()
}

// --- Hình chữ (shape transforms) ---

#[test]
fn dod_aa_thanh_a_moc() {
    assert_eq!(go(&mut phien(), "aa"), "â");
}

#[test]
fn dod_aw_thanh_a_breve() {
    assert_eq!(go(&mut phien(), "aw"), "ă");
}

#[test]
fn dod_ee_thanh_e_circumflex() {
    assert_eq!(go(&mut phien(), "ee"), "ê");
}

#[test]
fn dod_oo_thanh_o_circumflex() {
    assert_eq!(go(&mut phien(), "oo"), "ô");
}

#[test]
fn dod_ow_thanh_o_horn() {
    assert_eq!(go(&mut phien(), "ow"), "ơ");
}

#[test]
fn dod_uw_thanh_u_horn() {
    assert_eq!(go(&mut phien(), "uw"), "ư");
}

#[test]
fn dod_dd_thanh_d_stroke() {
    assert_eq!(go(&mut phien(), "dd"), "đ");
}

// --- Dấu thanh (tone marks) ---

#[test]
fn dod_tieengs_thanh_tieng() {
    assert_eq!(go(&mut phien(), "tieengs"), "tiếng");
}

#[test]
fn dod_vieetj_thanh_viet() {
    assert_eq!(go(&mut phien(), "Vieetj"), "Việt");
}

#[test]
fn dod_dduwowngf_thanh_duong() {
    assert_eq!(go(&mut phien(), "dduwowngf"), "đường");
}

#[test]
fn dod_ddaay_thanh_day() {
    assert_eq!(go(&mut phien(), "ddaay"), "đây");
}

#[test]
fn dod_nguowif_thanh_nguoi() {
    assert_eq!(go(&mut phien(), "nguowif"), "người");
}

// --- Escape (lặp phím modifier) ---

#[test]
fn dod_ass_thanh_as() {
    assert_eq!(go(&mut phien(), "ass"), "as");
}

#[test]
fn dod_aaa_thanh_aa() {
    assert_eq!(go(&mut phien(), "aaa"), "aa");
}

#[test]
fn dod_aww_thanh_aw() {
    assert_eq!(go(&mut phien(), "aww"), "aw");
}

#[test]
fn dod_ddd_thanh_dd() {
    assert_eq!(go(&mut phien(), "ddd"), "dd");
}

// --- Không can thiệp (Telex bypass) ---

#[test]
fn dod_async_khong_doi() {
    assert_eq!(go(&mut phien(), "async"), "async");
}

#[test]
fn dod_class_khong_doi() {
    assert_eq!(go(&mut phien(), "class"), "class");
}

#[test]
fn dod_ddm_thanh_dm() {
    assert_eq!(go(&mut phien(), "ddm"), "đm");
}
