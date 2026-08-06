// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Lựa chọn giữa raw và biến đổi Telex.
//!
//! Quy tắc Phase 3 (mỗi đoạn chữ độc lập):
//! 1. Shape transform + onset không hợp lệ → raw (`foo`→`foo`, `f` không là
//!    onset Việt). Shape + onset hợp lệ → Telex (`ddm`→`đm`, `aaq`→`âq`).
//! 2. Shape transform (onset hợp lệ) → Telex (ngay cả khi chưa phải âm tiết).
//! 3. Escape hình chữ → Telex (`ddd`→`dd`); escape dấu thanh → Telex (`ass`→`as`).
//! 4. Onset raw không hợp lệ (`cl` trong `class`) → raw.
//! 5. Chỉ tone + âm tiết không hợp lệ → raw (`async`→`async`).
//! 6. `them_nguyen_ban` chặn Telex ở tầng phân đoạn (đoạn `NguyenBan` luôn raw).
//!
//! Teencode lặp (3+ chữ cái hình chữ doubled-base có chữ khác trước, như
//! `brooo`) được bảo toàn raw TRƯỚC khi gọi `lua_chon` (xem `phan_doan`).

use super::am_tiet;
use super::chu_viet::{DauChu, DauThanh};
use super::telex::{DonViRender, NoiDungDonVi};
use crate::cau_hinh::ChinhSachLuaChon;
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
pub(crate) fn render_de_tu_don_vi(don_vi: &[DonViRender]) -> String {
    let mut s = String::new();
    for u in don_vi {
        match &u.noi_dung {
            NoiDungDonVi::Chu(chu) => {
                // Dùng ký tự thường không dấu thanh để parse âm tiết.
                let ky_tu = super::render::nguyen_am_nfc(chu.chu_goc, chu.dau_chu, DauThanh::Khong)
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
    co_nguyen_ban: bool,
    chinh_sach: ChinhSachLuaChon,
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
    // Đếm số đơn vị mang dấu thanh. Tiếng Việt có đúng một dấu thanh mỗi âm
    // tiết; hai dấu trở lên (`úẻ` từ `user`) không thể là âm tiết hợp lệ.
    let so_dau_thanh = don_vi
        .iter()
        .filter(|u| match &u.noi_dung {
            NoiDungDonVi::Chu(chu) => !matches!(chu.dau_thanh, DauThanh::Khong),
            NoiDungDonVi::Chuong(_) => false,
        })
        .count();

    let output = render_de_tu_don_vi(don_vi);

    // Rule 0: từ hai dấu thanh trở lên → raw (không phải âm tiết Việt).
    if so_dau_thanh >= 2 {
        return KetQuaLuaChon::NguyenBan;
    }

    // Rule 1: shape transform + onset không hợp lệ → raw.
    // Onset hợp lệ = nguyên âm đầu HOẶC phụ âm onset Việt (`b`,`c`,`d`,`đ`,...).
    // `foo`→`fô`: `f` không là onset Việt → raw. `ddm`→`đm`: `đ` là onset → Telex.
    if co_shape && !am_tiet::bat_dau_onset_hop_le(&output) {
        return KetQuaLuaChon::NguyenBan;
    }

    // Rule 2: shape transform (onset hợp lệ) → Telex.
    if co_shape {
        return KetQuaLuaChon::Telex;
    }

    // Rule 3: onset raw không hợp lệ (`cl` trong `class`) → raw.
    // Escape hình chữ (dd→đ rồi undo) vẫn giữ vì `dd` là cặp Telex hợp lệ.
    if !co_escape_hinh_chu && !am_tiet::raw_co_onset_hop_le(&output) {
        return KetQuaLuaChon::NguyenBan;
    }

    // Rule 4: escape luôn giữ (ý định người dùng thoát Telex).
    if co_escape {
        return KetQuaLuaChon::Telex;
    }

    // Rule 5: chỉ có tone transform → parse âm tiết đầy đủ.
    // Bỏ qua khi có `them_nguyen_ban` vì các đoạn độc lập.
    // `UuTienTiengViet` cho phép Telex ngay cả khi âm tiết chưa hoàn chỉnh.
    if co_tone
        && !co_nguyen_ban
        && chinh_sach != ChinhSachLuaChon::UuTienTiengViet
        && am_tiet::phan_tich_am_tiet(&output) == am_tiet::MucHopLe::KhongHopLe
    {
        return KetQuaLuaChon::NguyenBan;
    }

    KetQuaLuaChon::Telex
}
