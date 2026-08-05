// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus URL/email/path - cấu trúc kỹ thuật chắc chắn → raw.
//! Liên kết branch: `ngu_canh.rs` (nhan_url, nhan_email, nhan_duong_dan).

use cadence::{BoGo, CauHinh, ChinhSachLuaChon};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

fn go_cs(raw: &str, cs: ChinhSachLuaChon) -> String {
    let mut c = CauHinh::mac_dinh();
    c.dat_chinh_sach_lua_chon(cs);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in raw.chars() {
        phien.them_ky_tu(ch);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// URL mọi scheme → raw.
#[test]
fn url_moi_scheme() {
    let cases = [
        "https://example.com",
        "http://localhost:3000",
        "https://x.com/search?q=tiếng",
        "https://github.com/LumeWorks/Cadence",
        "ftp://ftp.example.com/file",
        "ssh://git@github.com/repo.git",
        "https://example.com/path?q=1&r=2#anchor",
    ];
    for raw in cases {
        assert_eq!(go(raw), raw, "url {raw}");
    }
}

/// Email → raw.
#[test]
fn email() {
    let cases = [
        "name@example.com",
        "test@as.com",
        "user.name+tag@sub.example.com",
        "a@b.co",
    ];
    for raw in cases {
        assert_eq!(go(raw), raw, "email {raw}");
    }
}

/// IPv4/IPv6/port → raw.
#[test]
fn ip_port() {
    assert_eq!(go("127.0.0.1:8080"), "127.0.0.1:8080");
    assert_eq!(go("192.168.1.1"), "192.168.1.1");
    assert_eq!(go("::1"), "::1");
    assert_eq!(go("fe80::1"), "fe80::1");
}

/// UUID/git SHA/checksum/version/date → raw.
#[test]
fn uuid_sha_version_date() {
    assert_eq!(
        go("550e8400-e29b-41d4-a716-446655440000"),
        "550e8400-e29b-41d4-a716-446655440000"
    );
    assert_eq!(go("c9868e1"), "c9868e1");
    assert_eq!(go("deadbeefcafe"), "deadbeefcafe");
    assert_eq!(go("v1.2.3"), "v1.2.3");
    assert_eq!(go("1.85.0"), "1.85.0");
    assert_eq!(go("2026-08-05"), "2026-08-05");
    assert_eq!(go("23:59:59"), "23:59:59");
}

/// File path Unix/Windows/relative → raw.
#[test]
fn file_path() {
    let cases = [
        "/home/minh/docs",
        "~/Documents/Cadence",
        "./install.sh",
        "../parent/dir",
        r"C:\Users\Name",
        r"D:\Projects\src\lib.rs",
        "/usr/local/bin",
    ];
    for raw in cases {
        assert_eq!(go(raw), raw, "path {raw}");
    }
}

/// Package coordinate → raw.
#[test]
fn package_coordinate() {
    assert_eq!(go("cadence-ime 0.1.0"), "cadence-ime 0.1.0");
    assert_eq!(go("serde = \"1\""), "serde = \"1\"");
    assert_eq!(
        go("unicode-segmentation = \"1\""),
        "unicode-segmentation = \"1\""
    );
}

/// URL raw trong mọi chính sách (bất biến).
#[test]
fn url_raw_moi_chinh_sach() {
    let raw = "https://example.com/as";
    for cs in [
        ChinhSachLuaChon::TuNhien,
        ChinhSachLuaChon::UuTienTiengViet,
        ChinhSachLuaChon::UuTienNguyenBan,
    ] {
        assert_eq!(go_cs(raw, cs), raw, "cs {cs:?}");
    }
}

/// `hoaf.com` không có `://` → không phải URL → Telex (tone trên o).
#[test]
fn hoaf_com_khong_phai_url() {
    assert_eq!(go("hoaf.com"), "hòa.com");
}
