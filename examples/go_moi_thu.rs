// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Ví dụ Phase 3 — "Gõ mọi thứ bạn cần".
//!
//! Demo trộn code, URL, tiếng Việt, teencode, emoticon trong cùng phiên
//! mà không cần bật/tắt bộ gõ. Mỗi dòng gõ một loại nội dung, in ra output
//! cho thấy Cadence tự quyết định đoạn nào biến đổi Telex, đoạn nào raw.

use cadence::{BoGo, CauHinh, ChinhSachLuaChon};

fn go(bo_go: &BoGo, raw: &str) -> String {
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

fn main() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh luon hop le");

    println!("=== Phase 3 — Gõ mọi thứ bạn cần ===\n");

    // Tiếng Việt thuần — Telex biến đổi.
    println!("Tiếng Việt:");
    for raw in ["tieengs", "nguowif", "dduwowngf"] {
        println!("  {raw:>12} → {}", go(&bo_go, raw));
    }

    // Code — raw, không biến đổi.
    println!("\nCode:");
    for raw in ["async", "class", "user_id", "foo::bar", "let mut buf = x;"] {
        println!("  {raw:>20} → {}", go(&bo_go, raw));
    }

    // URL / email / path — raw.
    println!("\nURL / email / path:");
    for raw in [
        "https://example.com",
        "name@example.com",
        "~/Documents/Cadence",
        "127.0.0.1:8080",
    ] {
        println!("  {raw:>24} → {}", go(&bo_go, raw));
    }

    // Teencode / emoticon — raw.
    println!("\nTeencode / emoticon:");
    for raw in ["brooooo", "vcl", "=))))", "???"] {
        println!("  {raw:>12} → {}", go(&bo_go, raw));
    }

    // Trộn — từng đoạn quyết định độc lập.
    println!("\nTrộn:");
    for raw in [
        "cargo build lỗi rồi =))",
        "user_id của m là gì?",
        "brooooo m đang làm gì đấy???",
    ] {
        println!("  {raw:>32} → {}", go(&bo_go, raw));
    }

    // Chính sách lựa chọn — `UuTienTiengViet` cho phép Telex trong đoạn mơ hồ.
    println!("\nChính sách UuTienTiengViet:");
    let mut ch = CauHinh::mac_dinh();
    ch.dat_chinh_sach_lua_chon(ChinhSachLuaChon::UuTienTiengViet);
    let bo_utv = BoGo::new(ch).expect("cau hinh hop le");
    for raw in ["async", "cargo", "tieengs"] {
        println!("  {raw:>12} → {}", go(&bo_utv, raw));
    }
}
