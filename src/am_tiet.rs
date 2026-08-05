// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Phân tích âm tiết tiếng Việt và bảng âm đầu/âm cuối.
//!
//! Module này cung cấp mô hình âm tiết và parser kiểm tra tính hợp lệ
//! của một chuỗi có phải âm tiết tiếng Việt hay không. Không dùng từ điển
//! hay regex; dùng bảng tĩnh và method có tên rõ.

/// Mức hợp lệ của một chuỗi khi parse thành âm tiết.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MucHopLe {
    /// Chưa hoàn chỉnh nhưng có thể tiếp tục gõ thành âm tiết hợp lệ.
    CoTheTiepTuc,
    /// Không thể thành âm tiết tiếng Việt.
    KhongHopLe,
}

/// Bảng âm đầu tiếng Việt (onset). Sắp xếp theo độ dài giảm để match
/// prefix dài trước (vd: `ngh` trước `ng` trước `n`).
const AM_DAU: &[&str] = &[
    "ngh", "ng", "nh", "gh", "gi", "kh", "ph", "th", "tr", "qu", "ch", "b", "c", "d", "đ", "g",
    "h", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "x",
];

/// Bảng âm cuối tiếng Việt (coda). Sắp xếp theo độ dài giảm.
const AM_CUOI: &[&str] = &["ch", "ng", "nh", "c", "m", "n", "p", "t"];

/// Trả `true` nếu `c` là nguyên âm tiếng Việt (dựng sẵn hoặc thường).
fn la_nguyen_am(c: char) -> bool {
    matches!(
        c.to_ascii_lowercase(),
        'a' | 'ă' | 'â' | 'e' | 'ê' | 'i' | 'o' | 'ô' | 'ơ' | 'u' | 'ư' | 'y'
    )
}

/// Trả độ dài của âm đầu matched (0 = onset rỗng, hợp lệ).
pub(crate) fn do_dai_am_dau(s: &str) -> usize {
    let thuong = s.to_ascii_lowercase();
    for &am in AM_DAU {
        if thuong.starts_with(am) {
            return am.len();
        }
    }
    0
}

/// Trả độ dài của âm cuối matched (0 = không có âm cuối, vần mở).
pub(crate) fn do_dai_am_cuoi(s: &str) -> usize {
    let thuong = s.to_ascii_lowercase();
    for &am in AM_CUOI {
        if thuong.ends_with(am) {
            return am.len();
        }
    }
    0
}

/// Kiểm tra xem một chuỗi (không dấu thanh, dạng thường) có thể là một
/// âm tiết tiếng Việt hợp lệ hoặc có thể tiếp tục.
///
/// Parser Phase 2:
/// 1. Tách âm đầu (onset).
/// 2. Sau âm đầu, ký tự tiếp phải là nguyên âm (nếu có).
/// 3. Tách âm cuối (coda) từ phần còn lại.
/// 4. Vần (giữa onset và coda) phải chỉ chứa nguyên âm.
pub(crate) fn phan_tich_am_tiet(s: &str) -> MucHopLe {
    if s.is_empty() {
        return MucHopLe::CoTheTiepTuc;
    }
    let thuong = s.to_ascii_lowercase();

    // Bước 1: tách âm đầu.
    let do_dai_dau = do_dai_am_dau(&thuong);
    let sau_dau = &thuong[do_dai_dau..];

    // Bước 2: phần sau âm đầu phải có nguyên âm ở đầu.
    if sau_dau.is_empty() {
        return MucHopLe::CoTheTiepTuc;
    }
    let ky_tu_dau_sau = sau_dau.chars().next().unwrap_or(' ');
    if !la_nguyen_am(ky_tu_dau_sau) {
        // Sau onset là phụ âm không thuộc coda → không hợp lệ.
        return MucHopLe::KhongHopLe;
    }

    // Bước 3: tách âm cuối.
    let do_dai_cuoi = do_dai_am_cuoi(sau_dau);
    let van = &sau_dau[..sau_dau.len() - do_dai_cuoi];

    // Bước 4: vần phải không rỗng và chỉ chứa nguyên âm.
    if van.is_empty() {
        // Chỉ có coda, không có vowel → có thể tiếp tục (chưa gõ vowel).
        return MucHopLe::CoTheTiepTuc;
    }
    if van.chars().any(|c| !la_nguyen_am(c)) {
        return MucHopLe::KhongHopLe;
    }

    // Phase 2: mọi vần chỉ chứa nguyên âm là CoTheTiepTuc.
    // Phase 3: nucleus 2+ nguyên âm mà không có glide {i,u,ư,y,o} thì không
    // thể là vần Việt (vd `ae` trong `CASE`, `uo` thì `u` là glide). Nguyên
    // âm đầy (a, ă, â, e, ê, ô, ơ) không bao giờ đứng làm glide nên tổ hợp
    // hai nguyên âm đầy không hợp lệ.
    if van.chars().count() >= 2
        && !van.chars().any(|c| matches!(c, 'i' | 'u' | 'ư' | 'y' | 'o'))
    {
        return MucHopLe::KhongHopLe;
    }

    MucHopLe::CoTheTiepTuc
}

/// Kiểm tra xem chuỗi bắt đầu bằng âm đầu hợp lệ (không yêu cầu vowel
/// theo sau). Dùng cho selection escape: `dd` bắt đầu bằng `d` hợp lệ.
pub(crate) fn bat_dau_onset_hop_le(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    let thuong = s.to_ascii_lowercase();
    let dau = thuong.chars().next().unwrap_or(' ');
    if la_nguyen_am(dau) || dau == 'đ' {
        return true;
    }
    do_dai_am_dau(&thuong) > 0
}

/// Kiểm tra xem raw token có âm đầu hợp lệ và theo sau là nguyên âm không.
///
/// Dùng cho selection tone: nếu raw bắt đầu bằng onset nhưng theo sau là
/// phụ âm (như `cl` trong `class`), toàn bộ token fallback về raw.
pub(crate) fn raw_co_onset_hop_le(raw: &str) -> bool {
    if raw.is_empty() {
        return true;
    }
    let thuong = raw.to_ascii_lowercase();
    if !bat_dau_onset_hop_le(&thuong) {
        return false;
    }
    let do_dai = do_dai_am_dau(&thuong);
    let sau = &thuong[do_dai..];
    if sau.is_empty() {
        return true;
    }
    let ky_tu_sau = sau.chars().next().unwrap_or(' ');
    la_nguyen_am(ky_tu_sau)
}
