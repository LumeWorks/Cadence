// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus teencode - tiếng lóng, chữ lặp, viết tắt. Bảo toàn hơn là sửa.
//! Liên kết branch: `phan_doan.rs::la_teencode_lap`, `lua_chon.rs`.

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// Teencode lặp (run 3+ doubled-base có chữ khác trước) → raw.
#[test]
fn teencode_lap() {
    assert_eq!(go("brooo"), "brooo");
    assert_eq!(go("brooooo"), "brooooo");
    assert_eq!(go("heyyyy"), "heyyyy");
    assert_eq!(go("soooo"), "soooo");
    assert_eq!(go("lolll"), "lolll");
}

/// Teencode viết tắt → raw.
#[test]
fn teencode_viet_tat() {
    let cases = ["ko", "dc", "j", "k", "bn", "vcl", "vl", "nma", "mk"];
    for raw in cases {
        assert_eq!(go(raw), raw, "{raw}");
    }
}

/// `ooo` nguyên đoạn → escape Phase 2 (`oo`), không phải teencode lặp.
#[test]
fn ooo_escape_khong_phai_teencode_lap() {
    assert_eq!(go("ooo"), "oo");
    // `oo` đơn thuần → `ô` (shape).
    assert_eq!(go("oo"), "ô");
}

/// Chữ lặp không doubled-base (vd `lll`, `sss`) → raw hoặc escape.
#[test]
fn chu_lap_khong_doubled_base() {
    // `sss` → escape `as`? Không, `sss` bắt đầu bằng `s` (tone key) không có vowel.
    // `sss` → `ss` (escape tone? không, s đầu không có tone). Thực tế raw.
    let kq = go("sss");
    assert!(kq == "sss" || kq == "ss", "kỳ vọng sss hoặc ss, được {kq}");
}

/// Teencode trộn tiếng Việt.
#[test]
fn teencode_tron_tieng_viet() {
    assert_eq!(go("ko có gì"), "ko có gì");
    assert_eq!(go("dc rồi"), "dc rồi");
}
