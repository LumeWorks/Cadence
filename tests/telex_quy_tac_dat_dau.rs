// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test quy tắc đặt dấu thanh: HienDai vs TruyenThong.

use cadence::{BoGo, CauHinh, QuyTacDatDau};

fn go(cau_hinh: CauHinh, raw: &str) -> String {
    let bo_go = BoGo::new(cau_hinh).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

fn hien_dai() -> CauHinh {
    let mut c = CauHinh::mac_dinh();
    c.dat_quy_tac_dat_dau(QuyTacDatDau::HienDai);
    c
}

fn truyen_thong() -> CauHinh {
    let mut c = CauHinh::mac_dinh();
    c.dat_quy_tac_dat_dau(QuyTacDatDau::TruyenThong);
    c
}

/// HienDai: `hoas` → `hóa` (dấu trên `o`).
#[test]
fn hien_dai_hoa_s_tren_o() {
    assert_eq!(go(hien_dai(), "hoas"), "hóa");
}

/// TruyenThong: `hoas` → `hoá` (dấu trên `a`).
#[test]
fn truyen_thong_hoa_s_tren_a() {
    assert_eq!(go(truyen_thong(), "hoas"), "hoá");
}

/// HienDai: `hoaf` → `hòa` (dấu trên `o`).
#[test]
fn hien_dai_hoa_f_tren_o() {
    assert_eq!(go(hien_dai(), "hoaf"), "hòa");
}

/// TruyenThong: `hoaf` → `hoà` (dấu trên `a`).
#[test]
fn truyen_thong_hoa_f_tren_a() {
    assert_eq!(go(truyen_thong(), "hoaf"), "hoà");
}

/// Đơn nguyên âm: cả hai quy tắc cho cùng kết quả.
#[test]
fn don_nguyen_am_khong_khac() {
    assert_eq!(go(hien_dai(), "as"), "á");
    assert_eq!(go(truyen_thong(), "as"), "á");
}

/// `oe`: HienDai trên `o`, TruyenThong trên `e`.
#[test]
fn hien_dai_oe_tren_o() {
    assert_eq!(go(hien_dai(), "does"), "dóe");
}

#[test]
fn truyen_thong_oe_tren_e() {
    assert_eq!(go(truyen_thong(), "does"), "doé");
}

/// `nguowif` → `người`: cả hai quy tắc cho cùng kết quả (tone trên `ơ`).
#[test]
fn nguoi_khong_khac_giua_hai_quy_tac() {
    assert_eq!(go(hien_dai(), "nguowif"), "người");
    assert_eq!(go(truyen_thong(), "nguowif"), "người");
}
