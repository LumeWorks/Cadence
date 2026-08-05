// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus command - shell option, flag, gán `=`, code span/fence.
//!
//! Lưu ý: `them_ky_tu` biến đổi đoạn `Chu` parse thành âm tiết Việt (vd
//! `rust`→`rút`, `test`→`tét`). Host muốn raw dùng `them_nguyen_ban`.
//! Liên kết branch: `ngu_canh.rs` (nhan_code, `=` adjacency).

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

fn go_raw(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_nguyen_ban(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// Command + flag → raw qua `them_nguyen_ban`.
#[test]
fn command_flag_raw() {
    let cases = [
        "cargo build --release",
        "git commit -m",
        "rustup run 1.85 cargo test",
        "ls -la --color=auto",
        "npm install --save-dev",
    ];
    for raw in cases {
        assert_eq!(go_raw(raw), raw, "{raw}");
    }
}

/// `cargo build --release` raw qua `them_ky_tu` (các segment đều raw: cargo, build).
#[test]
fn cargo_build_raw() {
    assert_eq!(go("cargo build --release"), "cargo build --release");
}

/// Phép gán `=` → raw (cả vế trước và sau `=` adjacency).
#[test]
fn gan_bang() {
    assert_eq!(go("x = y"), "x = y");
    assert_eq!(go("buf = String::new()"), "buf = String::new()");
    assert_eq!(go("let mut buf = x;"), "let mut buf = x;");
    assert_eq!(go("x=y"), "x=y");
    assert_eq!(go("const N = 10"), "const N = 10");
}

/// Code span/fence → raw.
#[test]
fn code_span_fence() {
    assert_eq!(go("`code`"), "`code`");
    assert_eq!(go_raw("```rust"), "```rust");
    assert_eq!(go_raw("```rust\ncode\n```"), "```rust\ncode\n```");
    // Code fence chưa đóng: backtick mở raw, content Telex nếu hợp lệ.
    assert_eq!(go("```as"), "```á");
}

/// Shell option + environment variable → raw qua `them_nguyen_ban`.
#[test]
fn shell_option_env_raw() {
    assert_eq!(go_raw("$HOME/bin"), "$HOME/bin");
    assert_eq!(go_raw("${PATH:-/usr/bin}"), "${PATH:-/usr/bin}");
    assert_eq!(go_raw("--features serde"), "--features serde");
    assert_eq!(go_raw("RUSTFLAGS=-D warnings"), "RUSTFLAGS=-D warnings");
}

/// MIME type + encoding name → raw qua `them_nguyen_ban` (`text` parse thành `tẽt`).
#[test]
fn mime_encoding_raw() {
    assert_eq!(go_raw("text/plain"), "text/plain");
    assert_eq!(go_raw("application/json"), "application/json");
    assert_eq!(go_raw("charset=utf-8"), "charset=utf-8");
    assert_eq!(go_raw("Content-Type: text/html"), "Content-Type: text/html");
}

/// `text` → `tẽt` qua `them_ky_tu` (âm tiết Việt hợp lệ).
#[test]
fn text_bien_doi_tet() {
    assert_eq!(go("text"), "tẽt");
    assert_eq!(go("test"), "tét");
}
