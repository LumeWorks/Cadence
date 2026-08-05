// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test chính sách lựa chọn `ChinhSachLuaChon` — sự khác biệt giữa
//! `TuNhien`, `UuTienTiengViet`, và `UuTienNguyenBan`.

use cadence::{BoGo, CauHinh, ChinhSachLuaChon};

fn go(raw: &str, cs: ChinhSachLuaChon) -> String {
    let mut ch = CauHinh::mac_dinh();
    ch.dat_chinh_sach_lua_chon(cs);
    let bo_go = BoGo::new(ch).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

use ChinhSachLuaChon::*;

/// `TuNhien`: tone + âm tiết không hợp lệ → raw (`async`→`async`).
#[test]
fn tu_nhien_tone_khong_hop_le_ve_raw() {
    assert_eq!(go("async", TuNhien), "async");
    assert_eq!(go("cargo", TuNhien), "cargo");
}

/// `UuTienTiengViet`: tone + âm tiết không hợp lệ → Telex (`async`→`áync`).
#[test]
fn uu_tien_tieng_viet_tone_telex() {
    assert_eq!(go("async", UuTienTiengViet), "áync");
    assert_eq!(go("cargo", UuTienTiengViet), "cảgo");
}

/// `UuTienNguyenBan`: giống `TuNhien` — tone + không hợp lệ → raw.
#[test]
fn uu_tien_nguyen_ban_tone_raw() {
    assert_eq!(go("async", UuTienNguyenBan), "async");
    assert_eq!(go("cargo", UuTienNguyenBan), "cargo");
}

/// Mọi chính sách: âm tiết Việt hoàn chỉnh → Telex (`tieengs`→`tiếng`).
#[test]
fn tat_ca_chinh_sach_am_tiet_telex() {
    for cs in [TuNhien, UuTienTiengViet, UuTienNguyenBan] {
        assert_eq!(go("tieengs", cs), "tiếng", "cs={cs:?}");
        assert_eq!(go("nguowif", cs), "người", "cs={cs:?}");
    }
}

/// Mọi chính sách: shape + onset hợp lệ → Telex (`ddm`→`đm`).
#[test]
fn tat_ca_chinh_sach_shape_telex() {
    for cs in [TuNhien, UuTienTiengViet, UuTienNguyenBan] {
        assert_eq!(go("ddm", cs), "đm", "cs={cs:?}");
    }
}

/// Mọi chính sách: shape + onset không hợp lệ → raw (`foo`→`foo`).
#[test]
fn tat_ca_chinh_sach_shape_onset_sai_raw() {
    for cs in [TuNhien, UuTienTiengViet, UuTienNguyenBan] {
        assert_eq!(go("foo", cs), "foo", "cs={cs:?}");
    }
}

/// Mọi chính sách: 2+ dấu thanh → raw (`user`→`user`).
#[test]
fn tat_ca_chinh_sach_2_dau_thanh_raw() {
    for cs in [TuNhien, UuTienTiengViet, UuTienNguyenBan] {
        assert_eq!(go("user", cs), "user", "cs={cs:?}");
    }
}

/// Mọi chính sách: cấu trúc kỹ thuật chắc chắn → raw.
#[test]
fn tat_ca_chinh_sach_cau_truc_raw() {
    for cs in [TuNhien, UuTienTiengViet, UuTienNguyenBan] {
        assert_eq!(
            go("https://example.com", cs),
            "https://example.com",
            "cs={cs:?}"
        );
        assert_eq!(go("foo::bar", cs), "foo::bar", "cs={cs:?}");
        assert_eq!(go("let mut buf = x;", cs), "let mut buf = x;", "cs={cs:?}");
    }
}
