// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus hình chữ - enumeration mọi cặp (base, modifier) → (ChuGoc, DauChu).
//! Liên kết branch: `kieu_go/telex.rs::cap_hinh_chu`, `kieu_go/render.rs::nguyen_am_nfc`.

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// Mọi cặp hình chữ hợp lệ → đúng ký tự dựng sẵn.
#[test]
fn moi_cap_hinh_chu() {
    let cases = [
        ("aa", "â"),
        ("aw", "ă"),
        ("ee", "ê"),
        ("oo", "ô"),
        ("ow", "ơ"),
        ("uw", "ư"),
        ("dd", "đ"),
    ];
    for (raw, exp) in cases {
        assert_eq!(go(raw), exp, "{raw}");
    }
}

/// Modifier không hợp lệ → giữ nguyên (raw).
#[test]
fn modifier_khong_hop_le_giu_nguyen() {
    // `iw`, `yw` không trong bảng cap_hinh_chu → `w` là literal.
    assert_eq!(go("iw"), "iw");
    assert_eq!(go("yw"), "yw");
    // `aw` hợp lệ nhưng `bw` không (b không là base).
    assert_eq!(go("bw"), "bw");
}

/// Hình chữ hoa: hoa theo ký tự gốc đầu tiên.
#[test]
fn hinh_chu_hoa() {
    let cases = [
        ("AA", "Â"),
        ("AW", "Ă"),
        ("EE", "Ê"),
        ("OO", "Ô"),
        ("OW", "Ơ"),
        ("UW", "Ư"),
        ("DD", "Đ"),
        ("Aa", "Â"),
        ("aA", "â"),
        ("Dd", "Đ"),
        ("dD", "đ"),
    ];
    for (raw, exp) in cases {
        assert_eq!(go(raw), exp, "{raw}");
    }
}

/// `uo` + `w` → `ươ` (w biến đổi cả u và o).
#[test]
fn uo_w_thanh_uo_horn() {
    // `uow` → `ươ` = ư(U+1B0) + ơ(U+01A1).
    assert_eq!(go("uow"), "\u{1B0}\u{01A1}");
    // `ow` đơn thuần → `ơ` (không ảnh hưởng u).
    assert_eq!(go("ow"), "ơ");
}

/// Escape hình chữ: lặp modifier → hoàn tác, giữ raw.
#[test]
fn escape_hinh_chu_lap_modifier() {
    let cases = [
        ("aaa", "aa"),
        ("aww", "aw"),
        ("eee", "ee"),
        ("ooo", "oo"),
        ("oww", "ow"),
        ("uww", "uw"),
        ("ddd", "dd"),
    ];
    for (raw, exp) in cases {
        assert_eq!(go(raw), exp, "{raw}");
    }
}

/// `phan_tich_ky_tu` round-trip: ký tự dựng sẵn → phân tích → render lại.
#[test]
fn phan_tich_ky_tu_round_trip() {
    // Gõ ký tự dựng sẵn trực tiếp → giữ nguyên (đã là NFC).
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    for c in ['â', 'ă', 'ê', 'ô', 'ơ', 'ư', 'đ', 'ế', 'ờ', 'ữ'] {
        let mut phien = bo_go.tao_phien();
        phien.them_ky_tu(c);
        assert_eq!(phien.ban_chup().noi_dung(), c.to_string(), "ky tu {c}");
    }
    // `ườ` là hai codepoint (ư + ờ) - gõ tuần tự giữ nguyên.
    let mut phien = bo_go.tao_phien();
    phien.them_ky_tu('ư');
    phien.them_ky_tu('ờ');
    assert_eq!(phien.ban_chup().noi_dung(), "\u{1B0}\u{1EDD}");
}
