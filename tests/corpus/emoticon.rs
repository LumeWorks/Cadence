// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus emoticon + emoji - bảo toàn raw, grapheme cluster đúng.
//! Liên kết branch: `ngu_canh.rs::nhan_emoticon`, `phan_doan.rs::LoaiDoan::Emoji`.

use cadence::{BoGo, CauHinh};
use unicode_segmentation::UnicodeSegmentation;

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// Emoticon mặt → raw.
#[test]
fn emoticon_mat_raw() {
    let cases = [":)", ":(", ":D", ":P", ":v", ":3", ";)", "^^", "=)", ":/", ":>", ":<"];
    for raw in cases {
        assert_eq!(go(raw), raw, "emoticon {raw}");
    }
}

/// Emoticon `=` + `)` lặp dài → raw.
#[test]
fn emoticon_dai_raw() {
    assert_eq!(go("=))"), "=))");
    assert_eq!(go("=))))"), "=))))");
    assert_eq!(go("=))))))))))))"), "=))))))))))))");
}

/// `?`/`!`/`.` lặp 3+ → raw (emoticon cảm xúc).
#[test]
fn dau_lap_emoticon_raw() {
    assert_eq!(go("???"), "???");
    assert_eq!(go("!!!!!!!"), "!!!!!!!");
    assert_eq!(go("..."), "...");
    assert_eq!(go("???!!!"), "???!!!");
}

/// Emoji đơn, ngoài BMP, skin tone, ZWJ → raw, grapheme đúng.
#[test]
fn emoji_grapheme_dung() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");

    // Emoji đơn ngoài BMP.
    let mut phien = bo_go.tao_phien();
    phien.them_ky_tu('😀');
    let bc = phien.ban_chup();
    assert_eq!(bc.noi_dung(), "😀");
    assert_eq!(bc.con_tro().chi_so_grapheme(), 1);
    assert_eq!(bc.con_tro().chi_so_utf16(), 2);

    // Emoji + variation selector.
    let mut phien = bo_go.tao_phien();
    phien.them_ky_tu('\u{2764}');
    phien.them_ky_tu('\u{FE0F}');
    let bc = phien.ban_chup();
    assert_eq!(bc.noi_dung(), "\u{2764}\u{FE0F}");
    assert_eq!(bc.con_tro().chi_so_grapheme(), 1, "VS16 mot grapheme");

    // Emoji ZWJ sequence: 👨‍👩‍👧 là một grapheme.
    let mut phien = bo_go.tao_phien();
    for c in ['\u{1F468}', '\u{200D}', '\u{1F469}', '\u{200D}', '\u{1F467}'] {
        phien.them_ky_tu(c);
    }
    let bc = phien.ban_chup();
    assert_eq!(bc.noi_dung().graphemes(true).count(), 1, "ZWJ sequence mot grapheme");

    // Cờ (regional indicator pair) - một grapheme.
    let mut phien = bo_go.tao_phien();
    phien.them_ky_tu('\u{1F1FB}'); // V
    phien.them_ky_tu('\u{1F1F3}'); // N
    let bc = phien.ban_chup();
    assert_eq!(bc.noi_dung().graphemes(true).count(), 1, "co VN mot grapheme");
}

/// Emoji skin tone ( modifiers) → grapheme đúng.
#[test]
fn emoji_skin_tone() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    // 👍 + skin tone modifier U+1F3FB.
    phien.them_ky_tu('\u{1F44D}');
    phien.them_ky_tu('\u{1F3FB}');
    let bc = phien.ban_chup();
    assert_eq!(bc.noi_dung().graphemes(true).count(), 1, "skin tone mot grapheme");
    // Raw giữ nguyên.
    assert_eq!(bc.noi_dung_goc(), "\u{1F44D}\u{1F3FB}");
}

/// Trộn emoticon + tiếng Việt.
#[test]
fn tron_emoticon_tieng_viet() {
    assert_eq!(go("lỗi rồi =))"), "lỗi rồi =))");
    assert_eq!(go("hay quá :D"), "hay quá :D");
}
