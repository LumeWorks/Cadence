// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test DayDu với tone marks: `ws`→`ứ`, `wf`→`ừ`, `[s`→`ứ`, `]f`→`ờ`.

use cadence::{BoGo, CauHinh, KieuTelex};

fn go_daydu(raw: &str) -> String {
    let mut c = CauHinh::mac_dinh();
    c.dat_kieu_telex(KieuTelex::DayDu);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in raw.chars() {
        phien.them_ky_tu(ch);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// `ws` → `ứ` (w→ư + sắc).
#[test]
fn daydu_ws_thanh_u_sac() {
    assert_eq!(go_daydu("ws"), "ứ");
}

/// `wf` → `ừ` (w→ư + huyền).
#[test]
fn daydu_wf_thanh_u_huyen() {
    assert_eq!(go_daydu("wf"), "ừ");
}

/// `wr` → `ử` (w→ư + hỏi).
#[test]
fn daydu_wr_thanh_u_hoi() {
    assert_eq!(go_daydu("wr"), "ử");
}

/// `wx` → `ữ` (w→ư + ngã).
#[test]
fn daydu_wx_thanh_u_nga() {
    assert_eq!(go_daydu("wx"), "ữ");
}

/// `wj` → `ự` (w→ư + nặng).
#[test]
fn daydu_wj_thanh_u_nang() {
    assert_eq!(go_daydu("wj"), "ự");
}

/// `wz` → `ưz` (z không có dấu để xóa → literal).
#[test]
fn daydu_wz_khong_dau_la_literal() {
    assert_eq!(go_daydu("wz"), "ưz");
}

/// `[s` → `ứ` ([→ư + sắc).
#[test]
fn daydu_ngoac_mo_s_thanh_u_sac() {
    assert_eq!(go_daydu("[s"), "ứ");
}

/// `]f` → `ờ` (]→ơ + huyền).
#[test]
fn daydu_ngoac_dong_f_thanh_o_huyen() {
    assert_eq!(go_daydu("]f"), "ờ");
}

/// `]r` → `ở` (]→ơ + hỏi).
#[test]
fn daydu_ngoac_dong_r_thanh_o_hoi() {
    assert_eq!(go_daydu("]r"), "ở");
}

/// `]j` → `ợ` (]→ơ + nặng).
#[test]
fn daydu_ngoac_dong_j_thanh_o_nang() {
    assert_eq!(go_daydu("]j"), "ợ");
}

/// Thay dấu: `wsf` → `ừ` (sắc bị huyền thay).
#[test]
fn daydu_thay_sac_bang_huyen() {
    assert_eq!(go_daydu("wsf"), "ừ");
}

/// `wsz` → `ư` (z xóa sắc).
#[test]
fn daydu_wsz_xoa_sac() {
    assert_eq!(go_daydu("wsz"), "ư");
}

/// Hoa: `Ws` → `Ứ`.
#[test]
fn daydu_w_hoa_s_thanh_u_sac_hoa() {
    assert_eq!(go_daydu("Ws"), "Ứ");
}

/// Hoa: `WS` → `Ứ` (cả w và s hoa).
#[test]
fn daydu_w_hoa_s_hoa() {
    assert_eq!(go_daydu("WS"), "Ứ");
}

/// `bws` → `bứ` (w đơn lẻ sau phụ âm + sắc).
#[test]
fn daydu_bw_s_thanh_b_u_sac() {
    assert_eq!(go_daydu("bws"), "bứ");
}
