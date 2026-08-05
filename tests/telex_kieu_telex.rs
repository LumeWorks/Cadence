// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test KieuTelex::DayDu: `w` đơn lẻ → ư, `[` → ư, `]` → ơ.

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

fn go_canbang(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in raw.chars() {
        phien.them_ky_tu(ch);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// DayDu: `w` đơn lẻ → `ư`.
#[test]
fn day_du_w_don_le_thanh_u_horn() {
    assert_eq!(go_daydu("w"), "ư");
}

/// CanBang: `w` đơn lẻ giữ nguyên.
#[test]
fn can_bang_w_don_le_giu_nguyen() {
    assert_eq!(go_canbang("w"), "w");
}

/// DayDu: `w` + `s` → `ứ`.
#[test]
fn day_du_w_s_nhan_dau_sac() {
    assert_eq!(go_daydu("ws"), "ứ");
}

/// DayDu: `bw` → `bư` (w đơn lẻ sau phụ âm).
#[test]
fn day_du_w_sau_phu_am() {
    assert_eq!(go_daydu("bw"), "bư");
}

/// DayDu: `[` → `ư`.
#[test]
fn day_du_ngoac_mo_thanh_u_horn() {
    assert_eq!(go_daydu("["), "ư");
}

/// DayDu: `]` → `ơ`.
#[test]
fn day_du_ngoac_dong_thanh_o_horn() {
    assert_eq!(go_daydu("]"), "ơ");
}

/// DayDu: `[` + `s` → `ứ`.
#[test]
fn day_du_ngoac_mo_s_thanh_u_horn_sac() {
    assert_eq!(go_daydu("[s"), "ứ");
}

/// DayDu: `]` + `f` → `ờ`.
#[test]
fn day_du_ngoac_dong_f_thanh_o_horn_huyen() {
    assert_eq!(go_daydu("]f"), "ờ");
}

/// DayDu: `ow` vẫn → `ơ` (w là modifier khi có `o` trước).
#[test]
fn day_du_ow_van_la_o_horn() {
    assert_eq!(go_daydu("ow"), "ơ");
}

/// DayDu: `uw` vẫn → `ư` (w là modifier khi có `u` trước).
#[test]
fn day_du_uw_van_la_u_horn() {
    assert_eq!(go_daydu("uw"), "ư");
}

/// DayDu: `W` (hoa) → `Ư`.
#[test]
fn day_du_w_hoa_thanh_u_horn_hoa() {
    assert_eq!(go_daydu("W"), "Ư");
}

/// Cùng raw `w`, CanBang → `w`, DayDu → `ư`.
#[test]
fn can_bang_vs_day_du_khac_nhau() {
    assert_eq!(go_canbang("w"), "w");
    assert_eq!(go_daydu("w"), "ư");
}
