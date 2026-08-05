// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Ví dụ trace - xem quyết định raw/Telex cho từng đoạn.
//!
//! Yêu cầu feature `trace`: `cargo run --features trace --example truy_vet`.

#[cfg(feature = "trace")]
mod enabled {
    use cadence::{BoGo, CauHinh};

    pub fn chay() {
        let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh luon hop le");
        let mut phien = bo_go.tao_phien();

        // Trộn code, URL, tiếng Việt trong cùng phiên.
        for c in "cargo build lỗi rồi =)) tieengs".chars() {
            phien.them_ky_tu(c);
        }

        println!("=== Trace quyết định ===\n");
        println!("Input raw: {}", phien.ban_chup().noi_dung_goc());
        println!("Output:    {}\n", phien.ban_chup().noi_dung());

        for (i, step) in phien.trace().iter().enumerate() {
            let raw_slice: String = phien
                .ban_chup()
                .noi_dung_goc()
                .chars()
                .skip(step.doan_bat_dau)
                .take(step.doan_ket_thuc - step.doan_bat_dau)
                .collect();
            println!(
                "{i}: [{}, {}) raw={raw_slice:?} → {:?} | bang_chung={:?} | ket_qua={:?}",
                step.doan_bat_dau, step.doan_ket_thuc, step.chuoi_ra, step.bang_chung, step.ket_qua
            );
        }
    }
}

#[cfg(not(feature = "trace"))]
mod disabled {
    pub fn chay() {
        eprintln!("Ví dụ này yêu cầu feature `trace`.");
        eprintln!("Chạy: cargo run --features trace --example truy_vet");
    }
}

fn main() {
    #[cfg(feature = "trace")]
    enabled::chay();
    #[cfg(not(feature = "trace"))]
    disabled::chay();
}
