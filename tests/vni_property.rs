// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Property tests VNI: deterministic replay, round-trip, cursor, modifier sequence.

use cadence::{BoGo, CauHinh, KetQuaXuLy, KieuGo};
use proptest::prelude::*;

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

// VNI replay deterministic: cùng input → cùng output.
proptest! {
    #[test]
    fn vni_replay_deterministic(s in "[a-z0-9]{0,32}") {
        let r1 = go_vni(&s);
        let r2 = go_vni(&s);
        prop_assert_eq!(r1, r2);
    }

    /// VNI raw round-trip: noi_dung_goc() luôn trả byte-for-byte raw.
    #[test]
    fn vni_raw_round_trip(s in "[a-z0-9]{0,32}") {
        let mut c = CauHinh::mac_dinh();
        c.dat_kieu_go(KieuGo::Vni);
        let bo_go = BoGo::new(c).expect("hop le");
        let mut phien = bo_go.tao_phien();
        for ch in s.chars() {
            phien.them_ky_tu(ch);
        }
        prop_assert_eq!(phien.ban_chup().noi_dung_goc(), s.clone());
    }

    /// VNI add-delete round-trip: thêm rồi xóa hết → rỗng.
    #[test]
    fn vni_add_delete_round_trip(s in "[a-z0-9]{0,16}") {
        let mut c = CauHinh::mac_dinh();
        c.dat_kieu_go(KieuGo::Vni);
        let bo_go = BoGo::new(c).expect("hop le");
        let mut phien = bo_go.tao_phien();
        for ch in s.chars() {
            phien.them_ky_tu(ch);
        }
        for _ in 0..s.len() {
            phien.xoa_lui();
        }
        prop_assert!(phien.dang_trong());
    }

    /// VNI them_nguyen_ban không biến đổi.
    #[test]
    fn vni_nguyen_ban_khong_bien_doi(s in "[a-z0-9]{0,16}") {
        let mut c = CauHinh::mac_dinh();
        c.dat_kieu_go(KieuGo::Vni);
        let bo_go = BoGo::new(c).expect("hop le");
        let mut phien = bo_go.tao_phien();
        for ch in s.chars() {
            phien.them_nguyen_ban(ch);
        }
        prop_assert_eq!(phien.ban_chup().noi_dung(), s.clone());
        prop_assert_eq!(phien.ban_chup().noi_dung_goc(), s);
    }

    /// VNI modifier sequence không panic (digit 1-9 lặp).
    #[test]
    fn vni_modifier_sequence_khong_panic(s in "[a-z1-9]{0,64}") {
        let _ = go_vni(&s);
    }

    /// VNI cursor di trái ở đầu → KhongDoi.
    #[test]
    fn vni_di_trai_dau_khong_doi(s in "[a-z0-9]{0,16}") {
        let mut c = CauHinh::mac_dinh();
        c.dat_kieu_go(KieuGo::Vni);
        let bo_go = BoGo::new(c).expect("hop le");
        let mut phien = bo_go.tao_phien();
        for ch in s.chars() {
            phien.them_ky_tu(ch);
        }
        phien.ve_dau();
        let kq = phien.di_trai();
        prop_assert_eq!(kq, KetQuaXuLy::KhongDoi);
    }

    /// VNI cursor di phải ở cuối → KhongDoi.
    #[test]
    fn vni_di_phai_cuoi_khong_doi(s in "[a-z0-9]{0,16}") {
        let mut c = CauHinh::mac_dinh();
        c.dat_kieu_go(KieuGo::Vni);
        let bo_go = BoGo::new(c).expect("hop le");
        let mut phien = bo_go.tao_phien();
        for ch in s.chars() {
            phien.them_ky_tu(ch);
        }
        let kq = phien.di_phai();
        prop_assert_eq!(kq, KetQuaXuLy::KhongDoi);
    }
}

/// Serde round-trip giữ đúng kiểu gõ.
#[cfg(feature = "serde")]
#[test]
fn serde_giu_dung_kieu_go() {
    use cadence::KieuGo;
    let json = serde_json::to_string(&KieuGo::Vni).expect("serialize");
    let decoded: KieuGo = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(decoded, KieuGo::Vni);
    assert_eq!(KieuGo::default(), KieuGo::Telex);
}
