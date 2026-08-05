// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test nhận diện ngữ cảnh qua public API — URL, email, đường dẫn, code,
//! namespace `::`, phép gán `=`, emoticon đều buộc raw.

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

// ── URL ─────────────────────────────────────────────────────────────────

#[test]
fn url_https_raw() {
    assert_eq!(go("https://example.com/as"), "https://example.com/as");
}

#[test]
fn url_http_localhost_raw() {
    assert_eq!(go("http://localhost:3000"), "http://localhost:3000");
}

#[test]
fn url_co_query_raw() {
    assert_eq!(
        go("https://x.com/search?q=abc"),
        "https://x.com/search?q=abc"
    );
}

// ── Email ───────────────────────────────────────────────────────────────

#[test]
fn email_raw() {
    assert_eq!(go("test@as.com"), "test@as.com");
}

// ── Đường dẫn ───────────────────────────────────────────────────────────

#[test]
fn duong_dan_unix_raw() {
    assert_eq!(go("/home/as"), "/home/as");
}

#[test]
fn duong_dan_tilde_raw() {
    assert_eq!(go("~/Documents/as"), "~/Documents/as");
}

#[test]
fn duong_dan_windows_raw() {
    assert_eq!(go(r"C:\Users\as"), r"C:\Users\as");
}

// ── Namespace `::` ─────────────────────────────────────────────────────

#[test]
fn namespace_2_ngoac_raw() {
    assert_eq!(go("foo::bar"), "foo::bar");
}

#[test]
fn namespace_3_ngoac_raw() {
    assert_eq!(go("foo::bar::baz"), "foo::bar::baz");
}

// ── Phép gán `=` ───────────────────────────────────────────────────────

#[test]
fn gan_bang_chu_truoc_raw() {
    assert_eq!(go("buf = x"), "buf = x");
}

#[test]
fn gan_bang_chu_sau_raw() {
    assert_eq!(go("x = buf"), "x = buf");
}

#[test]
fn gan_bang_khong_cach_raw() {
    assert_eq!(go("x=y"), "x=y");
}

// ── Code span/fence ─────────────────────────────────────────────────────

#[test]
fn code_span_raw() {
    assert_eq!(go("`as`"), "`as`");
}

#[test]
fn code_fence_raw() {
    assert_eq!(go("```as```"), "```as```");
}

#[test]
fn code_fence_khong_dong() {
    // Chưa đóng → backtick mở raw, "as" không khóa → Telex ("á").
    assert_eq!(go("```as"), "```á");
}

// ── Emoticon ───────────────────────────────────────────────────────────

#[test]
fn emoticon_raw() {
    assert_eq!(go(":v"), ":v");
}

#[test]
fn emoticon_dai_raw() {
    assert_eq!(go("=))))"), "=))))");
}

#[test]
fn emoticon_d_mat_raw() {
    assert_eq!(go(":D"), ":D");
}

#[test]
fn emoticon_3_dau_hoi_raw() {
    assert_eq!(go("???"), "???");
}

// ── Chu thường không bị buộc raw ───────────────────────────────────────

#[test]
fn chu_thuong_telex() {
    assert_eq!(go("tieengs"), "tiếng");
}

#[test]
fn chu_don_le_raw_khi_khong_hop_le() {
    assert_eq!(go("async"), "async");
}

// ── `hoaf.com` không phải URL (không có `://`) ─────────────────────────

#[test]
fn hoaf_com_khong_phai_url() {
    // "hoaf.com" không có "://" → "hoaf" Telex (f=huyền) → "hòa".
    assert_eq!(go("hoaf.com"), "hòa.com");
}
