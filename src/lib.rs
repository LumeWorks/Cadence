// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Cadence — lõi gõ tiếng Việt thế hệ mới.
//!
//! Phase 1 chỉ dựng nền móng bất biến: nhận và giữ nguyên mọi ký tự
//! người dùng nhập, duy trì lịch sử thao tác, hỗ trợ con trỏ trong
//! đoạn đang soạn và snapshot trung lập nền tảng. Telex chưa được
//! triển khai trong giai đoạn này.

//! Cadence mặc định dùng `std`, nhưng vẫn biên dịch được với `no_std + alloc`.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

pub mod ban_chup;
pub mod bo_go;
pub mod cau_hinh;
pub mod ket_qua;
pub mod loai_noi_dung;
pub mod phien_go;
pub(crate) mod thao_tac;
pub mod vi_tri;

pub use ban_chup::BanChupSoan;
pub use bo_go::BoGo;
pub use cau_hinh::{CauHinh, LoiCauHinh};
pub use ket_qua::KetQuaXuLy;
pub use loai_noi_dung::LoaiNoiDung;
pub use phien_go::PhienGo;
pub use vi_tri::ViTriVanBan;
