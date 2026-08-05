// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test phân đoạn qua public API - chia lịch sử thành đoạn cùng loại,
//! Telex chạy độc lập từng đoạn, không xuyên ranh giới.

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// Chu duy nhất → Telex biến đổi trọn vẹn.
#[test]
fn chu_duy_nhat_telex() {
    assert_eq!(go("tieengs"), "tiếng");
}

/// Khoảng trắng tách đoạn - tone không xuyên qua.
#[test]
fn khoang_trang_tach_doan() {
    // "cargo build" → cả hai đoạn raw (identifier), không "cảgo bủild".
    assert_eq!(go("cargo build"), "cargo build");
}

/// Dấu câu tách đoạn (`_` là DauCau).
#[test]
fn dau_cau_tach_doan() {
    // "user_id" → "user" và "id" độc lập, "user" raw (onset "us" + tone).
    assert_eq!(go("user_id"), "user_id");
}

/// `them_nguyen_ban` tạo ranh giới đoạn - Telex không vượt qua.
#[test]
fn nguyen_ban_tach_rieng() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('x');
    phien.them_ky_tu('b');
    // "a" và "b" độc lập; "a" không có tone → "a", "b" → "b".
    assert_eq!(phien.ban_chup().noi_dung(), "axb");
}

/// DayDu: `[`/`]` là Chu (sinh ư/ơ).
#[test]
fn daydu_ngoac_la_chu() {
    let mut ch = CauHinh::mac_dinh();
    ch.dat_kieu_telex(cadence::KieuTelex::DayDu);
    let bo_go = BoGo::new(ch).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    // "]f" trong DayDu → ơ + tone (f=huyền) → "ờ".
    phien.them_ky_tu(']');
    phien.them_ky_tu('f');
    assert_eq!(phien.ban_chup().noi_dung(), "ờ");
}

/// CanBang: `[`/`]` là KyThuat (ranh giới).
#[test]
fn canbang_ngoac_la_ky_thuat() {
    // CanBang: "]f" → "]" (KyThuat, raw) + "f" (Chu, raw).
    assert_eq!(go("]f"), "]f");
}

/// Emoji tách riêng - không qua Telex.
#[test]
fn emoji_rieng() {
    assert_eq!(go("a😀b"), "a😀b");
}

/// Teencode lặp `brooo` → raw (có chữ khác trước).
#[test]
fn teencode_lap_brooo_raw() {
    assert_eq!(go("brooo"), "brooo");
    assert_eq!(go("brooooo"), "brooooo");
}

/// `ooo` nguyên đoạn → escape Phase 2 (`oo`), không phải teencode lap.
#[test]
fn ooo_escape_khong_phai_teencode_lap() {
    // "ooo" → "oo" (escape Telex, run bắt đầu ở 0).
    assert_eq!(go("ooo"), "oo");
}

/// Teencode lap không ảnh hưởng tone escape hợp lệ.
#[test]
fn teencode_lap_khong_anh_huong_tone() {
    // "ass" → "as" (escape), "ddm" → "đm" (shape), "tieengs" → "tiếng".
    assert_eq!(go("ass"), "as");
    assert_eq!(go("ddm"), "đm");
    assert_eq!(go("tieengs"), "tiếng");
}
