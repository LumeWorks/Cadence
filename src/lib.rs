// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

pub(crate) mod am_tiet;
pub(crate) mod anh_xa;
pub mod ban_chup;
pub mod bo_go;
pub mod cau_hinh;
pub(crate) mod chu_viet;
pub mod ket_qua;
pub mod loai_noi_dung;
pub(crate) mod lua_chon;
pub(crate) mod ngu_canh;
pub(crate) mod phan_doan;
pub mod phien_go;
pub(crate) mod render;
pub(crate) mod telex;
pub(crate) mod thao_tac;
#[cfg(feature = "trace")]
pub mod trace;
pub mod vi_tri;

pub use ban_chup::BanChupSoan;
pub use bo_go::BoGo;
pub use cau_hinh::{CauHinh, ChinhSachLuaChon, DangUnicode, KieuTelex, LoiCauHinh, QuyTacDatDau};
pub use ket_qua::KetQuaXuLy;
pub use loai_noi_dung::LoaiNoiDung;
pub use phien_go::PhienGo;
pub use vi_tri::ViTriVanBan;

#[cfg(feature = "trace")]
pub use ngu_canh::BangChungLuaChon;
#[cfg(feature = "trace")]
pub use trace::{TraceKetQua, TraceStep};
