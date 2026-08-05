// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test edge case của âm tiết tiếng Việt qua public API.
//! Kiểm tra parser bằng cách gõ các âm tiết hợp lệ/không hợp lệ.

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// Âm tiết hợp lệ: `con` → `con` (onset c + vowel o + coda n).
#[test]
fn am_tiet_con_hop_le() {
    assert_eq!(go("con"), "con");
}

/// Âm tiết hợp lệ: `ngang` → `ngang` (onset ng + vowel a + coda ng).
#[test]
fn am_tiet_ngang_hop_le() {
    assert_eq!(go("ngang"), "ngang");
}

/// Âm tiết hợp lệ: `nghiem` → `nghiem` (onset ngh + vowel ie + coda m).
#[test]
fn am_tiet_nghiem_hop_le() {
    assert_eq!(go("nghiem"), "nghiem");
}

/// Âm tiết hợp lệ với tone: `cong` + `s` → `cóng`.
#[test]
fn am_tiet_cong_sac_hop_le() {
    assert_eq!(go("congs"), "cóng");
}

/// Onset `ngh` + vowel `i`: `nghia` → `nghia` (hợp lệ).
#[test]
fn am_tiet_nghia_hop_le() {
    assert_eq!(go("nghia"), "nghia");
}

/// Âm tiết rỗng onset: `an` → `an` (vowel a + coda n).
#[test]
fn am_tiet_an_hop_le() {
    assert_eq!(go("an"), "an");
}

/// Tone trên âm tiết hợp lệ: `an` + `s` → `án`.
#[test]
fn am_tiet_an_sac_hop_le() {
    assert_eq!(go("ans"), "án");
}

/// Tone trên `cong` + `f` → `còng` (hợp lệ).
#[test]
fn am_tiet_cong_huyen() {
    assert_eq!(go("congf"), "còng");
}

/// Onset không hợp lệ `cl`: `class` → `class` (raw).
#[test]
fn am_tiet_cl_khong_hop_le() {
    assert_eq!(go("class"), "class");
}

/// Onset `fl` không hợp lệ: `flag` → `flag` (raw).
#[test]
fn am_tiet_fl_khong_hop_le() {
    assert_eq!(go("flag"), "flag");
}

/// Onset `bl` không hợp lệ: `blue` → `blue` (raw).
#[test]
fn am_tiet_bl_khong_hop_le() {
    assert_eq!(go("blue"), "blue");
}

/// Vần chỉ chứa nguyên âm: `aio` → `aio` (hợp lệ, CoTheTiepTuc).
#[test]
fn am_tiet_aio_hop_le() {
    assert_eq!(go("aio"), "aio");
}

/// Tone trên `aio`: `aios` → `aío` (tone trên `i`, `o` là off-glide).
#[test]
fn am_tiet_aio_sac() {
    let kq = go("aios");
    assert!(kq.contains('í'), "kỳ vọng í, được {kq}");
}

/// Triphthong `nguowif` → `người` (tone trên `ơ`).
#[test]
fn am_tiet_nguoi_triphthong() {
    assert_eq!(go("nguowif"), "người");
}

/// Triphthong `dduwowngf` → `đường` (tone trên `ư`).
#[test]
fn am_tiet_duong_triphthong() {
    assert_eq!(go("dduwowngf"), "đường");
}

/// `qu` + vowel: `quyen` → `quyen` (onset `qu` + vowel `ye` + coda `n`).
#[test]
fn am_tiet_quyen_hop_le() {
    assert_eq!(go("quyen"), "quyen");
}

/// Tone trên `quyen`: `quyens` → `quyén` (tone trên `y`).
#[test]
fn am_tiet_quyen_sac() {
    assert_eq!(go("quyens"), "quyén");
}

/// `gi` + vowel: `gien` → `gien` (onset `gi` + vowel `e` + coda `n`).
#[test]
fn am_tiet_gien_hop_le() {
    assert_eq!(go("gien"), "gien");
}

/// Tone trên `gien`: `giens` → `gién`.
#[test]
fn am_tiet_gien_sac() {
    assert_eq!(go("giens"), "gién");
}

/// Coda `ch`: `bach` → `bach` (onset b + vowel a + coda ch).
#[test]
fn am_tiet_bach_hop_le() {
    assert_eq!(go("bach"), "bach");
}

/// Tone trên `bach`: `bachs` → `bách`.
#[test]
fn am_tiet_bach_sac() {
    assert_eq!(go("bachs"), "bách");
}
