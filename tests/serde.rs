// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Kiểm tra derive serde (chỉ chạy khi bật feature `serde`).
//!
//! Phase 4 mở rộng: kiểm tra tất cả public type có derive serde,
//! và round-trip serialize → deserialize với serde_json.

#![cfg(feature = "serde")]

use cadence::{ChinhSachLuaChon, DangUnicode, KetQuaXuLy, KieuTelex, LoaiNoiDung, QuyTacDatDau};
use serde::{Deserialize, Serialize};

fn assert_serialize<T: Serialize>() {}
fn assert_deserialize<'de, T: Deserialize<'de>>() {}

#[test]
fn ket_qua_xu_ly_co_serde() {
    assert_serialize::<KetQuaXuLy>();
    assert_deserialize::<KetQuaXuLy>();
}

#[test]
fn loai_noi_dung_co_serde() {
    assert_serialize::<LoaiNoiDung>();
    assert_deserialize::<LoaiNoiDung>();
}

#[test]
fn kieu_telex_co_serde() {
    assert_serialize::<KieuTelex>();
    assert_deserialize::<KieuTelex>();
}

#[test]
fn quy_tac_dat_dau_co_serde() {
    assert_serialize::<QuyTacDatDau>();
    assert_deserialize::<QuyTacDatDau>();
}

#[test]
fn dang_unicode_co_serde() {
    assert_serialize::<DangUnicode>();
    assert_deserialize::<DangUnicode>();
}

#[test]
fn chinh_sach_lua_chon_co_serde() {
    assert_serialize::<ChinhSachLuaChon>();
    assert_deserialize::<ChinhSachLuaChon>();
}

/// Round-trip: serialize → deserialize → equals cho mọi variant KieuTelex.
#[test]
fn kieu_telex_round_trip() {
    for k in [KieuTelex::CanBang, KieuTelex::DayDu] {
        let json = serde_json::to_string(&k).expect("serialize");
        let k2: KieuTelex = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(k, k2);
    }
}

/// Round-trip: QuyTacDatDau.
#[test]
fn quy_tac_dat_dau_round_trip() {
    for q in [QuyTacDatDau::HienDai, QuyTacDatDau::TruyenThong] {
        let json = serde_json::to_string(&q).expect("serialize");
        let q2: QuyTacDatDau = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(q, q2);
    }
}

/// Round-trip: DangUnicode.
#[test]
fn dang_unicode_round_trip() {
    for d in [DangUnicode::Nfc, DangUnicode::Nfd] {
        let json = serde_json::to_string(&d).expect("serialize");
        let d2: DangUnicode = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(d, d2);
    }
}

/// Round-trip: ChinhSachLuaChon.
#[test]
fn chinh_sach_lua_chon_round_trip() {
    for c in [
        ChinhSachLuaChon::TuNhien,
        ChinhSachLuaChon::UuTienTiengViet,
        ChinhSachLuaChon::UuTienNguyenBan,
    ] {
        let json = serde_json::to_string(&c).expect("serialize");
        let c2: ChinhSachLuaChon = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(c, c2);
    }
}

/// Round-trip: LoaiNoiDung — tất cả variant.
#[test]
fn loai_noi_dung_round_trip() {
    let tat_ca = [
        LoaiNoiDung::Trong,
        LoaiNoiDung::NguyenBan,
        LoaiNoiDung::BienDoiTelex,
        LoaiNoiDung::AmTietTiengViet,
    ];
    for l in tat_ca {
        let json = serde_json::to_string(&l).expect("serialize {l:?}");
        let l2: LoaiNoiDung = serde_json::from_str(&json).expect("deserialize {l:?}");
        assert_eq!(l, l2, "round-trip {l:?}");
    }
}

/// Round-trip: KetQuaXuLy — ChapNhan, KhongDoi, CapNhat.
#[test]
fn ket_qua_xu_ly_round_trip() {
    let chap_nhan = KetQuaXuLy::ChapNhan {
        noi_dung: "tiếng".into(),
    };
    let json = serde_json::to_string(&chap_nhan).expect("serialize");
    let kq2: KetQuaXuLy = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(kq2, chap_nhan);

    let khong_doi = KetQuaXuLy::KhongDoi;
    let json = serde_json::to_string(&khong_doi).expect("serialize");
    let kq2: KetQuaXuLy = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(kq2, khong_doi);

    let cap_nhat = KetQuaXuLy::CapNhat;
    let json = serde_json::to_string(&cap_nhat).expect("serialize");
    let kq2: KetQuaXuLy = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(kq2, cap_nhat);
}
