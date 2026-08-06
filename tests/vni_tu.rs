// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test VNI từ và âm tiết: các từ tiếng Việt phổ biến.

use cadence::{BoGo, CauHinh, KieuGo, QuyTacDatDau};

fn go_vni(raw: &str) -> String {
    let mut c = CauHinh::mac_dinh();
    c.dat_kieu_go(KieuGo::Vni);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in raw.chars() {
        phien.them_ky_tu(ch);
    }
    phien.ban_chup().noi_dung().to_string()
}

fn go_vni_qt(raw: &str, qt: QuyTacDatDau) -> String {
    let mut c = CauHinh::mac_dinh();
    c.dat_kieu_go(KieuGo::Vni);
    c.dat_quy_tac_dat_dau(qt);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in raw.chars() {
        phien.them_ky_tu(ch);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// Các từ phổ biến.
#[test]
fn cac_tu_pho_bien() {
    assert_eq!(go_vni("tieng61"), "tiếng");
    assert_eq!(go_vni("nguo7i2"), "người");
    assert_eq!(go_vni("d9uo7ng2"), "đường");
    assert_eq!(go_vni("Vie65t"), "Việt");
    assert_eq!(go_vni("d9a6y"), "đây");
    assert_eq!(go_vni("gioi3"), "giỏi");
    assert_eq!(go_vni("nghie6ng2"), "nghi\u{1ec1}ng"); // nghiềng (ê + huyền)
}

/// `thủy` / `thủy`: tone trên `u` (bán âm `y`).
#[test]
fn thuy_tone_tren_u() {
    assert_eq!(go_vni("thuy3"), "thủy");
    assert_eq!(go_vni("thuy1"), "thúy");
    assert_eq!(go_vni("thuy2"), "thùy");
}

/// `quý` / `quý`: tone trên `y` (nucleus sau onset `qu`).
#[test]
fn quy_tone_tren_y() {
    assert_eq!(go_vni("quy1"), "quý");
    assert_eq!(go_vni("quy3"), "quỷ");
    assert_eq!(go_vni("quy4"), "quỹ");
}

/// `khuỷu`: 3 nguyên âm `u y u`, tone trên `y` (tone ở cuối âm tiết).
#[test]
fn khuyu_tone_tren_y() {
    assert_eq!(go_vni("khuyu3"), "khuỷu");
}

/// `hòa` / `hóa`: quy tắc đặt dấu HienDai vs TruyenThong.
#[test]
fn hoa_quy_tac_dat_dau() {
    assert_eq!(go_vni_qt("hoa2", QuyTacDatDau::HienDai), "hòa");
    assert_eq!(go_vni_qt("hoa2", QuyTacDatDau::TruyenThong), "hoà");
    assert_eq!(go_vni_qt("hoa1", QuyTacDatDau::HienDai), "hóa");
    assert_eq!(go_vni_qt("hoa1", QuyTacDatDau::TruyenThong), "hoá");
}

/// Raw history giữ byte-for-byte.
#[test]
fn raw_giu_nguyen() {
    let mut c = CauHinh::mac_dinh();
    c.dat_kieu_go(KieuGo::Vni);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in "tieng61".chars() {
        phien.them_ky_tu(ch);
    }
    assert_eq!(phien.ban_chup().noi_dung(), "tiếng");
    assert_eq!(phien.ban_chup().noi_dung_goc(), "tieng61");
}

/// Backspace hoàn tác một raw action.
#[test]
fn backspace_hoan_tac_raw() {
    let mut c = CauHinh::mac_dinh();
    c.dat_kieu_go(KieuGo::Vni);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in "a61".chars() {
        phien.them_ky_tu(ch);
    }
    assert_eq!(phien.ban_chup().noi_dung(), "ấ");
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "â");
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "a");
    phien.xoa_lui();
    assert!(phien.dang_trong());
}

/// `them_nguyen_ban` chặn VNI.
#[test]
fn them_nguyen_ban_chan_vni() {
    let mut c = CauHinh::mac_dinh();
    c.dat_kieu_go(KieuGo::Vni);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    phien.them_ky_tu('a');
    phien.them_nguyen_ban('1');
    assert_eq!(phien.ban_chup().noi_dung(), "a1");
    assert_eq!(phien.ban_chup().noi_dung_goc(), "a1");
}
