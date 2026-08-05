// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Kiểm tra derive serde (chỉ chạy khi bật feature `serde`).
//!
//! Không kéo thêm data-format crate; chỉ xác minh các public type thực
//! sự cài đặt `Serialize`/`Deserialize` để giữ surface nhỏ.

#![cfg(feature = "serde")]

use cadence::{KetQuaXuLy, LoaiNoiDung};
use serde::{Deserialize, Serialize};

fn assert_serialize<T: Serialize>() {}
fn assert_deserialize<'de, T: Deserialize<'de>>() {}

#[test]
fn ket_qua_xu_ly_co_serde() {
    assert_serialize::<KetQuaXuLy>();
    assert_deserialize::<KetQuaXuLy>();
}

#[test]
fn loai_noi_dung_co_serde() {
    assert_serialize::<LoaiNoiDung>();
    assert_deserialize::<LoaiNoiDung>();
}
