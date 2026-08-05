// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus editing - chèn/xóa/navigation giữa đoạn, Telex và raw.
//! Liên kết branch: `phien_go.rs`, `anh_xa.rs` (cursor, snap, navigable).

use cadence::{BoGo, CauHinh, KetQuaXuLy};

fn phien() -> cadence::PhienGo {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    bo_go.tao_phien()
}

/// Chèn giữa hai ký tự ASCII → đúng thứ tự.
#[test]
fn chen_giua_ascii() {
    let mut p = phien();
    for c in "ac".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    p.di_phai();
    p.them_ky_tu('b');
    assert_eq!(p.ban_chup().noi_dung(), "abc");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 2);
}

/// Chèn giữa Telex grapheme - cursor snap đúng, không panic.
#[test]
fn chen_giua_telex_grapheme() {
    let mut p = phien();
    for c in "tieengs".chars() {
        p.them_ky_tu(c);
    }
    // 5 grapheme: t i ế n g.
    p.ve_dau();
    for _ in 0..3 {
        p.di_phai();
    }
    p.them_nguyen_ban('x');
    let bc = p.ban_chup();
    // Chèn 'x' (nguyên bản) tạo ranh giới đoạn, re-run Telex hai nửa.
    // Bất biến: không panic, byte index là boundary, raw giữ.
    assert!(bc.noi_dung().is_char_boundary(bc.con_tro().chi_so_byte()));
    assert!(bc.noi_dung().contains('x'));
    // Xóa 'x' vừa chèn → phục hồi (vì 'x' nguyên bản là một raw action).
    p.xoa_lui();
    assert_eq!(p.ban_chup().noi_dung(), "tiếng");
}

/// Xóa lùi hoàn tác một raw action (backspace semantic).
#[test]
fn xoa_lui_hoan_tac_mot_raw() {
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    let truoc = p.ban_chup().noi_dung().to_string();
    p.them_ky_tu('d');
    p.xoa_lui();
    assert_eq!(p.ban_chup().noi_dung(), truoc);
}

/// Xóa phía trước (delete forward) ở giữa.
#[test]
fn xoa_phia_truoc_giua() {
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    p.di_trai();
    p.xoa_phia_truoc();
    assert_eq!(p.ban_chup().noi_dung(), "ab");
}

/// Navigation đạt đầu/cuối trong hữu hạn bước, không loop.
#[test]
fn navigation_dat_dau_cuoi() {
    let mut p = phien();
    for c in "tieengs".chars() {
        p.them_ky_tu(c);
    }
    // Đi trái liên tục - phải chạm đầu và KhongDoi.
    for _ in 0..100 {
        if matches!(p.di_trai(), KetQuaXuLy::KhongDoi) {
            break;
        }
    }
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 0);
    // Đi phải liên tục - phải chạm cuối.
    for _ in 0..100 {
        if matches!(p.di_phai(), KetQuaXuLy::KhongDoi) {
            break;
        }
    }
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 5);
}

/// Ve_dau/ve_cuoi idempotent.
#[test]
fn ve_dau_cuoi_idempotent() {
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    assert!(matches!(p.ve_dau(), KetQuaXuLy::KhongDoi));
    p.ve_cuoi();
    assert!(matches!(p.ve_cuoi(), KetQuaXuLy::KhongDoi));
}

/// Backspace xuyên ranh giới nguyên bản.
#[test]
fn backspace_xuyen_ranh_gioi_nguyen_ban() {
    let mut p = phien();
    for c in "as".chars() {
        p.them_ky_tu(c);
    }
    p.them_nguyen_ban('x');
    assert_eq!(p.ban_chup().noi_dung(), "áx");
    p.xoa_lui();
    assert_eq!(p.ban_chup().noi_dung(), "á");
}

/// Restore raw idempotent (no-op).
#[test]
fn restore_raw_idempotent() {
    let mut p = phien();
    for c in "tieengs".chars() {
        p.them_ky_tu(c);
    }
    let truoc = p.ban_chup().noi_dung().to_string();
    assert!(matches!(p.khoi_phuc_nguyen_ban(), KetQuaXuLy::KhongDoi));
    assert!(matches!(p.khoi_phuc_nguyen_ban(), KetQuaXuLy::KhongDoi));
    assert_eq!(p.ban_chup().noi_dung(), truoc);
}

/// Commit/reset liên tục không rò state.
#[test]
fn commit_reset_lien_tuc() {
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    let _ = p.chap_nhan();
    assert!(p.dang_trong());
    p.dat_lai();
    assert!(p.dang_trong());
    p.them_ky_tu('z');
    assert_eq!(p.ban_chup().noi_dung(), "z");
}
