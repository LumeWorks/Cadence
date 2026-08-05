// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus acceptance tests Phase 3 - "Gõ mọi thứ bạn cần".
//!
//! Test end-to-end qua `PhienGo::them_ky_tu` cho từng DoD category:
//! identifier, URL/email/path, code, emoticon, teencode, tiếng Việt, trộn.

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

// ── Identifier ─────────────────────────────────────────────────────────

#[test]
fn id_async() {
    assert_eq!(go("async"), "async");
    assert_eq!(loai("async"), LoaiNoiDung::NguyenBan);
}

#[test]
fn id_class() {
    assert_eq!(go("class"), "class");
}

#[test]
fn id_struct() {
    assert_eq!(go("struct"), "struct");
}

#[test]
fn id_snake_case() {
    assert_eq!(go("user_id"), "user_id");
}

#[test]
fn id_camel_case() {
    assert_eq!(go("userName"), "userName");
}

#[test]
fn id_pascal_case() {
    assert_eq!(go("UserName"), "UserName");
}

#[test]
fn id_screaming_snake() {
    assert_eq!(go("SCREAMING_SNAKE_CASE"), "SCREAMING_SNAKE_CASE");
}

#[test]
fn id_acronym_prefix() {
    assert_eq!(go("HTTPServer"), "HTTPServer");
}

// ── URL / email / path ─────────────────────────────────────────────────

#[test]
fn url_https() {
    assert_eq!(go("https://example.com"), "https://example.com");
    assert_eq!(loai("https://example.com"), LoaiNoiDung::NguyenBan);
}

#[test]
fn url_http_localhost() {
    assert_eq!(go("http://localhost:3000"), "http://localhost:3000");
}

#[test]
fn url_with_query() {
    assert_eq!(
        go("https://example.com?q=tiếng"),
        "https://example.com?q=tiếng"
    );
}

#[test]
fn email() {
    assert_eq!(go("name@example.com"), "name@example.com");
}

#[test]
fn path_unix_absolute() {
    assert_eq!(go("/home/minh/docs"), "/home/minh/docs");
}

#[test]
fn path_tilde() {
    assert_eq!(go("~/Documents/Cadence"), "~/Documents/Cadence");
}

#[test]
fn path_relative() {
    assert_eq!(go("./install.sh"), "./install.sh");
}

#[test]
fn path_windows() {
    assert_eq!(go(r"C:\Users\Name"), r"C:\Users\Name");
}

#[test]
fn ip_port() {
    assert_eq!(go("127.0.0.1:8080"), "127.0.0.1:8080");
}

#[test]
fn version() {
    assert_eq!(go("v1.2.3"), "v1.2.3");
}

#[test]
fn git_hash() {
    assert_eq!(go("c9868e1"), "c9868e1");
}

#[test]
fn uuid() {
    assert_eq!(
        go("550e8400-e29b-41d4-a716-446655440000"),
        "550e8400-e29b-41d4-a716-446655440000"
    );
}

// ── Code ───────────────────────────────────────────────────────────────

#[test]
fn code_namespace() {
    assert_eq!(go("foo::bar"), "foo::bar");
}

#[test]
fn code_let_assignment() {
    assert_eq!(
        go("let mut buf = String::new();"),
        "let mut buf = String::new();"
    );
}

#[test]
fn code_cargo_build() {
    assert_eq!(go("cargo build --release"), "cargo build --release");
}

#[test]
fn code_fn_main() {
    assert_eq!(go("fn main() {}"), "fn main() {}");
}

// ── Emoticon ───────────────────────────────────────────────────────────

#[test]
fn emoticon_paren() {
    assert_eq!(go("=))"), "=))");
}

#[test]
fn emoticon_long_paren() {
    assert_eq!(go("=))))))))))))"), "=))))))))))))");
}

#[test]
fn emoticon_face() {
    assert_eq!(go(":v"), ":v");
}

#[test]
fn emoticon_question_repeat() {
    assert_eq!(go("???"), "???");
}

#[test]
fn emoticon_bang_repeat() {
    assert_eq!(go("!!!!!!!"), "!!!!!!!");
}

// ── Teencode ───────────────────────────────────────────────────────────

#[test]
fn teencode_broooo() {
    assert_eq!(go("brooooo"), "brooooo");
}

#[test]
fn teencode_vcl() {
    assert_eq!(go("vcl"), "vcl");
}

#[test]
fn teencode_ko() {
    assert_eq!(go("ko"), "ko");
}

#[test]
fn teencode_dc() {
    assert_eq!(go("dc"), "dc");
}

#[test]
fn teencode_ddm() {
    assert_eq!(go("ddm"), "đm");
}

// ── Tiếng Việt ─────────────────────────────────────────────────────────

#[test]
fn tv_tieengs() {
    assert_eq!(go("tieengs"), "tiếng");
    assert_eq!(loai("tieengs"), LoaiNoiDung::AmTietTiengViet);
}

#[test]
fn tv_nguowif() {
    assert_eq!(go("nguowif"), "người");
}

#[test]
fn tv_dduwowngf() {
    assert_eq!(go("dduwowngf"), "đường");
}

#[test]
fn tv_aa_hoa() {
    assert_eq!(go("AA"), "Â");
}

#[test]
fn tv_dd_hoa() {
    assert_eq!(go("DD"), "Đ");
}

// ── Trộn ───────────────────────────────────────────────────────────────

#[test]
fn tron_cargo_loi_emoticon() {
    assert_eq!(go("cargo build lỗi rồi =))"), "cargo build lỗi rồi =))");
}

#[test]
fn tron_let_tieng_viet() {
    assert_eq!(
        go("let ten_nguoi_dung = \"Minh\";"),
        "let ten_nguoi_dung = \"Minh\";"
    );
}

#[test]
fn tron_user_id_hoi() {
    assert_eq!(go("user_id của m là gì?"), "user_id của m là gì?");
}

#[test]
fn tron_broooo_hoi() {
    assert_eq!(
        go("brooooo m đang làm gì đấy???"),
        "brooooo m đang làm gì đấy???"
    );
}
