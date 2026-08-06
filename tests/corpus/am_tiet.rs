// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus âm tiết - enumeration mọi âm đầu + âm cuối + nguyên âm.
//! Liên kết branch: `kieu_go/am_tiet.rs` (bảng onset/coda, parser), `kieu_go/lua_chon.rs`.

use cadence::{BoGo, CauHinh, LoaiNoiDung};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

fn loai(raw: &str) -> LoaiNoiDung {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().loai_noi_dung()
}

/// Mọi onset + `a` hợp lệ → CoTheTiepTuc (raw vì không tone/shape).
#[test]
fn onset_hop_le_raw() {
    let onsets = [
        "b", "c", "d", "đ", "g", "h", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "x", "ch",
        "gh", "gi", "kh", "ng", "ngh", "nh", "ph", "qu", "th", "tr",
    ];
    for o in onsets {
        let raw = format!("{o}a");
        assert_eq!(go(&raw), raw, "onset {o}");
        // Không có biến đổi → NguyenBan.
        assert_eq!(loai(&raw), LoaiNoiDung::NguyenBan, "onset {o}");
    }
}

/// Onset không hợp lệ (`cl`, `fl`, `bl`) + tone → raw.
#[test]
fn onset_khong_hop_le_tone_raw() {
    assert_eq!(go("class"), "class");
    assert_eq!(go("flag"), "flag");
    assert_eq!(go("blue"), "blue");
}

/// Mọi coda sau `a` → raw (không tone).
#[test]
fn coda_hop_le_raw() {
    let codas = ["c", "m", "n", "p", "t", "ch", "ng", "nh"];
    for cd in codas {
        let raw = format!("a{cd}");
        assert_eq!(go(&raw), raw, "coda {cd}");
    }
}

/// Tone trên âm tiết hợp lệ (onset + vowel + coda) → Telex.
#[test]
fn tone_tren_am_tiet_hop_le() {
    assert_eq!(go("congs"), "cóng");
    assert_eq!(go("congf"), "còng");
    assert_eq!(go("bachs"), "bách");
    assert_eq!(go("ans"), "án");
}

/// Nucleus-glide: hai nguyên âm đầy không glide → raw (ngăn `ae` CASE).
#[test]
fn nucleus_glide_hai_nguyen_am_day_raw() {
    // `ae` không có glide {i,u,ư,y,o} → KhongHopLe → raw.
    assert_eq!(go("ae"), "ae");
    // `uo` có glide `u` → hợp lệ.
    assert_eq!(go("uo"), "uo");
}

/// `qu`/`gi`/`gh`/`ngh` edge cases.
#[test]
fn dac_biet_qu_gi_gh_ngh() {
    assert_eq!(go("quyen"), "quyen");
    assert_eq!(go("quyens"), "quyén");
    assert_eq!(go("gien"), "gien");
    assert_eq!(go("giens"), "gién");
    assert_eq!(go("ghi"), "ghi");
    assert_eq!(go("nghia"), "nghia");
}

/// Âm tiết mở (không coda) + tone → Telex.
#[test]
fn am_tiet_mo_tone() {
    assert_eq!(go("ba"), "ba");
    assert_eq!(go("bas"), "bá");
    assert_eq!(go("maf"), "mà");
}
