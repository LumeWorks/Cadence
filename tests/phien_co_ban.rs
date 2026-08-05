// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test phiên cơ bản: thêm, snapshot, reset, commit.

use cadence::{BoGo, CauHinh, KetQuaXuLy};

fn tao_phien() -> cadence::PhienGo {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh hop le");
    bo_go.tao_phien()
}

#[test]
fn phien_moi_rong() {
    let phien = tao_phien();
    assert!(phien.dang_trong());
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "");
    assert_eq!(ban_chup.noi_dung_goc(), "");
    assert!(ban_chup.dang_trong());
}

#[test]
fn them_mot_ky_tu() {
    let mut phien = tao_phien();
    let ket_qua = phien.them_ky_tu('a');
    assert!(matches!(ket_qua, KetQuaXuLy::CapNhat));
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "a");
    assert_eq!(ban_chup.noi_dung_goc(), "a");
    assert!(!phien.dang_trong());
}

#[test]
fn them_nhieu_ky_tu() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "abc");
    assert_eq!(ban_chup.noi_dung_goc(), "abc");
}

#[test]
fn them_tai_giua() {
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('c');
    // Con trỏ ở cuối; di về giữa rồi chèn 'b'.
    phien.di_trai();
    phien.them_ky_tu('b');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "abc");
}

#[test]
fn hai_phien_doc_lap() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh hop le");
    let mut phien_a = bo_go.tao_phien();
    let mut phien_b = bo_go.tao_phien();
    phien_a.them_ky_tu('x');
    phien_b.them_ky_tu('y');
    assert_eq!(phien_a.ban_chup().noi_dung(), "x");
    assert_eq!(phien_b.ban_chup().noi_dung(), "y");
}

#[test]
fn snapshot_tra_noi_dung_dung() {
    let mut phien = tao_phien();
    phien.them_nguyen_ban('a');
    phien.them_nguyen_ban('b');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung(), "ab");
}

#[test]
fn noi_dung_goc_chinh_xac() {
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('b');
    phien.them_ky_tu('c');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.noi_dung_goc(), "abc");
}

#[test]
fn them_nguyen_ban_bao_toan_co_noi_bo() {
    // Phase 1: hiển thị giống them_ky_tu nhưng lịch sử ghi cờ NguyenBan.
    let mut phien = tao_phien();
    let ket_qua = phien.them_nguyen_ban('a');
    assert!(matches!(ket_qua, KetQuaXuLy::CapNhat));
    assert_eq!(phien.ban_chup().noi_dung(), "a");
}

#[test]
fn reset_xoa_toan_bo() {
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('b');
    phien.dat_lai();
    assert!(phien.dang_trong());
    assert_eq!(phien.ban_chup().noi_dung(), "");
}

#[test]
fn reset_nhieu_lan_khong_loi() {
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    phien.dat_lai();
    phien.dat_lai();
    phien.dat_lai();
    assert!(phien.dang_trong());
}

#[test]
fn commit_phien_rong_khong_sinh_noi_dung() {
    let mut phien = tao_phien();
    let ket_qua = phien.chap_nhan();
    assert!(matches!(ket_qua, KetQuaXuLy::KhongDoi));
    assert!(phien.dang_trong());
}

#[test]
fn commit_phien_co_noi_dung_tra_dung_chuoi() {
    let mut phien = tao_phien();
    for ky_tu in "hello".chars() {
        phien.them_ky_tu(ky_tu);
    }
    let ket_qua = phien.chap_nhan();
    match ket_qua {
        KetQuaXuLy::ChapNhan { noi_dung } => {
            assert_eq!(noi_dung, "hello");
        }
        _ => panic!("commit phai tra ChapNhan"),
    }
}

#[test]
fn sau_commit_phien_rong_hoan_toan() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    let _ = phien.chap_nhan();
    assert!(phien.dang_trong());
    assert_eq!(phien.ban_chup().noi_dung(), "");
    assert_eq!(phien.ban_chup().noi_dung_goc(), "");
}

#[test]
fn token_sau_commit_khong_chua_state_cu() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    let _ = phien.chap_nhan();
    // Token mới sau commit phải rỗng.
    phien.them_ky_tu('d');
    assert_eq!(phien.ban_chup().noi_dung(), "d");
}

#[test]
fn khoi_phuc_nguyen_ban_idempotent() {
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    let lan1 = phien.khoi_phuc_nguyen_ban();
    let lan2 = phien.khoi_phuc_nguyen_ban();
    assert!(matches!(lan1, KetQuaXuLy::KhongDoi));
    assert!(matches!(lan2, KetQuaXuLy::KhongDoi));
    assert_eq!(phien.ban_chup().noi_dung(), "a");
}
