// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test `them_nguyen_ban` bypass Telex: ký tự literal, chặn Telex rules.

use cadence::{BoGo, CauHinh};

fn phien() -> cadence::PhienGo {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    bo_go.tao_phien()
}

/// `them_nguyen_ban('ế')` giữ nguyên `ế`, không re-process.
#[test]
fn nguyen_ban_giu_ky_tu_dung_san() {
    let mut phien = phien();
    phien.them_nguyen_ban('ế');
    assert_eq!(phien.ban_chup().noi_dung(), "ế");
}

/// `them_ky_tu('a')` + `them_nguyen_ban('w')` + `them_ky_tu('a')` → `awa`
/// (raw `w` không là shape modifier).
#[test]
fn nguyen_ban_w_khong_la_shape_modifier() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('w');
    phien.them_ky_tu('a');
    assert_eq!(phien.ban_chup().noi_dung(), "awa");
}

/// `them_ky_tu('a')` + `them_nguyen_ban('a')` → `aa` (raw `a` không trigger shape).
#[test]
fn nguyen_ban_a_chan_shape_aa() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('a');
    assert_eq!(phien.ban_chup().noi_dung(), "aa");
}

/// `them_nguyen_ban('A')` + `them_ky_tu('A')` → `AA` (raw `A` chặn shape).
#[test]
fn nguyen_ban_hoa_chan_telex() {
    let mut phien = phien();
    phien.them_nguyen_ban('A');
    phien.them_ky_tu('A');
    assert_eq!(phien.ban_chup().noi_dung(), "AA");
}

/// Telex + raw + Telex: `as` + raw `x` + `as` → `áxá` (hai đoạn độc lập).
#[test]
fn telex_roi_nguyen_ban_roi_telex() {
    let mut phien = phien();
    for c in "as".chars() {
        phien.them_ky_tu(c);
    }
    phien.them_nguyen_ban('x');
    for c in "as".chars() {
        phien.them_ky_tu(c);
    }
    assert_eq!(phien.ban_chup().noi_dung(), "áxá");
}

/// Raw `ế` chặn tone xuyên: `a` + raw `ế` + `s` → `aếs`.
#[test]
fn nguyen_ban_chan_tone_quay_lai() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('ế');
    phien.them_ky_tu('s');
    assert_eq!(phien.ban_chup().noi_dung(), "aếs");
}

/// Backspace trên `them_nguyen_ban` xóa đúng một raw action.
#[test]
fn nguyen_ban_xoa_lui_hoan_tac_mot_raw() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('x');
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "a");
}

/// `noi_dung_goc` chứa cả raw `them_nguyen_ban`.
#[test]
fn nguyen_ban_xuat_hien_trong_noi_dung_goc() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('ế');
    phien.them_ky_tu('s');
    assert_eq!(phien.ban_chup().noi_dung_goc(), "aếs");
}

/// Raw `w` giữa hai `a` không tạo `ă` và không tạo `â`.
#[test]
fn nguyen_ban_w_giua_hai_a() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('w');
    phien.them_ky_tu('a');
    // raw `w` chặn `aa` shape → `awa`, không `â` hay `ă`.
    assert_eq!(phien.ban_chup().noi_dung(), "awa");
}
