// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Phân đoạn lịch sử thao tác theo loại ký tự.
//!
//! Phase 3 chia lịch sử raw thành các đoạn liên tục cùng loại. Mỗi đoạn chữ
//! (Chu) được đưa qua Telex độc lập; mọi đoạn khác được render nguyên bản.
//! Việc này ngăn phím dấu thanh và phím hình chữ xuyên qua ranh giới từ,
//! cho phép code, URL, command và tiếng Việt trộn trong cùng phiên.
//!
//! ```text
//! lịch sử thao tác → phan_doan → Vec<Doan> → mỗi đoạn render riêng
//! ```

use crate::cau_hinh::KieuTelex;
use crate::render;
use crate::thao_tac::{CachNhap, ThaoTacNhap};
use alloc::vec::Vec;

/// Loại một đoạn raw.
///
/// Mọi loại trừ `Chu` đều được render nguyên bản (không Telex). `Chu` là ứng
/// viên Telex: ASCII letters và chữ Việt dựng sẵn; trong `KieuTelex::DayDu`,
/// `[` và `]` cũng là `Chu` vì chúng sinh `ư`/`ơ`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoaiDoan {
    /// Chữ cái (Telex-eligible): ASCII letters, chữ Việt dựng sẵn, `[`/`]` DayDu.
    Chu,
    /// Chữ số ASCII.
    So,
    /// Khoảng trắng ASCII.
    KhoangTrang,
    /// Dấu câu văn bản (`.`, `,`, `!`, `?`, `;`, `'`, `"`, `(`, `)`, `-`, `_`).
    DauCau,
    /// Ký tự kỹ thuật (`:`, `/`, `\`, `@`, `#`, ...).
    KyThuat,
    /// Non-ASCII không phải chữ Việt (emoji, combining mark, dấu câu Unicode).
    Emoji,
    /// Ký tự do `them_nguyen_ban` — literal, tạo ranh giới đoạn.
    NguyenBan,
}

/// Một đoạn raw liên tục cùng [`LoaiDoan`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Doan {
    /// Vị trí raw đầu (inclusive) trong lịch sử.
    pub(crate) bat_dau: usize,
    /// Vị trí raw cuối (exclusive).
    pub(crate) ket_thuc: usize,
    /// Loại đoạn.
    pub(crate) loai: LoaiDoan,
}

/// Trả `true` nếu `c` là một trong các chữ cái nền có shape doubled-base
/// (`a`/`e`/`o`/`d`), không phân biệt hoa thường.
fn la_chu_hinh_nhan_doi(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'o' | 'd')
}

/// Trả `true` nếu đoạn chữ raw là teencode lặp: có run 3+ chữ cái hình chữ
/// doubled-base (`a`/`e`/`o`/`d`) giống nhau liên tiếp, bắt đầu sau một ký tự
/// khác trong đoạn.
///
/// Rule:
/// * `"ooo"` (nguyên đoạn) → escape Telex → `"oo"` (giữ behavior Phase 2).
/// * `"brooo"` (lặp có chữ khác trước) → bảo toàn raw → `"brooo"`.
///
/// Tiếng Việt không có nguyên âm/phụ âm doubled-base lặp 3+, nên rule này
/// chỉ chạm vào teencode/nước ngoài, không phá âm tiết Việt hợp lệ.
pub(crate) fn la_teencode_lap(thao_tac: &[ThaoTacNhap]) -> bool {
    let mut i = 0;
    while i < thao_tac.len() {
        let c = thao_tac[i].ky_tu;
        if la_chu_hinh_nhan_doi(c) {
            let run = thao_tac[i..]
                .iter()
                .take_while(|t| t.ky_tu.eq_ignore_ascii_case(&c))
                .count();
            if run >= 3 && i > 0 {
                return true;
            }
            i += run;
        } else {
            i += 1;
        }
    }
    false
}

/// Trả `true` nếu ký tự là khoảng trắng ASCII.
fn la_khoang_trang(c: char) -> bool {
    c.is_ascii_whitespace()
}

/// Trả `true` nếu ký tự là dấu câu văn bản (không kỹ thuật).
fn la_dau_cau(c: char) -> bool {
    matches!(
        c,
        '.' | ',' | '!' | '?' | ';' | '\'' | '"' | '(' | ')' | '-' | '_'
    )
}

/// Trả `true` nếu ký tự là ký tự kỹ thuật (ranh giới mạnh).
fn la_ky_thuat(c: char) -> bool {
    matches!(
        c,
        ':' | '/'
            | '\\'
            | '@'
            | '#'
            | '$'
            | '%'
            | '^'
            | '&'
            | '*'
            | '+'
            | '='
            | '<'
            | '>'
            | '{'
            | '}'
            | '|'
            | '`'
            | '~'
    )
}

/// Phân loại một thao tác raw (chỉ xét ký tự và cách nhập).
fn phan_loai(t: &ThaoTacNhap, kieu_telex: KieuTelex) -> LoaiDoan {
    if t.cach_nhap == CachNhap::NguyenBan {
        return LoaiDoan::NguyenBan;
    }
    let c = t.ky_tu;
    if c.is_ascii() {
        // DayDu: `[` và `]` sinh `ư`/`ơ` nên là ứng viên Telex.
        if kieu_telex == KieuTelex::DayDu && (c == '[' || c == ']') {
            return LoaiDoan::Chu;
        }
        if c.is_ascii_alphabetic() {
            LoaiDoan::Chu
        } else if c.is_ascii_digit() {
            LoaiDoan::So
        } else if la_khoang_trang(c) {
            LoaiDoan::KhoangTrang
        } else if la_dau_cau(c) {
            LoaiDoan::DauCau
        } else if la_ky_thuat(c) {
            LoaiDoan::KyThuat
        } else {
            // Các ký tự ASCII còn lại (control, ký tự lạ) — giữ nguyên.
            LoaiDoan::KyThuat
        }
    } else {
        // Non-ASCII: chữ Việt dựng sẵn → Chu; còn lại (emoji, combining,
        // dấu câu Unicode) → Emoji (render nguyên bản).
        if render::phan_tich_ky_tu(c).is_some() {
            LoaiDoan::Chu
        } else {
            LoaiDoan::Emoji
        }
    }
}

/// Phân đoạn lịch sử thao tác thành các đoạn liên tục cùng loại.
///
/// Các thao tác `them_nguyen_ban` luôn là `NguyenBan` (ranh giới đoạn),
/// bất kể ký tự. Hai thao tác cạnh nhau cùng loại được gộp.
pub(crate) fn phan_doan(thao_tac: &[ThaoTacNhap], kieu_telex: KieuTelex) -> Vec<Doan> {
    let mut ket_qua = Vec::new();
    let mut i = 0;
    while i < thao_tac.len() {
        let loai = phan_loai(&thao_tac[i], kieu_telex);
        let bat_dau = i;
        while i < thao_tac.len() && phan_loai(&thao_tac[i], kieu_telex) == loai {
            i += 1;
        }
        ket_qua.push(Doan {
            bat_dau,
            ket_thuc: i,
            loai,
        });
    }
    ket_qua
}
