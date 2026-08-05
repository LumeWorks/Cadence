// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test cursor navigation trong Telex-transformed text: grapheme boundaries
//! đúng, di_trai/di_phai không dừng giữa cluster.

use cadence::{BoGo, CauHinh};

fn phien() -> cadence::PhienGo {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    bo_go.tao_phien()
}

/// Sau `aa`→`â` (2 raw, 1 grapheme), di_phai từ đầu → cuối, chỉ 1 bước.
#[test]
fn di_phai_sau_shape_mot_grapheme() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('a');
    // Cursor ở raw 2 (cuối). Ve_dau → raw 0.
    phien.ve_dau();
    assert_eq!(phien.ban_chup().con_tro().chi_so_grapheme(), 0);
    phien.di_phai();
    assert_eq!(phien.ban_chup().con_tro().chi_so_grapheme(), 1);
    // Di_phai nữa → KhongDoi (đã ở cuối).
    assert!(matches!(phien.di_phai(), cadence::KetQuaXuLy::KhongDoi));
}

/// Sau `aws`→`ắ` (3 raw, 1 grapheme), di_trai từ cuối → đầu, chỉ 1 bước.
#[test]
fn di_trai_sau_tone_mot_grapheme() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('w');
    phien.them_ky_tu('s');
    // Cursor ở raw 3 (cuối). chi_so_grapheme = 1.
    assert_eq!(phien.ban_chup().con_tro().chi_so_grapheme(), 1);
    phien.di_trai();
    assert_eq!(phien.ban_chup().con_tro().chi_so_grapheme(), 0);
    // Di_trai nữa → KhongDoi.
    assert!(matches!(phien.di_trai(), cadence::KetQuaXuLy::KhongDoi));
}

/// Sau `tieengs`→`tiếng` (7 raw, 5 graphemes), di_phai 5 lần → hết.
#[test]
fn di_phai_qua_tieng_5_grapheme() {
    let mut phien = phien();
    for c in "tieengs".chars() {
        phien.them_ky_tu(c);
    }
    phien.ve_dau();
    for i in 1..=5 {
        phien.di_phai();
        assert_eq!(
            phien.ban_chup().con_tro().chi_so_grapheme(),
            i,
            "grapheme {i}"
        );
    }
    assert!(matches!(phien.di_phai(), cadence::KetQuaXuLy::KhongDoi));
}

/// Byte index sau di_trai luôn là char boundary.
#[test]
fn byte_index_sau_di_trai_la_char_boundary() {
    let mut phien = phien();
    for c in "tieengs".chars() {
        phien.them_ky_tu(c);
    }
    for _ in 0..3 {
        phien.di_trai();
        let bc = phien.ban_chup();
        assert!(
            bc.noi_dung()
                .is_char_boundary(bc.con_tro().chi_so_byte()),
            "byte {} khong la char boundary",
            bc.con_tro().chi_so_byte()
        );
    }
}

/// Backspace sau `aa`→`â` hoàn tác `a` cuối → `a`.
#[test]
fn backspace_sau_shape_hoan_tac() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('a');
    assert_eq!(phien.ban_chup().noi_dung(), "â");
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "a");
}

/// Backspace sau `aws`→`ắ` hoàn tác `s` → `ă`.
#[test]
fn backspace_sau_tone_hoan_tac() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('w');
    phien.them_ky_tu('s');
    assert_eq!(phien.ban_chup().noi_dung(), "ắ");
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "ă");
}

/// Backspace sau `ass`→`as` (escape) hoàn tác escape trigger → `á`.
#[test]
fn backspace_sau_escape_chay_lai_telex() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('s');
    phien.them_ky_tu('s');
    assert_eq!(phien.ban_chup().noi_dung(), "as");
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "á");
}

/// Backspace sau `as`→`á` rồi `z`→`a` hoàn tác `z` → `á`.
#[test]
fn backspace_sau_z_chay_lai_dau_thanh() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('s');
    phien.them_ky_tu('z');
    assert_eq!(phien.ban_chup().noi_dung(), "a");
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "á");
}

/// Backspace sau `uow`→`ươ` hoàn tác `w` → `uo`.
#[test]
fn backspace_hoan_tac_uo_w() {
    let mut phien = phien();
    phien.them_ky_tu('u');
    phien.them_ky_tu('o');
    phien.them_ky_tu('w');
    assert_eq!(phien.ban_chup().noi_dung(), "ươ");
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "uo");
}

/// Backspace rồi nhập lại tạo shape lại: `aa`→`â`, xóa, nhập `a`→`â`.
#[test]
fn backspace_roi_nhap_lai_tao_shape() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('a');
    assert_eq!(phien.ban_chup().noi_dung(), "â");
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), "a");
    phien.them_ky_tu('a');
    assert_eq!(phien.ban_chup().noi_dung(), "â");
}

/// Xoa_phia_truoc trên Telex text: `aws`→`ắ`, xoa_phia_truoc ở đầu → `ws`.
#[test]
fn xoa_phia_truoc_tren_telex() {
    let mut phien = phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('w');
    phien.them_ky_tu('s');
    phien.ve_dau();
    phien.xoa_phia_truoc();
    // Xóa raw `a`, còn `ws` → w đơn lẻ literal, s là tone nhưng không có vowel.
    let kq = phien.ban_chup().noi_dung();
    assert!(
        kq == "ws" || kq == "s" || kq == "w",
        "kỳ vọng ws/s/w, được {kq}"
    );
}
