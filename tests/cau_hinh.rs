// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test cấu hình và lỗi domain.

use cadence::{CauHinh, LoiCauHinh};

#[test]
fn cau_hinh_mac_dinh_hop_le() {
    let cau_hinh = CauHinh::mac_dinh();
    assert_eq!(cau_hinh.gioi_han_thao_tac(), 128);
}

#[test]
fn dat_gioi_han_toi_thieu_hop_le() {
    let mut cau_hinh = CauHinh::mac_dinh();
    cau_hinh.dat_gioi_han_thao_tac(1).expect("1 phai hop le");
    assert_eq!(cau_hinh.gioi_han_thao_tac(), 1);
}

#[test]
fn dat_gioi_han_toi_da_hop_le() {
    let mut cau_hinh = CauHinh::mac_dinh();
    cau_hinh
        .dat_gioi_han_thao_tac(4096)
        .expect("4096 phai hop le");
    assert_eq!(cau_hinh.gioi_han_thao_tac(), 4096);
}

#[test]
fn dat_gioi_han_bang_khong_bi_tu_choi() {
    let mut cau_hinh = CauHinh::mac_dinh();
    let loi = cau_hinh.dat_gioi_han_thao_tac(0).unwrap_err();
    match loi {
        LoiCauHinh::GioiHanThaoTacKhongHopLe {
            gioi_han,
            toi_thieu,
            toi_da,
        } => {
            assert_eq!(gioi_han, 0);
            assert_eq!(toi_thieu, 1);
            assert_eq!(toi_da, 4096);
        }
    }
}

#[test]
fn dat_gioi_han_vuot_toi_da_bi_tu_choi() {
    let mut cau_hinh = CauHinh::mac_dinh();
    let loi = cau_hinh.dat_gioi_han_thao_tac(4097).unwrap_err();
    match loi {
        LoiCauHinh::GioiHanThaoTacKhongHopLe {
            gioi_han,
            toi_thieu,
            toi_da,
        } => {
            assert_eq!(gioi_han, 4097);
            assert_eq!(toi_thieu, 1);
            assert_eq!(toi_da, 4096);
        }
    }
}

#[test]
fn cau_hinh_loi_khong_thay_doi_gia_tri_cu() {
    let mut cau_hinh = CauHinh::mac_dinh();
    let ket_qua = cau_hinh.dat_gioi_han_thao_tac(0);
    assert!(ket_qua.is_err());
    // Giá trị cũ phải được giữ nguyên.
    assert_eq!(cau_hinh.gioi_han_thao_tac(), 128);
}

#[test]
fn display_loi_cau_hinh_co_ngu_nghia() {
    let loi = LoiCauHinh::GioiHanThaoTacKhongHopLe {
        gioi_han: 0,
        toi_thieu: 1,
        toi_da: 4096,
    };
    let chuoi = loi.to_string();
    assert!(chuoi.contains("0"));
    assert!(chuoi.contains("1"));
    assert!(chuoi.contains("4096"));
    assert!(chuoi.contains("khong hop le"));
}
