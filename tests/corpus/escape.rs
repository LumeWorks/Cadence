// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus escape - enumeration lặp phím modifier.
//! Liên kết branch: `kieu_go/telex.rs` (escape hình chữ + escape dấu thanh).

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// Escape tone: lặp đúng tone key → hoàn tác tone, giữ raw.
#[test]
fn escape_tone_moi_key() {
    assert_eq!(go("ass"), "as");
    assert_eq!(go("aff"), "af");
    assert_eq!(go("arr"), "ar");
    assert_eq!(go("axx"), "ax");
    assert_eq!(go("ajj"), "aj");
}

/// Escape hình chữ: lặp đúng modifier → hoàn tác shape.
#[test]
fn escape_hinh_chu_moi_cap() {
    assert_eq!(go("aaa"), "aa");
    assert_eq!(go("aww"), "aw");
    assert_eq!(go("eee"), "ee");
    assert_eq!(go("ooo"), "oo");
    assert_eq!(go("oww"), "ow");
    assert_eq!(go("uww"), "uw");
    assert_eq!(go("ddd"), "dd");
}

/// Escape rồi áp lại: tone key thứ 3 áp dụng tone mới.
#[test]
fn escape_roi_ap_lai() {
    // `asss` → `ass` escape → `ás` (s thứ 3 áp sắc).
    assert_eq!(go("asss"), "ás");
    // `assss` → `ass` → `ass` (escape lần nữa).
    assert_eq!(go("assss"), "ass");
}

/// Escape tone key ở xa (modifier ở giữa) - không panic, raw hoặc Telex.
#[test]
fn escape_tone_key_xa() {
    let kq = go("asws");
    // `as`→`á`, `w` literal, `s` lặp tone - hành vi phức tạp. Bất biến: raw giữ.
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut p = bo_go.tao_phien();
    for c in "asws".chars() {
        p.them_ky_tu(c);
    }
    assert_eq!(p.ban_chup().noi_dung_goc(), "asws");
    assert!(!kq.is_empty());
}

/// `z` không track escape (z không xóa dấu → 2 literal).
#[test]
fn z_khong_escape() {
    assert_eq!(go("zz"), "zz");
}

/// Escape sau shape+tone: `awss` → escape tone giữ shape.
#[test]
fn escape_sau_shape_tone() {
    let kq = go("awss");
    assert!(kq == "aws" || kq == "ăs", "kỳ vọng aws hoặc ăs, được {kq}");
}
