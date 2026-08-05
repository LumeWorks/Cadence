// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test dấu thanh Telex (s/f/r/x/j/z), thay dấu, xóa dấu, escape.

use cadence::{BoGo, CauHinh};

fn tao_phien() -> cadence::PhienGo {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh hop le");
    bo_go.tao_phien()
}

fn nhap(phien: &mut cadence::PhienGo, s: &str) {
    for c in s.chars() {
        phien.them_ky_tu(c);
    }
}

// --- Dấu thanh cơ bản ---

#[test]
fn as_thanh_a_sac() {
    let mut phien = tao_phien();
    nhap(&mut phien, "as");
    assert_eq!(phien.ban_chup().noi_dung(), "á");
}

#[test]
fn af_thanh_a_huyen() {
    let mut phien = tao_phien();
    nhap(&mut phien, "af");
    assert_eq!(phien.ban_chup().noi_dung(), "à");
}

#[test]
fn ar_thanh_a_hoi() {
    let mut phien = tao_phien();
    nhap(&mut phien, "ar");
    assert_eq!(phien.ban_chup().noi_dung(), "ả");
}

#[test]
fn ax_thanh_a_nga() {
    let mut phien = tao_phien();
    nhap(&mut phien, "ax");
    assert_eq!(phien.ban_chup().noi_dung(), "ã");
}

#[test]
fn aj_thanh_a_nang() {
    let mut phien = tao_phien();
    nhap(&mut phien, "aj");
    assert_eq!(phien.ban_chup().noi_dung(), "ạ");
}

#[test]
fn az_xoa_dau() {
    let mut phien = tao_phien();
    nhap(&mut phien, "as");
    assert_eq!(phien.ban_chup().noi_dung(), "á");
    phien.them_ky_tu('z');
    assert_eq!(phien.ban_chup().noi_dung(), "a");
}

#[test]
fn az_khong_dau_la_literal() {
    let mut phien = tao_phien();
    nhap(&mut phien, "az");
    assert_eq!(phien.ban_chup().noi_dung(), "az");
}

// --- Dấu thanh trên các nguyên âm khác ---

#[test]
fn es_thanh_e_sac() {
    let mut phien = tao_phien();
    nhap(&mut phien, "es");
    assert_eq!(phien.ban_chup().noi_dung(), "é");
}

#[test]
fn ees_thanh_ê_sac() {
    let mut phien = tao_phien();
    nhap(&mut phien, "ees");
    assert_eq!(phien.ban_chup().noi_dung(), "ế");
}

#[test]
fn aws_thanh_ă_sac() {
    let mut phien = tao_phien();
    nhap(&mut phien, "aws");
    assert_eq!(phien.ban_chup().noi_dung(), "ắ");
}

#[test]
fn ows_thanh_ơ_sac() {
    let mut phien = tao_phien();
    nhap(&mut phien, "ows");
    assert_eq!(phien.ban_chup().noi_dung(), "ớ");
}

#[test]
fn uws_thanh_ư_sac() {
    let mut phien = tao_phien();
    nhap(&mut phien, "uws");
    assert_eq!(phien.ban_chup().noi_dung(), "ứ");
}

#[test]
fn ys_thanh_y_sac() {
    let mut phien = tao_phien();
    nhap(&mut phien, "ys");
    assert_eq!(phien.ban_chup().noi_dung(), "ý");
}

// --- Thay dấu ---

#[test]
fn asf_thay_dau_thanh_huyen() {
    let mut phien = tao_phien();
    nhap(&mut phien, "asf");
    assert_eq!(phien.ban_chup().noi_dung(), "à");
    assert_eq!(phien.ban_chup().noi_dung_goc(), "asf");
}

#[test]
fn asr_thay_dau_thanh_hoi() {
    let mut phien = tao_phien();
    nhap(&mut phien, "asr");
    assert_eq!(phien.ban_chup().noi_dung(), "ả");
}

#[test]
fn asx_thay_dau_thanh_nga() {
    let mut phien = tao_phien();
    nhap(&mut phien, "asx");
    assert_eq!(phien.ban_chup().noi_dung(), "ã");
}

#[test]
fn asj_thay_dau_thanh_nang() {
    let mut phien = tao_phien();
    nhap(&mut phien, "asj");
    assert_eq!(phien.ban_chup().noi_dung(), "ạ");
}

// --- Dấu thanh uppercase ---

#[test]
fn a_hoa_s_thanh_a_sac() {
    let mut phien = tao_phien();
    nhap(&mut phien, "aS");
    assert_eq!(phien.ban_chup().noi_dung(), "á");
}

#[test]
fn a_hoa_s_hoa_thanh_a_sac_hoa() {
    let mut phien = tao_phien();
    nhap(&mut phien, "AS");
    assert_eq!(phien.ban_chup().noi_dung(), "Á");
}

// --- Escape lặp phím ---

#[test]
fn ass_escape_thanh_as() {
    let mut phien = tao_phien();
    nhap(&mut phien, "ass");
    assert_eq!(phien.ban_chup().noi_dung(), "as");
}

#[test]
fn aaa_escape_thanh_aa() {
    let mut phien = tao_phien();
    nhap(&mut phien, "aaa");
    assert_eq!(phien.ban_chup().noi_dung(), "aa");
}

#[test]
fn aww_escape_thanh_aw() {
    let mut phien = tao_phien();
    nhap(&mut phien, "aww");
    assert_eq!(phien.ban_chup().noi_dung(), "aw");
}

#[test]
fn ddd_escape_thanh_dd() {
    let mut phien = tao_phien();
    nhap(&mut phien, "ddd");
    assert_eq!(phien.ban_chup().noi_dung(), "dd");
}

#[test]
fn eee_escape_thanh_ee() {
    let mut phien = tao_phien();
    nhap(&mut phien, "eee");
    assert_eq!(phien.ban_chup().noi_dung(), "ee");
}

#[test]
fn ooo_escape_thanh_oo() {
    let mut phien = tao_phien();
    nhap(&mut phien, "ooo");
    assert_eq!(phien.ban_chup().noi_dung(), "oo");
}

#[test]
fn oww_escape_thanh_ow() {
    let mut phien = tao_phien();
    nhap(&mut phien, "oww");
    assert_eq!(phien.ban_chup().noi_dung(), "ow");
}

#[test]
fn uww_escape_thanh_uw() {
    let mut phien = tao_phien();
    nhap(&mut phien, "uww");
    assert_eq!(phien.ban_chup().noi_dung(), "uw");
}

// --- Escape chuỗi dài ---

#[test]
fn asss_escape_roi_ap_lai() {
    // ass → as (escape), s thứ 3 áp dụng sắc lại → ás
    let mut phien = tao_phien();
    nhap(&mut phien, "asss");
    assert_eq!(phien.ban_chup().noi_dung(), "ás");
}

#[test]
fn assss_escape_lan_nua() {
    // asss → ás, s thứ 4 escape → ass
    let mut phien = tao_phien();
    nhap(&mut phien, "assss");
    assert_eq!(phien.ban_chup().noi_dung(), "ass");
}

// --- Backspace hoàn tác ---

#[test]
fn backspace_sau_tone_hoan_tac() {
    let mut phien = tao_phien();
    nhap(&mut phien, "as");
    assert_eq!(phien.ban_chup().noi_dung(), "á");
    // Xóa s (tone key) → về a
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "a");
}

#[test]
fn backspace_sau_shape_tone_hoan_tac() {
    let mut phien = tao_phien();
    nhap(&mut phien, "aws");
    assert_eq!(phien.ban_chup().noi_dung(), "ắ");
    // Xóa s (tone) → ă
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "ă");
    // Xóa w (shape modifier) → a
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "a");
}

// --- them_nguyen_ban không kích hoạt tone ---

#[test]
fn them_nguyen_ban_khong_kich_hoat_tone() {
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('s');
    assert_eq!(phien.ban_chup().noi_dung(), "as");
}
