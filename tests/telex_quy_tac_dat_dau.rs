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

// ---------------------------------------------------------------------------
// Matrix: mọi tone key × mọi on-glide combo (HienDai vs TruyenThong).
// ---------------------------------------------------------------------------

/// Matrix: `oa` + tone — HienDai trên `o`, TruyenThong trên `a`.
#[test]
fn matrix_oa_tone() {
    let cases = [
        ('s', "hóa", "hoá"),
        ('f', "hòa", "hoà"),
        ('r', "hỏa", "hoả"),
        ('x', "hõa", "hoã"),
        ('j', "họa", "hoạ"),
    ];
    for (tone, hd, tt) in cases {
        let raw = format!("hoa{tone}");
        assert_eq!(go(hien_dai(), &raw), hd, "HienDai {raw}");
        assert_eq!(go(truyen_thong(), &raw), tt, "TruyenThong {raw}");
    }
}

/// Matrix: `oe` + tone — HienDai trên `o`, TruyenThong trên `e`.
#[test]
fn matrix_oe_tone() {
    let cases = [
        ('s', "dóe", "doé"),
        ('f', "dòe", "doè"),
        ('r', "dỏe", "doẻ"),
        ('x', "dõe", "doẽ"),
        ('j', "dọe", "doẹ"),
    ];
    for (tone, hd, tt) in cases {
        let raw = format!("doe{tone}");
        assert_eq!(go(hien_dai(), &raw), hd, "HienDai {raw}");
        assert_eq!(go(truyen_thong(), &raw), tt, "TruyenThong {raw}");
    }
}

/// Matrix: `ua` + tone — cả hai quy tắc cho cùng kết quả (tone trên `a`).
#[test]
fn matrix_ua_tone_cung_ket_qua() {
    for tone in ['s', 'f', 'r', 'x', 'j'] {
        let raw = format!("hua{tone}");
        assert_eq!(
            go(hien_dai(), &raw),
            go(truyen_thong(), &raw),
            "ua{tone} phai cung ket qua"
        );
    }
}

/// Đơn nguyên âm + tone: cả hai quy tắc cho cùng kết quả.
#[test]
fn matrix_don_nguyen_am_tone_cung_ket_qua() {
    for v in ['a', 'e', 'i', 'o', 'u', 'y'] {
        for tone in ['s', 'f', 'r', 'x', 'j'] {
            let raw = format!("{v}{tone}");
            assert_eq!(
                go(hien_dai(), &raw),
                go(truyen_thong(), &raw),
                "{v}{tone} phai cung ket qua"
            );
        }
    }
}

/// Hình chữ + tone: cả hai quy tắc cho cùng kết quả (shape không có glide).
#[test]
fn matrix_hinh_chu_tone_cung_ket_qua() {
    let shapes = ["aa", "aw", "ee", "oo", "ow", "uw"];
    for shape in &shapes {
        for tone in ['s', 'f', 'r', 'x', 'j'] {
            let raw = format!("{shape}{tone}");
            assert_eq!(
                go(hien_dai(), &raw),
                go(truyen_thong(), &raw),
                "{shape}{tone} phai cung ket qua"
            );
        }
    }
}
