// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test VNI xung đột số: chuỗi kỹ thuật phải giữ nguyên raw.

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

/// Mỗi case phải ra byte-for-byte.
#[test]
fn chuoi_ky_thuat_giu_raw() {
    let cases = [
        "sha256",
        "md5",
        "h264",
        "h265",
        "ipv4",
        "ipv6",
        "utf8",
        "utf16",
        "x86",
        "x86_64",
        "arm64",
        "aarch64",
        "v1.2.3",
        "1.85.0",
        "127.0.0.1",
        "192.168.1.1:8080",
        "2026-08-06",
        "user123",
        "port3000",
        "localhost3000",
    ];
    for raw in &cases {
        assert_eq!(go_vni(raw), *raw, "xung dot so: {raw}");
    }
}

/// UUID giữ raw.
#[test]
fn uuid_giu_raw() {
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    assert_eq!(go_vni(uuid), uuid);
}

/// `user123 cua toi6` — `user123` raw, `toi6` → tôi.
#[test]
fn mixed_user_toi() {
    assert_eq!(go_vni("user123"), "user123");
    assert_eq!(go_vni("toi6"), "tôi");
}

/// `sha256 bi loi64` — sha256 raw.
#[test]
fn sha256_raw() {
    assert_eq!(go_vni("sha256"), "sha256");
}

/// Telex và VNI không ảnh hưởng nhau.
#[test]
fn telex_vni_doc_lap() {
    let mut c = CauHinh::mac_dinh();
    let bo_go = BoGo::new(c).expect("hop le");
    let mut phien_tx = bo_go.tao_phien();
    for ch in "sha256".chars() {
        phien_tx.them_ky_tu(ch);
    }
    assert_eq!(phien_tx.ban_chup().noi_dung(), "sha256");

    c.dat_kieu_go(KieuGo::Vni);
    let bo_vni = BoGo::new(c).expect("hop le");
    let mut phien_vni = bo_vni.tao_phien();
    for ch in "sha256".chars() {
        phien_vni.them_ky_tu(ch);
    }
    assert_eq!(phien_vni.ban_chup().noi_dung(), "sha256");
}
