// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus adversarial - input độc, boundary, storm. Không panic, không treo,
//! không invariant failure. Giới hạn phiên đặt 4096 cho mọi test.
//!
//! Liên kết branch: `phien_go.rs` (giới hạn, cursor), `phan_doan.rs`,
//! `ngu_canh.rs`, `kieu_go/telex.rs` (escape loop), `vi_tri.rs` (grapheme storm).

use cadence::{BoGo, CauHinh, KetQuaXuLy, PhienGo};
use unicode_segmentation::UnicodeSegmentation;

fn phien() -> PhienGo {
    let mut c = CauHinh::mac_dinh();
    c.dat_gioi_han_thao_tac(4096).expect("4096 hop le");
    BoGo::new(c).expect("cau hinh hop le").tao_phien()
}

fn nhap(p: &mut PhienGo, s: &str) {
    for ch in s.chars() {
        p.them_ky_tu(ch);
    }
}

/// 128 tone modifiers liên tiếp - không panic, không treo.
#[test]
fn _128_tone_modifiers() {
    let mut p = phien();
    p.them_ky_tu('a');
    for _ in 0..128 {
        p.them_ky_tu('s');
    }
    let bc = p.ban_chup();
    let g = bc.noi_dung().graphemes(true).count();
    // Một nguyên âm + tone storm → ít grapheme (escape xen kẽ).
    assert!(g <= 130, "grapheme {g}");
    assert!(bc.con_tro().chi_so_byte() <= bc.noi_dung().len());
}

/// 128 shape modifiers liên tiếp.
#[test]
fn _128_shape_modifiers() {
    let mut p = phien();
    p.them_ky_tu('a');
    for _ in 0..128 {
        p.them_ky_tu('w');
    }
    let bc = p.ban_chup();
    assert!(bc.con_tro().chi_so_byte() <= bc.noi_dung().len());
    // Raw giữ nguyên.
    assert_eq!(bc.noi_dung_goc().chars().count(), 129);
}

/// 128 dấu `)` sau `=`.
#[test]
fn _128_dau_ngoac() {
    let mut p = phien();
    p.them_ky_tu('=');
    for _ in 0..128 {
        p.them_ky_tu(')');
    }
    let bc = p.ban_chup();
    assert_eq!(bc.noi_dung(), bc.noi_dung_goc());
    assert!(bc.noi_dung().starts_with("=)"));
}

/// 128 dấu `?`.
#[test]
fn _128_dau_hoi() {
    let mut p = phien();
    for _ in 0..128 {
        p.them_ky_tu('?');
    }
    let bc = p.ban_chup();
    assert_eq!(bc.noi_dung(), "?".repeat(128));
}

/// 128 combining marks sau `e` - một grapheme lớn, không tách.
#[test]
fn _128_combining_marks() {
    let mut p = phien();
    p.them_ky_tu('e');
    for _ in 0..128 {
        p.them_ky_tu('\u{0301}');
    }
    let bc = p.ban_chup();
    // Toàn bộ là một grapheme (base + 128 combining).
    assert_eq!(
        bc.noi_dung().graphemes(true).count(),
        1,
        "combining storm mot grapheme"
    );
    // Byte index là boundary.
    assert!(bc.noi_dung().is_char_boundary(bc.con_tro().chi_so_byte()));
}

/// Emoji ZWJ dài (5 emoji nối).
#[test]
fn emoji_zwj_dai() {
    let mut p = phien();
    // 👨‍👩‍👧‍👦‍👧 = 5 emoji + 4 ZWJ.
    let seq = [
        '\u{1F468}',
        '\u{200D}',
        '\u{1F469}',
        '\u{200D}',
        '\u{1F467}',
        '\u{200D}',
        '\u{1F466}',
        '\u{200D}',
        '\u{1F467}',
    ];
    for c in seq {
        p.them_ky_tu(c);
    }
    let bc = p.ban_chup();
    assert_eq!(
        bc.noi_dung().graphemes(true).count(),
        1,
        "ZWJ dai mot grapheme"
    );
}

/// Markdown fence chưa đóng - không treo, content Telex nếu hợp lệ.
#[test]
fn markdown_fence_chua_dong() {
    let mut p = phien();
    nhap(&mut p, "```as");
    // Backtick mở raw, `as` Telex → `á`.
    assert_eq!(p.ban_chup().noi_dung(), "```á");
}

/// Chuỗi delimiter chưa đóng dài - không treo, raw giữ.
#[test]
fn delimiter_chua_dong_dai() {
    let mut p = phien();
    nhap(&mut p, "```rust\nlet x = 1;\n// chua dong");
    let bc = p.ban_chup();
    // Code fence chưa đóng: `rust` là đoạn Chu tự do → có thể biến đổi (`rút`).
    // Bất biến: không panic, không treo, raw giữ byte-for-byte.
    assert_eq!(bc.noi_dung_goc(), "```rust\nlet x = 1;\n// chua dong");
    assert!(bc.noi_dung().starts_with("```"));
}

/// URL-like token cực dài trong giới hạn.
#[test]
fn url_like_cuc_dai() {
    let mut p = phien();
    let path = "a".repeat(200);
    let raw = format!("https://example.com/{path}");
    nhap(&mut p, &raw);
    assert_eq!(p.ban_chup().noi_dung(), raw);
}

/// Nhiều `_`.
#[test]
fn nhieu_gach_duoi() {
    let mut p = phien();
    nhap(&mut p, &"a".repeat(50));
    nhap(&mut p, &"_".repeat(50));
    nhap(&mut p, &"b".repeat(50));
    assert_eq!(p.ban_chup().noi_dung_goc().len(), 150);
}

/// Nhiều `::`.
#[test]
fn nhieu_namespace() {
    let mut p = phien();
    let raw = "foo::bar::baz::qux::quux::corge::grault::garply::waldo::fred";
    nhap(&mut p, raw);
    assert_eq!(p.ban_chup().noi_dung(), raw);
}

/// Nhiều `://`.
#[test]
fn nhieu_scheme() {
    let mut p = phien();
    nhap(&mut p, "https://a.com http://b.com ftp://c.com");
    assert_eq!(
        p.ban_chup().noi_dung(),
        "https://a.com http://b.com ftp://c.com"
    );
}

/// Nhiều `@`.
#[test]
fn nhieu_a_cong() {
    let mut p = phien();
    nhap(&mut p, "a@b.com x@y.com p@q.com");
    assert_eq!(p.ban_chup().noi_dung(), "a@b.com x@y.com p@q.com");
}

/// Chuỗi mixed NFC/NFD - không panic, canonical stable.
#[test]
fn mixed_nfc_nfd() {
    let mut p = phien();
    p.them_ky_tu('ế'); // NFC precomposed
    p.them_ky_tu('e');
    p.them_ky_tu('\u{0302}'); // combining circumflex (NFD-ish ê)
    p.them_ky_tu('\u{0301}'); // combining acute
    p.them_ky_tu('đ');
    let bc = p.ban_chup();
    // Không panic; byte index boundary.
    assert!(bc.noi_dung().is_char_boundary(bc.con_tro().chi_so_byte()));
}

/// Chèn/xóa lặp ở giữa - không panic, state nhất quán.
#[test]
fn chen_xoa_lap_o_giua() {
    let mut p = phien();
    nhap(&mut p, "abcdef");
    for _ in 0..50 {
        p.ve_dau();
        p.di_phai();
        p.them_ky_tu('x');
        p.xoa_lui();
    }
    // Sau 50 cặp chen-xoa, nội dung như cũ.
    assert_eq!(p.ban_chup().noi_dung(), "abcdef");
}

/// Di trái/phải liên tục - không loop vô hạn.
#[test]
fn di_chuyen_lien_tuc() {
    let mut p = phien();
    nhap(&mut p, "tieengs");
    for _ in 0..200 {
        let _ = p.di_trai();
    }
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 0);
    for _ in 0..200 {
        let _ = p.di_phai();
    }
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 5);
}

/// Commit/reset liên tục - không rò state.
#[test]
fn commit_reset_lien_tuc() {
    let mut p = phien();
    for i in 0..20 {
        nhap(&mut p, "ab");
        let kq = p.chap_nhan();
        assert!(matches!(kq, KetQuaXuLy::ChapNhan { .. }), "lan {i}");
        assert!(p.dang_trong());
        p.dat_lai();
    }
    p.them_ky_tu('z');
    assert_eq!(p.ban_chup().noi_dung(), "z");
}

/// Restore raw liên tục - idempotent.
#[test]
fn restore_raw_lien_tuc() {
    let mut p = phien();
    nhap(&mut p, "tieengs");
    let truoc = p.ban_chup().noi_dung().to_string();
    for _ in 0..20 {
        assert!(matches!(p.khoi_phuc_nguyen_ban(), KetQuaXuLy::KhongDoi));
    }
    assert_eq!(p.ban_chup().noi_dung(), truoc);
}

/// Vượt giới hạn không sửa state cũ.
#[test]
fn vuot_gioi_han_khong_sua_state() {
    let mut c = CauHinh::mac_dinh();
    c.dat_gioi_han_thao_tac(4).expect("4 hop le");
    let mut p = BoGo::new(c).expect("ok").tao_phien();
    nhap(&mut p, "abcd");
    let snap = p.ban_chup().noi_dung().to_string();
    // Vượt giới hạn.
    assert!(matches!(p.them_ky_tu('e'), KetQuaXuLy::KhongDoi));
    assert_eq!(p.ban_chup().noi_dung(), snap);
}

/// Mọi char 0..0xD800 không panic (BMP + một số supplementary).
#[test]
fn moi_char_bmp_khong_panic() {
    let mut p = phien();
    for cp in [
        0u32, 1, 0x20, 0x7F, 0xA0, 0x100, 0x1B0, 0x1EBF, 0x2028, 0x2029, 0x2060, 0xFE0F, 0x10FFFF,
    ] {
        if let Some(c) = char::from_u32(cp) {
            p.them_ky_tu(c);
            let bc = p.ban_chup();
            assert!(bc.con_tro().chi_so_byte() <= bc.noi_dung().len());
        }
    }
}
