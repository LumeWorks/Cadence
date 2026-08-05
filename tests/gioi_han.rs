// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test giới hạn thao tác của phiên.

use cadence::{BoGo, CauHinh, KetQuaXuLy};

fn tao_phien(gioi_han: usize) -> cadence::PhienGo {
    let mut cau_hinh = CauHinh::mac_dinh();
    cau_hinh
        .dat_gioi_han_thao_tac(gioi_han)
        .expect("gioi han hop le");
    let bo_go = BoGo::new(cau_hinh).expect("cau hinh hop le");
    bo_go.tao_phien()
}

#[test]
fn dat_dung_gioi_han() {
    let mut phien = tao_phien(3);
    phien.them_ky_tu('a');
    phien.them_ky_tu('b');
    phien.them_ky_tu('c');
    assert_eq!(phien.ban_chup().noi_dung(), "abc");
}

#[test]
fn them_qua_gioi_han_khong_doi_state() {
    let mut phien = tao_phien(2);
    phien.them_ky_tu('a');
    phien.them_ky_tu('b');
    // Đã đạt giới hạn 2 thao tác.
    let ket_qua = phien.them_ky_tu('c');
    assert!(matches!(ket_qua, KetQuaXuLy::KhongDoi));
    // Snapshot không đổi.
    assert_eq!(phien.ban_chup().noi_dung(), "ab");
}

#[test]
fn xoa_sau_khi_dat_gioi_han() {
    let mut phien = tao_phien(2);
    phien.them_ky_tu('a');
    phien.them_ky_tu('b');
    // Vượt giới hạn bị từ chối.
    let _ = phien.them_ky_tu('c');
    // Xóa lùi giải phóng một chỗ.
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "a");
}

#[test]
fn sau_khi_xoa_co_the_them_lai() {
    let mut phien = tao_phien(2);
    phien.them_ky_tu('a');
    phien.them_ky_tu('b');
    let _ = phien.them_ky_tu('c');
    phien.xoa_lui();
    // Giờ có chỗ lại, thêm được.
    let ket_qua = phien.them_ky_tu('d');
    assert!(matches!(ket_qua, KetQuaXuLy::CapNhat));
    assert_eq!(phien.ban_chup().noi_dung(), "ad");
}

#[test]
fn nhieu_phien_co_gioi_han_doc_lap() {
    let bo_go = BoGo::new({
        let mut cau_hinh = CauHinh::mac_dinh();
        cau_hinh.dat_gioi_han_thao_tac(2).expect("2 phai hop le");
        cau_hinh
    })
    .expect("cau hinh hop le");

    let mut phien_a = bo_go.tao_phien();
    let mut phien_b = bo_go.tao_phien();

    phien_a.them_ky_tu('a');
    phien_a.them_ky_tu('b');
    let qua = phien_a.them_ky_tu('c');
    assert!(matches!(qua, KetQuaXuLy::KhongDoi));

    // phien_b vẫn rỗng và độc lập.
    assert!(phien_b.dang_trong());
    phien_b.them_ky_tu('x');
    assert_eq!(phien_b.ban_chup().noi_dung(), "x");
}

#[test]
fn gioi_han_ap_dung_cho_nguyen_ban() {
    let mut phien = tao_phien(1);
    phien.them_nguyen_ban('a');
    let ket_qua = phien.them_nguyen_ban('b');
    assert!(matches!(ket_qua, KetQuaXuLy::KhongDoi));
    assert_eq!(phien.ban_chup().noi_dung(), "a");
}
