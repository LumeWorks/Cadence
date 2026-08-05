// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Property tests Phase 3 - bất biến mới sau phân đoạn và nhận diện.
//!
//! 1. `noi_dung_goc` nguyên vẹn byte-for-byte.
//! 2. Round-trip: `them_nguyen_ban` cho mỗi char → `noi_dung` == raw.
//! 3. Cấu trúc kỹ thuật (URL, `::`, `=`) luôn raw.
//! 4. Deterministic: cùng input → cùng output.

use cadence::{BoGo, CauHinh, ChinhSachLuaChon};
use proptest::prelude::*;

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

fn go_nguyen_ban(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_nguyen_ban(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

fn noi_dung_goc(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung_goc().to_string()
}

/// Chiến lược sinh chuỗi từ pool có nghĩa: ASCII letters, dấu câu, kỹ thuật.
fn chu_co_nghia() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop_oneof![
            Just('a'),
            Just('b'),
            Just('c'),
            Just('d'),
            Just('e'),
            Just('f'),
            Just('g'),
            Just('h'),
            Just('i'),
            Just('o'),
            Just('r'),
            Just('s'),
            Just('t'),
            Just('u'),
            Just('w'),
            Just('x'),
            Just('z'),
            Just(':'),
            Just('/'),
            Just('.'),
            Just('-'),
            Just('_'),
            Just('@'),
            Just('='),
            Just(' '),
            Just('1'),
            Just('2'),
        ],
        0..20,
    )
    .prop_map(|cs| cs.into_iter().collect())
}

/// Chiến lược sinh chuỗi không có khoảng trắng (cho path/identifier).
fn chu_lien_tuc() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop_oneof![
            Just('a'),
            Just('b'),
            Just('c'),
            Just('d'),
            Just('e'),
            Just('f'),
            Just('g'),
            Just('h'),
            Just('i'),
            Just('o'),
            Just('r'),
            Just('s'),
            Just('t'),
            Just('u'),
            Just('w'),
            Just('x'),
            Just('z'),
            Just('.'),
            Just('-'),
            Just('_'),
            Just('1'),
            Just('2'),
        ],
        0..16,
    )
    .prop_map(|cs| cs.into_iter().collect())
}

/// Chiến lược sinh chuỗi chỉ chữ cái (cho identifier/segment liền kề kỹ thuật).
fn chu_chi() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop_oneof![
            Just('a'),
            Just('b'),
            Just('c'),
            Just('d'),
            Just('e'),
            Just('f'),
            Just('g'),
            Just('h'),
            Just('i'),
            Just('o'),
            Just('r'),
            Just('s'),
            Just('t'),
            Just('u'),
            Just('w'),
            Just('x'),
            Just('z'),
        ],
        0..16,
    )
    .prop_map(|cs| cs.into_iter().collect())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Bất biến: `noi_dung_goc` == raw input, byte-for-byte.
    #[test]
    fn noi_dung_goc_nguyen_ven(raw in chu_co_nghia()) {
        prop_assert_eq!(&noi_dung_goc(&raw), &raw);
    }

    /// Bất biến: round-trip `them_nguyen_ban` → `noi_dung` == raw.
    #[test]
    fn round_trip_nguyen_ban(raw in chu_co_nghia()) {
        prop_assert_eq!(&go_nguyen_ban(&raw), &raw);
    }

    /// Bất biến: deterministic - cùng input → cùng output.
    #[test]
    fn deterministic(raw in chu_co_nghia()) {
        let a = go(&raw);
        let b = go(&raw);
        prop_assert_eq!(a, b);
    }

    /// Bất biến: URL `https://...` luôn raw (không bị Telex).
    #[test]
    fn url_luon_raw(path in chu_lien_tuc()) {
        let raw = format!("https://x.com/{path}");
        let out = go(&raw);
        prop_assert_eq!(out, raw);
    }

    /// Bất biến: `foo::bar` luôn raw (bar chỉ chữ cái).
    #[test]
    fn namespace_luon_raw(bar in chu_chi()) {
        let raw = format!("foo::{bar}");
        let out = go(&raw);
        prop_assert_eq!(out, raw);
    }

    /// Bất biến: `x = y` luôn raw (y chỉ chữ cái).
    #[test]
    fn gan_bang_luon_raw(y in chu_chi()) {
        let raw = format!("x = {y}");
        let out = go(&raw);
        prop_assert_eq!(out, raw);
    }

    /// Bất biến: output không dài hơn raw × 4 (NFD có thể phình combining).
    #[test]
    fn output_khong_qua_dai(raw in chu_co_nghia()) {
        let out = go(&raw);
        prop_assert!(out.len() <= raw.len() * 4 + 4);
    }

    /// Bất biến: mọi chính sách cho cùng URL raw.
    #[test]
    fn moi_chinh_sach_url_raw(path in chu_lien_tuc()) {
        let raw = format!("https://x.com/{path}");
        for cs in [ChinhSachLuaChon::TuNhien, ChinhSachLuaChon::UuTienTiengViet, ChinhSachLuaChon::UuTienNguyenBan] {
            let mut ch = CauHinh::mac_dinh();
            ch.dat_chinh_sach_lua_chon(cs);
            let bo = BoGo::new(ch).expect("cau hinh hop le");
            let mut p = bo.tao_phien();
            for c in raw.chars() { p.them_ky_tu(c); }
            prop_assert_eq!(p.ban_chup().noi_dung(), &raw[..]);
        }
    }
}
