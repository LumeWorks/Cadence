// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Soak test VNI: chịu tải dài, random Telex/VNI, modifier sequence, technical number.

use cadence::{BoGo, CauHinh, KetQuaXuLy, KieuGo};

/// 1000 ký tự VNI liên tục: không panic, cursor hợp lệ.
#[test]
fn ngan_ky_tu_vni_lien_tuc() {
    let mut c = CauHinh::mac_dinh();
    c.dat_kieu_go(KieuGo::Vni);
    c.dat_gioi_han_thao_tac(4096).expect("hop le");
    let bo_go = BoGo::new(c).expect("hop le");
    let mut phien = bo_go.tao_phien();
    let mut seed = 42u64;
    for _ in 0..1000 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let c = match seed % 20 {
            0..=9 => char::from_digit((seed % 10) as u32, 10).expect("digit"),
            _ => char::from(b'a' + (seed % 26) as u8),
        };
        phien.them_ky_tu(c);
    }
    let noi_dung_len = phien.ban_chup().noi_dung().len();
    phien.ve_cuoi();
    let mut dem = 0;
    while phien.di_trai() == KetQuaXuLy::CapNhat {
        dem += 1;
    }
    assert!(dem > 0 || noi_dung_len == 0);
    phien.ve_dau();
    let mut dem2 = 0;
    while phien.di_phai() == KetQuaXuLy::CapNhat {
        dem2 += 1;
    }
    assert!(dem2 > 0 || noi_dung_len == 0);
}

/// Random Telex/VNI theo seed: đủ bước kiểm tra ổn định.
#[test]
fn soak_telex_vni_random() {
    let steps = 5_000;
    let mut seed = 12345u64;
    for _ in 0..steps {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let kieu = if seed % 2 == 0 {
            KieuGo::Telex
        } else {
            KieuGo::Vni
        };
        let mut c = CauHinh::mac_dinh();
        c.dat_kieu_go(kieu);
        let bo_go = BoGo::new(c).expect("hop le");
        let mut phien = bo_go.tao_phien();
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let n = (seed % 32) as usize;
        for _ in 0..n {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let ch = char::from(b'a' + (seed % 26) as u8);
            phien.them_ky_tu(ch);
        }
        let _ = phien.ban_chup().noi_dung();
    }
}

/// Chuỗi digit modifier lặp không panic.
#[test]
fn soak_modifier_lap() {
    let mut c = CauHinh::mac_dinh();
    c.dat_kieu_go(KieuGo::Vni);
    c.dat_gioi_han_thao_tac(4096).expect("hop le");
    let bo_go = BoGo::new(c).expect("hop le");
    let mut phien = bo_go.tao_phien();
    for i in 0..500 {
        let ch = if i % 2 == 0 {
            'a'
        } else {
            char::from_digit((i % 10) as u32, 10).expect("digit")
        };
        phien.them_ky_tu(ch);
    }
    let _ = phien.ban_chup().noi_dung();
}

/// Soak generator: chạy cả Telex và VNI.
#[test]
fn soak_ca_telex_va_vni() {
    for kieu in [KieuGo::Telex, KieuGo::Vni] {
        let mut c = CauHinh::mac_dinh();
        c.dat_kieu_go(kieu);
        c.dat_gioi_han_thao_tac(4096).expect("hop le");
        let bo_go = BoGo::new(c).expect("hop le");
        let mut phien = bo_go.tao_phien();
        for i in 0..1000 {
            let ch = if i % 5 == 0 {
                'a'
            } else if i % 5 == 1 {
                'o'
            } else {
                'n'
            };
            phien.them_ky_tu(ch);
        }
        let _ = phien.ban_chup().noi_dung();
    }
}
