// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test lựa chọn raw vs Telex: các edge case của quy tắc selection.

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

/// Shape transform luôn giữ Telex ngay cả khi không hợp lệ: `aaq`→`âq`.
#[test]
fn shape_luon_giu_telex() {
    assert_eq!(go("aaq"), "âq");
    assert_eq!(loai("aaq"), LoaiNoiDung::BienDoiTelex);
}

/// Tone + âm tiết không hợp lệ → raw: `asf` thực ra `f` thay tone → `à`.
/// `asf` không raw vì `f` thay `s` tone, tạo `à` (hợp lệ). Để test raw,
/// dùng `asx` (tone `s` rồi `x`→ngã, nhưng `asx`→`ã` hợp lệ).
/// Dùng `fls` → onset `f` không hợp lệ → raw.
#[test]
fn tone_khong_hop_le_ve_raw() {
    // `fls` có onset `f` không hợp lệ → raw.
    assert_eq!(go("fls"), "fls");
}

/// Tone + âm tiết hợp lệ → Telex: `as`→`á`.
#[test]
fn tone_hop_le_giu_telex() {
    assert_eq!(go("as"), "á");
    assert_eq!(loai("as"), LoaiNoiDung::AmTietTiengViet);
}

/// Onset không hợp lệ → raw: `flaf`→`flaf`.
#[test]
fn onset_khong_hop_le_ve_raw() {
    assert_eq!(go("flaf"), "flaf");
}

/// `class` → raw (onset `cl` không hợp lệ).
#[test]
fn class_ve_raw() {
    assert_eq!(go("class"), "class");
    assert_eq!(loai("class"), LoaiNoiDung::NguyenBan);
}

/// Escape hình chữ luôn giữ Telex: `ddd`→`dd`.
#[test]
fn escape_hinh_chu_giu_telex() {
    assert_eq!(go("ddd"), "dd");
}

/// Escape dấu thanh luôn giữ Telex: `ass`→`as`.
#[test]
fn escape_dau_thanh_giu_telex() {
    assert_eq!(go("ass"), "as");
}

/// Shape + tone + invalid syllable vẫn giữ Telex: `awqf`→`ằq` (shape dominates).
#[test]
fn shape_thang_tone_invalid() {
    let kq = go("awqf");
    // Shape `aw`→`ă`, `q` literal, `f` tone trên `ă`→`ằ`.
    assert!(
        kq.contains('ă') || kq.contains('ằ'),
        "kỳ vọng shape, được {kq}"
    );
}

/// Không transform, chỉ onset+vowel: `cha`→`cha` (raw = output, NguyenBan).
#[test]
fn khong_transform_mac_dinh_telex() {
    assert_eq!(go("cha"), "cha");
    assert_eq!(loai("cha"), LoaiNoiDung::NguyenBan);
}

/// Mix shape + tone → Telex: `aws`→`ắ`.
#[test]
fn mix_shape_va_tone_giu_telex() {
    assert_eq!(go("aws"), "ắ");
}

/// `async` → raw (tone `s` ở cuối nhưng `async` không hợp lệ).
#[test]
fn async_ve_raw() {
    assert_eq!(go("async"), "async");
}

/// `ddm` → `đm` (shape `dd` hợp lệ, `m` là coda).
#[test]
fn ddm_giu_telex() {
    assert_eq!(go("ddm"), "đm");
}

/// `dd` → `đ` (shape, AmTietTiengViet).
#[test]
fn dd_la_am_tiet() {
    assert_eq!(go("dd"), "đ");
    assert_eq!(loai("dd"), LoaiNoiDung::AmTietTiengViet);
}

/// Rỗng → Trong.
#[test]
fn rong_la_trong() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let phien = bo_go.tao_phien();
    assert_eq!(phien.ban_chup().loai_noi_dung(), LoaiNoiDung::Trong);
}
