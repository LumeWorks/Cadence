// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test con trỏ và chỉnh sửa giữa đoạn.

use cadence::{BoGo, CauHinh, KetQuaXuLy};

fn tao_phien() -> cadence::PhienGo {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh hop le");
    bo_go.tao_phien()
}

/// Trả grapheme index của con trỏ snapshot.
fn grapheme_con_tro(phien: &cadence::PhienGo) -> usize {
    phien.ban_chup().con_tro().chi_so_grapheme()
}

#[test]
fn di_trai_khi_o_dau() {
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    phien.ve_dau();
    let ket_qua = phien.di_trai();
    assert!(matches!(ket_qua, KetQuaXuLy::KhongDoi));
    assert_eq!(grapheme_con_tro(&phien), 0);
}

#[test]
fn di_phai_khi_o_cuoi() {
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    let ket_qua = phien.di_phai();
    assert!(matches!(ket_qua, KetQuaXuLy::KhongDoi));
    assert_eq!(grapheme_con_tro(&phien), 1);
}

#[test]
fn di_trai_mot_buoc() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    let ket_qua = phien.di_trai();
    assert!(matches!(ket_qua, KetQuaXuLy::CapNhat));
    assert_eq!(grapheme_con_tro(&phien), 2);
}

#[test]
fn di_phai_mot_buoc() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    phien.ve_dau();
    let ket_qua = phien.di_phai();
    assert!(matches!(ket_qua, KetQuaXuLy::CapNhat));
    assert_eq!(grapheme_con_tro(&phien), 1);
}

#[test]
fn ve_dau_tu_cuoi() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    phien.ve_dau();
    assert_eq!(grapheme_con_tro(&phien), 0);
}

#[test]
fn ve_cuoi_tu_dau() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    phien.ve_dau();
    phien.ve_cuoi();
    assert_eq!(grapheme_con_tro(&phien), 3);
}

#[test]
fn xoa_lui_o_dau() {
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    phien.ve_dau();
    let ket_qua = phien.xoa_lui();
    assert!(matches!(ket_qua, KetQuaXuLy::KhongDoi));
    assert_eq!(phien.ban_chup().noi_dung(), "a");
}

#[test]
fn xoa_lui_o_giua() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    phien.di_trai();
    // Con trỏ giữa b và c; xóa lùi bỏ b.
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "ac");
}

#[test]
fn xoa_lui_o_cuoi() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "ab");
}

#[test]
fn xoa_phia_truoc_o_dau() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    phien.ve_dau();
    phien.xoa_phia_truoc();
    assert_eq!(phien.ban_chup().noi_dung(), "bc");
}

#[test]
fn xoa_phia_truoc_o_giua() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    phien.di_trai();
    // Con trỏ giữa b và c; xóa phía trước bỏ c.
    phien.xoa_phia_truoc();
    assert_eq!(phien.ban_chup().noi_dung(), "ab");
}

#[test]
fn xoa_phia_truoc_o_cuoi() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    let ket_qua = phien.xoa_phia_truoc();
    assert!(matches!(ket_qua, KetQuaXuLy::KhongDoi));
    assert_eq!(phien.ban_chup().noi_dung(), "abc");
}

#[test]
fn chen_sau_khi_di_chuyen() {
    let mut phien = tao_phien();
    for ky_tu in "ac".chars() {
        phien.them_ky_tu(ky_tu);
    }
    phien.ve_dau();
    phien.di_phai();
    // Con trỏ giữa a và c; chèn b.
    phien.them_ky_tu('b');
    assert_eq!(phien.ban_chup().noi_dung(), "abc");
    assert_eq!(grapheme_con_tro(&phien), 2);
}

#[test]
fn chuoi_thao_tac_chinh_sua_phuc_tap() {
    let mut phien = tao_phien();
    // Nhập "hello".
    for ky_tu in "hello".chars() {
        phien.them_ky_tu(ky_tu);
    }
    // Di về đầu, xóa 'h', chèn 'j' -> "jello".
    phien.ve_dau();
    phien.xoa_phia_truoc();
    phien.them_ky_tu('j');
    // Về cuối, xóa lùi 'o', chèn 'y' -> "jelly".
    phien.ve_cuoi();
    phien.xoa_lui();
    phien.them_ky_tu('y');
    assert_eq!(phien.ban_chup().noi_dung(), "jelly");
}
