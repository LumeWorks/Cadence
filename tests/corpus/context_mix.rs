// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus trộn context - code + tiếng Việt + URL + chat trong cùng phiên.
//! Liên kết branch: `phan_doan.rs`, `ngu_canh.rs`, `lua_chon.rs`.

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// Code trộn tiếng Việt - từng đoạn quyết định độc lập.
#[test]
fn code_tron_tieng_viet() {
    assert_eq!(go("cargo build lỗi rồi =))"), "cargo build lỗi rồi =))");
    assert_eq!(
        go("let ten_nguoi_dung = \"Minh\";"),
        "let ten_nguoi_dung = \"Minh\";"
    );
    assert_eq!(go("user_id của m là gì?"), "user_id của m là gì?");
    assert_eq!(
        go("brooooo m đang làm gì đấy???"),
        "brooooo m đang làm gì đấy???"
    );
}

/// URL cạnh tiếng Việt.
#[test]
fn url_canh_tieng_viet() {
    assert_eq!(go("xem https://example.com nhé"), "xem https://example.com nhé");
    assert_eq!(go("gửi cho name@example.com đi"), "gửi cho name@example.com đi");
}

/// Code fence + tiếng Việt bên trong.
#[test]
fn code_fence_tieng_viet_ben_trong() {
    let raw = "```rust\n// lỗi rồi\nlet x = 1;\n```";
    assert_eq!(go(raw), raw);
}

/// Teencode + tiếng Việt + emoji.
#[test]
fn teencode_tieng_viet_emoji() {
    assert_eq!(go("ko có gì 😀"), "ko có gì 😀");
    assert_eq!(go("vl bug nữa =))"), "vl bug nữa =))");
}

/// Namespace + tiếng Việt. `use` parse thành `úe` qua `them_ky_tu`; host
/// muốn raw dùng `them_nguyen_ban`.
#[test]
fn namespace_tieng_viet() {
    // `std::vec::Vec` raw vì `::` adjacency.
    assert_eq!(go("std::vec::Vec; nhé"), "std::vec::Vec; nhé");
    assert_eq!(go("foo::bar lỗi rồi"), "foo::bar lỗi rồi");
    // `use` → `úe` (âm tiết Việt hợp lệ).
    assert_eq!(go("use"), "úe");
    // Host giữ raw qua `them_nguyen_ban`.
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut p = bo_go.tao_phien();
    for c in "use std::vec::Vec;".chars() {
        p.them_nguyen_ban(c);
    }
    assert_eq!(p.ban_chup().noi_dung(), "use std::vec::Vec;");
}

/// Markdown + tiếng Việt.
#[test]
fn markdown_tieng_viet() {
    assert_eq!(go("# Tiêu đề"), "# Tiêu đề");
    assert_eq!(go("- mục một"), "- mục một");
    assert_eq!(go("**đậm**"), "**đậm**");
}

/// JSON/TOML + giá trị tiếng Việt.
#[test]
fn json_toml_tieng_viet() {
    assert_eq!(go("{\"ten\": \"Minh\"}"), "{\"ten\": \"Minh\"}");
    assert_eq!(go("name = \"Minh\""), "name = \"Minh\"");
}
