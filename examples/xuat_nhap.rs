// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Ví dụ serde - serialize/deserialize các public data type.
//!
//! Yêu cầu feature `serde`: `cargo run --features serde --example xuat_nhap`.

#[cfg(feature = "serde")]
mod enabled {
    use cadence::{BoGo, CauHinh, KetQuaXuLy, LoaiNoiDung};

    pub fn chay() {
        let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh luon hop le");
        let mut phien = bo_go.tao_phien();

        for c in "tieengs".chars() {
            phien.them_ky_tu(c);
        }

        let kq = phien.chap_nhan();
        println!("=== Serde round-trip ===\n");

        // Serialize KetQuaXuLy.
        let json = serde_json::to_string(&kq).expect("serialize KetQuaXuLy");
        println!("KetQuaXuLy → {json}");

        // Deserialize ngược.
        let kq2: KetQuaXuLy = serde_json::from_str(&json).expect("deserialize KetQuaXuLy");
        assert_eq!(kq, kq2);
        println!("Round-trip OK\n");

        // Serialize LoaiNoiDung.
        for c in "tieengs".chars() {
            phien.them_ky_tu(c);
        }
        let loai = phien.ban_chup().loai_noi_dung();
        let json = serde_json::to_string(&loai).expect("serialize LoaiNoiDung");
        println!("LoaiNoiDung → {json}");
        let loai2: LoaiNoiDung = serde_json::from_str(&json).expect("deserialize LoaiNoiDung");
        assert_eq!(loai, loai2);
        println!("Round-trip OK");
    }
}

#[cfg(not(feature = "serde"))]
mod disabled {
    pub fn chay() {
        eprintln!("Ví dụ này yêu cầu feature `serde`.");
        eprintln!("Chạy: cargo run --features serde --example xuat_nhap");
    }
}

fn main() {
    #[cfg(feature = "serde")]
    enabled::chay();
    #[cfg(not(feature = "serde"))]
    disabled::chay();
}
