// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test biến đổi hình chữ Telex (aa/aw/ee/oo/ow/uw/dd).

use cadence::{BoGo, CauHinh, LoaiNoiDung};

fn tao_phien() -> cadence::PhienGo {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh hop le");
    bo_go.tao_phien()
}

fn nhap(phien: &mut cadence::PhienGo, s: &str) {
    for c in s.chars() {
        phien.them_ky_tu(c);
    }
}

#[test]
fn aa_thanh_a_mu() {
    let mut phien = tao_phien();
    nhap(&mut phien, "aa");
    assert_eq!(phien.ban_chup().noi_dung(), "â");
}

#[test]
fn aw_thanh_a_trang() {
    let mut phien = tao_phien();
    nhap(&mut phien, "aw");
    assert_eq!(phien.ban_chup().noi_dung(), "ă");
}

#[test]
fn ee_thanh_e_mu() {
    let mut phien = tao_phien();
    nhap(&mut phien, "ee");
    assert_eq!(phien.ban_chup().noi_dung(), "ê");
}

#[test]
fn oo_thanh_o_mu() {
    let mut phien = tao_phien();
    nhap(&mut phien, "oo");
    assert_eq!(phien.ban_chup().noi_dung(), "ô");
}

#[test]
fn ow_thanh_o_moc() {
    let mut phien = tao_phien();
    nhap(&mut phien, "ow");
    assert_eq!(phien.ban_chup().noi_dung(), "ơ");
}

#[test]
fn uw_thanh_u_moc() {
    let mut phien = tao_phien();
    nhap(&mut phien, "uw");
    assert_eq!(phien.ban_chup().noi_dung(), "ư");
}

#[test]
fn dd_thanh_d_gach() {
    let mut phien = tao_phien();
    nhap(&mut phien, "dd");
    assert_eq!(phien.ban_chup().noi_dung(), "đ");
}

#[test]
fn aa_giu_nguyen_ban() {
    let mut phien = tao_phien();
    nhap(&mut phien, "aa");
    assert_eq!(phien.ban_chup().noi_dung_goc(), "aa");
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.loai_noi_dung(), LoaiNoiDung::BienDoiTelex);
}

#[test]
fn aa_hoa_bien_athuong() {
    let mut phien = tao_phien();
    nhap(&mut phien, "AA");
    assert_eq!(phien.ban_chup().noi_dung(), "Â");
}

#[test]
fn aa_hoa_dau_bien_a_hoa() {
    // Aa → Â (kiểu hoa theo chữ gốc đầu tiên).
    let mut phien = tao_phien();
    nhap(&mut phien, "Aa");
    assert_eq!(phien.ban_chup().noi_dung(), "Â");
}

#[test]
fn aa_thuong_dau_hoa_bien_a_thuong() {
    // aA → â (chữ gốc đầu thường).
    let mut phien = tao_phien();
    nhap(&mut phien, "aA");
    assert_eq!(phien.ban_chup().noi_dung(), "â");
}

#[test]
fn dd_hoa_bien_d_hoa() {
    let mut phien = tao_phien();
    nhap(&mut phien, "DD");
    assert_eq!(phien.ban_chup().noi_dung(), "Đ");
}

#[test]
fn dd_hoa_dau_bien_d_hoa() {
    let mut phien = tao_phien();
    nhap(&mut phien, "Dd");
    assert_eq!(phien.ban_chup().noi_dung(), "Đ");
}

#[test]
fn dd_thuong_dau_hoa_bien_d_thuong() {
    let mut phien = tao_phien();
    nhap(&mut phien, "dD");
    assert_eq!(phien.ban_chup().noi_dung(), "đ");
}

#[test]
fn them_nguyen_ban_khong_bien_doi_aa() {
    let mut phien = tao_phien();
    phien.them_nguyen_ban('a');
    phien.them_nguyen_ban('a');
    assert_eq!(phien.ban_chup().noi_dung(), "aa");
    assert_eq!(phien.ban_chup().loai_noi_dung(), LoaiNoiDung::NguyenBan);
}

#[test]
fn nguyen_ban_chan_telex_noi_xuyen() {
    // them_ky_tu('a') rồi them_nguyen_ban('a'): 'a' nguyên bản không làm
    // modifier nên không thành â.
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('a');
    assert_eq!(phien.ban_chup().noi_dung(), "aa");
}

#[test]
fn backspace_hoan_tac_mot_thao_tac() {
    let mut phien = tao_phien();
    nhap(&mut phien, "aa");
    assert_eq!(phien.ban_chup().noi_dung(), "â");
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "a");
}

#[test]
fn w_don_le_can_bang_giu_nguyen() {
    // KieuTelex::CanBang (mặc định): w đơn lẻ giữ nguyên.
    let mut phien = tao_phien();
    nhap(&mut phien, "w");
    assert_eq!(phien.ban_chup().noi_dung(), "w");
}

#[test]
fn w_sau_chu_khong_hop_le_giu_nguyen() {
    // w chỉ là modifier khi có a/o/u phù hợp. Sau 'b' thì w giữ nguyên.
    let mut phien = tao_phien();
    nhap(&mut phien, "bw");
    assert_eq!(phien.ban_chup().noi_dung(), "bw");
}
