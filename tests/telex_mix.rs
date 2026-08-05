// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test mixed raw + Telex: them_nguyen_ban tạo ranh giới đoạn độc lập.

use cadence::{BoGo, CauHinh};

fn phien() -> cadence::PhienGo {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    bo_go.tao_phien()
}

/// Telex + raw + Telex: `as` + raw `x` + `as` → `áxá`.
#[test]
fn telex_nguyen_ban_telex_hai_doan() {
    let mut phien = phien();
    for c in "as".chars() {
        phien.them_ky_tu(c);
    }
    phien.them_nguyen_ban('x');
    for c in "as".chars() {
        phien.them_ky_tu(c);
    }
    assert_eq!(phien.ban_chup().noi_dung(), "áxá");
}

/// Raw + Telex: raw `x` + `as` → `xá`.
#[test]
fn nguyen_ban_roi_telex() {
    let mut phien = phien();
    phien.them_nguyen_ban('x');
    for c in "as".chars() {
        phien.them_ky_tu(c);
    }
    assert_eq!(phien.ban_chup().noi_dung(), "xá");
}

/// Raw chặn tone xuyên: `a` + raw `ế` + `s` → `aếs`.
#[test]
fn nguyen_ban_chan_tone_xuyen() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('ế');
    phien.them_ky_tu('s');
    assert_eq!(phien.ban_chup().noi_dung(), "aếs");
}

/// Raw chặn shape: `a` + raw `a` → `aa` (không `â`).
#[test]
fn nguyen_ban_chan_shape() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('a');
    assert_eq!(phien.ban_chup().noi_dung(), "aa");
}

/// Raw `w` giữa hai `a`: `a` + raw `w` + `a` → `awa`.
#[test]
fn nguyen_ban_w_giua_hai_a() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('w');
    phien.them_ky_tu('a');
    assert_eq!(phien.ban_chup().noi_dung(), "awa");
}

/// Backspace xuyên ranh giới raw: `as` + raw `x` → backspace → `á`.
#[test]
fn backspace_xoa_qua_ranh_gioi_nguyen_ban() {
    let mut phien = phien();
    for c in "as".chars() {
        phien.them_ky_tu(c);
    }
    phien.them_nguyen_ban('x');
    assert_eq!(phien.ban_chup().noi_dung(), "áx");
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "á");
}

/// Nhiều ranh giới raw: `as` + raw `x` + `as` + raw `y` + `as` → `áxáyás`.
#[test]
fn nhieu_ranh_gioi_nguyen_ban() {
    let mut phien = phien();
    for c in "as".chars() {
        phien.them_ky_tu(c);
    }
    phien.them_nguyen_ban('x');
    for c in "as".chars() {
        phien.them_ky_tu(c);
    }
    phien.them_nguyen_ban('y');
    for c in "as".chars() {
        phien.them_ky_tu(c);
    }
    assert_eq!(phien.ban_chup().noi_dung(), "áxáyá");
}

/// Raw với emoji: `as` + raw `😀` + `as` → `á😀á`.
#[test]
fn nguyen_ban_voi_emoji() {
    let mut phien = phien();
    for c in "as".chars() {
        phien.them_ky_tu(c);
    }
    phien.them_nguyen_ban('😀');
    for c in "as".chars() {
        phien.them_ky_tu(c);
    }
    assert_eq!(phien.ban_chup().noi_dung(), "á😀á");
}

/// Raw chặn `dd` shape: `d` + raw `d` → `dd` (không `đ`).
#[test]
fn nguyen_ban_chan_dd_shape() {
    let mut phien = phien();
    phien.them_ky_tu('d');
    phien.them_nguyen_ban('d');
    assert_eq!(phien.ban_chup().noi_dung(), "dd");
}

/// Tone key sau raw không ảnh hưởng nguyên âm trước raw: `as` + raw `x` + `f` → `àxf`.
#[test]
fn tone_sau_raw_khong_xuyen() {
    let mut phien = phien();
    for c in "as".chars() {
        phien.them_ky_tu(c);
    }
    phien.them_nguyen_ban('x');
    phien.them_ky_tu('f');
    // `f` là tone key nhưng không có nguyên âm trong đoạn sau raw `x`.
    // `f` không xuyên về `á` → `à`. Chỉ `x` + `f` literal.
    let kq = phien.ban_chup().noi_dung();
    assert!(kq.starts_with("á"), "kỳ vọng á ở đầu, được {kq}");
}
