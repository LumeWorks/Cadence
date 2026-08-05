// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

pub mod ban_chup;
pub mod bo_go;
pub mod cau_hinh;
pub(crate) mod chu_viet;
pub mod ket_qua;
pub mod loai_noi_dung;
pub mod phien_go;
pub(crate) mod render;
pub(crate) mod thao_tac;
pub mod vi_tri;

pub use ban_chup::BanChupSoan;
pub use bo_go::BoGo;
pub use cau_hinh::{CauHinh, DangUnicode, KieuTelex, LoiCauHinh, QuyTacDatDau};
pub use ket_qua::KetQuaXuLy;
pub use loai_noi_dung::LoaiNoiDung;
pub use phien_go::PhienGo;
pub use vi_tri::ViTriVanBan;
