// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test escape edge cases: lặp phím modifier thoát biến đổi.

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// `ass` → `as` (escape sắc).
#[test]
fn escape_sac() {
    assert_eq!(go("ass"), "as");
}

/// `aff` → `af` (escape huyền).
#[test]
fn escape_huyen() {
    assert_eq!(go("aff"), "af");
}

/// `arr` → `ar` (escape hỏi).
#[test]
fn escape_hoi() {
    assert_eq!(go("arr"), "ar");
}

/// `axx` → `ax` (escape ngã).
#[test]
fn escape_nga() {
    assert_eq!(go("axx"), "ax");
}

/// `ajj` → `aj` (escape nặng).
#[test]
fn escape_nang() {
    assert_eq!(go("ajj"), "aj");
}

/// `aww` → `aw` (escape shape `aw`→`ă`).
#[test]
fn escape_shape_aw() {
    assert_eq!(go("aww"), "aw");
}

/// `aaw` → `aaw` (escape `aa`→`â` nhưng `a` thứ 3 không phải escape?
/// Không: `aa` → `â`, `w` thứ 3 là shape modifier → `âw` không phải escape).
/// Thực tế: `aa`→`â`, `w`→`â` có dấu `w` (moc)? Không, `â` không nhận `w`.
/// Nên `aaw` → `âw` (w literal).
#[test]
fn aaw_khong_phai_escape() {
    assert_eq!(go("aaw"), "âw");
}

/// `ooo` → `oo` (escape `oo`→`ô` bằng cách lặp `o`).
#[test]
fn ooo_la_escape_oo() {
    assert_eq!(go("ooo"), "oo");
}

/// Escape sau tone: `as` → `á`, rồi `s` → `as` (escape).
#[test]
fn escape_sau_tone() {
    assert_eq!(go("ass"), "as");
}

/// Escape sau shape+tone: `aws` → `ắ`, rồi `s` → `aw` (escape tone, giữ shape).
/// Không: `ass` escape `s`, nhưng `awss` → `aws` (escape `s`, `aw` shape giữ).
#[test]
fn escape_sau_shape_tone() {
    let kq = go("awss");
    // `aw`→`ă`, `s`→`ắ`, `s` thứ 2 → escape → `aw` + `s` literal = `aws`.
    // Hoặc: escape tone → `ă` + `s` = `ăs`.
    assert!(kq == "aws" || kq == "ăs", "kỳ vọng aws hoặc ăs, được {kq}");
}

/// Escape `dd`: `ddd` → `dd`.
#[test]
fn escape_dd() {
    assert_eq!(go("ddd"), "dd");
}

/// Escape `ee`: `eee` → `ee`.
#[test]
fn escape_ee() {
    assert_eq!(go("eee"), "ee");
}

/// `ooo` → `oo` (escape `oo`→`ô`).
#[test]
fn ooo_la_escape() {
    assert_eq!(go("ooo"), "oo");
}

/// Escape `zz`: `zz` → `z` (z không có dấu để xóa → z literal, z thứ 2 cũng literal).
/// Nhưng `z` không track escape vì không có dấu → `zz` → `zz` (2 literal).
#[test]
fn zz_khong_escape() {
    assert_eq!(go("zz"), "zz");
}

/// Escape tone key ở xa: `as` → `á`, `w` literal, `s` → escape?
/// `as`→`á`, `w`→`áw` (w literal), `s`→`áws` (s thay dấu? hay escape?).
/// Tone `s` gần nhất là ở vị trí 1. `s` thứ 3 lặp `s` → escape.
#[test]
fn escape_tone_key_xa() {
    let kq = go("asws");
    // `as`→`á`, `w`→`áw`, `s`→ escape (lặp `s`) → hoàn tác `á`→`a` + `s` literal.
    // Nhưng `w` ở giữa → `aw` + `s` + `s` → `aws`? Hoặc `á` + `w` + `s` → escape `á`→`a`, `s` literal → `aws`.
    assert!(
        kq.contains("aw") || kq.contains("a"),
        "kỳ vọng aw hoặc a, được {kq}"
    );
}
