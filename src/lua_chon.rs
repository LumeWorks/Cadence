// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Lựa chọn giữa raw và biến đổi Telex.
//!
//! Quy tắc Phase 2:
//! 1. Nếu kết quả Telex có shape transform (â, ă, ê, ô, ơ, ư, đ) → giữ Telex.
//! 2. Nếu chỉ có tone transform, parse Telex output: nếu không hợp lệ → raw.
//! 3. Escape luôn được giữ (ý định người dùng).
//! 4. `them_nguyen_ban` luôn giữ raw (đảm bảo ở tầng telex).

use crate::am_tiet;
use crate::chu_viet::{DauChu, DauThanh};
use crate::telex::{DonViRender, NoiDungDonVi};
use alloc::string::String;

/// Kết quả lựa chọn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KetQuaLuaChon {
    /// Dùng kết quả Telex.
    Telex,
    /// Dùng raw input.
    NguyenBan,
}

/// Render Telex output thành chuỗi thường (không dấu thanh) để parse.
fn render_de_tu_don_vi(don_vi: &[DonViRender]) -> String {
    let mut s = String::new();
    for u in don_vi {
        match &u.noi_dung {
            NoiDungDonVi::Chu(chu) => {
                // Dùng ký tự thường không dấu thanh để parse âm tiết.
                let ky_tu = crate::render::nguyen_am_nfc(chu.chu_goc, chu.dau_chu, DauThanh::Khong)
                    .unwrap_or_else(|| chu.chu_goc.ky_tu_thuong());
                s.push(ky_tu);
            }
            NoiDungDonVi::Chuong(c) => {
                s.push(*c);
            }
        }
    }
    s
}

/// Quyết định dùng Telex hay raw cho một đoạn chữ.
pub(crate) fn lua_chon(
    don_vi: &[DonViRender],
    _raw: &str,
    co_escape: bool,
    co_escape_hinh_chu: bool,
) -> KetQuaLuaChon {
    // Kiểm tra loại transform.
    let co_shape = don_vi.iter().any(|u| match &u.noi_dung {
        NoiDungDonVi::Chu(chu) => matches!(
            chu.dau_chu,
            DauChu::Trang | DauChu::Mu | DauChu::Moc | DauChu::Gach
        ),
        NoiDungDonVi::Chuong(_) => false,
    });
    let co_tone = don_vi.iter().any(|u| match &u.noi_dung {
        NoiDungDonVi::Chu(chu) => !matches!(chu.dau_thanh, DauThanh::Khong),
        NoiDungDonVi::Chuong(_) => false,
    });

    // Rule 1: có shape transform → giữ Telex (ngay cả khi chưa phải âm tiết).
    if co_shape {
        return KetQuaLuaChon::Telex;
    }

    let output = render_de_tu_don_vi(don_vi);

    // Rule 2: onset+vowel không hợp lệ (như `cl` trong `clas`) → raw.
    // Escape hình chữ (dd→đ rồi undo) vẫn giữ vì `dd` là cặp Telex hợp lệ.
    if !co_escape_hinh_chu && !am_tiet::raw_co_onset_hop_le(&output) {
        return KetQuaLuaChon::NguyenBan;
    }

    // Rule 3: escape luôn giữ (ý định người dùng thoát Telex).
    if co_escape {
        return KetQuaLuaChon::Telex;
    }

    // Rule 4: chỉ có tone transform → parse âm tiết đầy đủ.
    if co_tone && am_tiet::phan_tich_am_tiet(&output) == am_tiet::MucHopLe::KhongHopLe {
        return KetQuaLuaChon::NguyenBan;
    }

    KetQuaLuaChon::Telex
}
