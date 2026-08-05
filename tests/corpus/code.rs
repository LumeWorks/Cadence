// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus code - nhiều ngôn ngữ, cấu trúc chung, không parser riêng.
//!
//! Lưu ý hành vi: `them_ky_tu` biến đổi đoạn `Chu` parse thành âm tiết Việt
//! hợp lệ, kể cả khi là từ tiếng Anh (vd `text`→`tẽt`, `use`→`úe`). Đây là
//! thiết kế "không phán xét" (RFC 0013). Host muốn giữ raw cho nội dung kỹ thuật
//! dùng `them_nguyen_ban` (xem `them_nguyen_ban_bao_toan_raw`).
//!
//! Liên kết branch: `phan_doan.rs` (LoaiDoan KyThuat), `ngu_canh.rs` (`::`, `=`).

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// `them_nguyen_ban` giữ raw cho mọi nội dung kỹ thuật (host mechanism).
fn go_raw(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_nguyen_ban(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// `them_ky_tu` biến đổi từ tiếng Anh trông như âm tiết Việt (hành vi đúng).
#[test]
fn them_ky_tu_bien_doi_tieng_anh_giong_viet() {
    // `text` = t + ẽ + t (âm tiết Việt hợp lệ) → `tẽt`.
    assert_eq!(go("text"), "tẽt");
    // `use` = u + sắc + e → `úe`.
    assert_eq!(go("use"), "úe");
    // `char` = ch + ả + r? → `chả`.
    assert_eq!(go("char"), "chả");
}

/// Identifier mọi kiểu case → raw qua `them_nguyen_ban`.
#[test]
fn identifier_case_raw() {
    let cases = [
        "snake_case",
        "camelCase",
        "PascalCase",
        "SCREAMING_SNAKE_CASE",
        "HTTPServer",
        "user_id",
        "userName",
        "MyClass",
        "CONST_VALUE",
    ];
    for raw in cases {
        assert_eq!(go_raw(raw), raw, "{raw}");
    }
}

/// Identifier staying raw qua `them_ky_tu` (không phải âm tiết Việt).
#[test]
fn identifier_raw_via_them_ky_tu() {
    // Các identifier này có onset không hợp lệ hoặc 2+ dấu thanh → raw.
    let cases = ["async", "class", "struct", "user_id", "HTTPServer"];
    for raw in cases {
        assert_eq!(go(raw), raw, "{raw}");
    }
}

/// Keyword ngôn ngữ khác → raw qua `them_nguyen_ban`.
#[test]
fn keyword_nhieu_ngo_ngu_raw() {
    let cases = [
        "fn",
        "let",
        "mut",
        "struct",
        "impl",
        "pub",
        "use",
        "match",
        "function",
        "return",
        "const",
        "class",
        "extends",
        "import",
        "def",
        "lambda",
        "nil",
        "func",
        "package",
        "interface",
        "type",
        "select",
        "go",
        "chan",
        "defer",
        "switch",
        "case",
        "public",
        "static",
        "void",
        "int",
        "char",
        "if",
        "else",
        "while",
        "begin",
        "end",
        "do",
        "then",
    ];
    for raw in cases {
        assert_eq!(go_raw(raw), raw, "keyword {raw}");
    }
}

/// Generic/lifetime/annotation → raw qua `them_nguyen_ban`.
#[test]
fn generic_lifetime_annotation_raw() {
    assert_eq!(go_raw("Vec<T>"), "Vec<T>");
    assert_eq!(go_raw("HashMap<K, V>"), "HashMap<K, V>");
    assert_eq!(go_raw("fn foo<'a>(x: &'a str)"), "fn foo<'a>(x: &'a str)");
    assert_eq!(go_raw("#[derive(Debug)]"), "#[derive(Debug)]");
    assert_eq!(go_raw("@Override"), "@Override");
    assert_eq!(go_raw("@decorator(arg)"), "@decorator(arg)");
}

/// Comment/string nhiều ngôn ngữ → raw qua `them_nguyen_ban`.
#[test]
fn comment_string_raw() {
    assert_eq!(go_raw("// comment"), "// comment");
    assert_eq!(go_raw("# comment"), "# comment");
    assert_eq!(go_raw("/* block */"), "/* block */");
    assert_eq!(go_raw("-- sql comment"), "-- sql comment");
    assert_eq!(go_raw("\"string\""), "\"string\"");
    assert_eq!(go_raw("'char'"), "'char'");
    assert_eq!(go_raw("`backtick`"), "`backtick`");
}

/// Escape sequence → raw (qua `them_nguyen_ban`).
#[test]
fn escape_sequence_raw() {
    assert_eq!(go_raw(r"\n"), r"\n");
    assert_eq!(go_raw(r"\t"), r"\t");
    assert_eq!(go_raw(r#"\"quote\""#), r#"\"quote\""#);
}

/// Operator + number literal → raw (số/kỹ thuật tự raw).
#[test]
fn operator_number() {
    assert_eq!(go("a + b"), "a + b");
    assert_eq!(go("x == 1"), "x == 1");
    assert_eq!(go("0x1F"), "0x1F");
    assert_eq!(go("1_000_000"), "1_000_000");
    assert_eq!(go("3.14"), "3.14");
    assert_eq!(go("1e10"), "1e10");
}

/// Namespace `::` → raw (adjacency `::` buộc raw).
#[test]
fn namespace() {
    assert_eq!(go("std::vec::Vec"), "std::vec::Vec");
    assert_eq!(go("foo::bar::baz"), "foo::bar::baz");
    assert_eq!(go("::std::io"), "::std::io");
    assert_eq!(go("crate::foo"), "crate::foo");
    assert_eq!(go("super::bar"), "super::bar");
}

/// File extension sau dấu `.` → raw.
#[test]
fn file_extension() {
    assert_eq!(go("main.rs"), "main.rs");
    assert_eq!(go("index.ts"), "index.ts");
    assert_eq!(go("app.py"), "app.py");
    assert_eq!(go("Cargo.toml"), "Cargo.toml");
}
