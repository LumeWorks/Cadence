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

// ---------------------------------------------------------------------------
// Editing matrix: mỗi thao tác tại mỗi vị trí (rỗng, đầu, giữa, cuối).
// ---------------------------------------------------------------------------

/// Insert tại mọi vị trí: rỗng, đầu, giữa, cuối — nội dung đúng, cursor đúng.
#[test]
fn matrix_them_tai_moi_vi_tri() {
    // Rỗng → insert 'a' → "a", cursor 1.
    let mut p = phien();
    p.them_ky_tu('a');
    assert_eq!(p.ban_chup().noi_dung(), "a");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 1);

    // Đầu của "bc" → insert 'a' → "abc", cursor 1.
    let mut p = phien();
    for c in "bc".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    p.them_ky_tu('a');
    assert_eq!(p.ban_chup().noi_dung(), "abc");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 1);

    // Giữa "ac" → insert 'b' → "abc", cursor 2.
    let mut p = phien();
    for c in "ac".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    p.di_phai();
    p.them_ky_tu('b');
    assert_eq!(p.ban_chup().noi_dung(), "abc");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 2);

    // Cuối "ab" → insert 'c' → "abc", cursor 3.
    let mut p = phien();
    for c in "ab".chars() {
        p.them_ky_tu(c);
    }
    p.them_ky_tu('c');
    assert_eq!(p.ban_chup().noi_dung(), "abc");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 3);
}

/// Backspace tại mọi vị trí: rỗng (no-op), đầu (no-op), giữa, cuối.
#[test]
fn matrix_xoa_lui_tai_moi_vi_tri() {
    // Rỗng → KhongDoi.
    let mut p = phien();
    assert!(matches!(p.xoa_lui(), KetQuaXuLy::KhongDoi));
    assert!(p.dang_trong());

    // Đầu "abc" → KhongDoi, nội dung giữ.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    assert!(matches!(p.xoa_lui(), KetQuaXuLy::KhongDoi));
    assert_eq!(p.ban_chup().noi_dung(), "abc");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 0);

    // Giữa "abc" (cursor sau 'b') → xóa 'b', "ac", cursor 1.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    p.di_trai();
    p.xoa_lui();
    assert_eq!(p.ban_chup().noi_dung(), "ac");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 1);

    // Cuối "abc" → xóa 'c', "ab", cursor 2.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    p.xoa_lui();
    assert_eq!(p.ban_chup().noi_dung(), "ab");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 2);
}

/// Delete-forward tại mọi vị trí: rỗng (no-op), đầu, giữa, cuối (no-op).
#[test]
fn matrix_xoa_phia_truoc_tai_moi_vi_tri() {
    // Rỗng → KhongDoi.
    let mut p = phien();
    assert!(matches!(p.xoa_phia_truoc(), KetQuaXuLy::KhongDoi));

    // Đầu "abc" → xóa 'a', "bc", cursor 0.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    p.xoa_phia_truoc();
    assert_eq!(p.ban_chup().noi_dung(), "bc");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 0);

    // Giữa "abc" (cursor sau 'a') → xóa 'b', "ac", cursor 1.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    p.di_phai();
    p.xoa_phia_truoc();
    assert_eq!(p.ban_chup().noi_dung(), "ac");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 1);

    // Cuối "abc" → KhongDoi, nội dung giữ.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    assert!(matches!(p.xoa_phia_truoc(), KetQuaXuLy::KhongDoi));
    assert_eq!(p.ban_chup().noi_dung(), "abc");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 3);
}

/// Di trái tại mọi vị trí: rỗng (no-op), đầu (no-op), giữa, cuối.
#[test]
fn matrix_di_trai_tai_moi_vi_tri() {
    // Rỗng → KhongDoi.
    let mut p = phien();
    assert!(matches!(p.di_trai(), KetQuaXuLy::KhongDoi));

    // Đầu → KhongDoi.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    assert!(matches!(p.di_trai(), KetQuaXuLy::KhongDoi));
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 0);

    // Giữa → CapNhat, cursor giảm 1.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    assert!(matches!(p.di_trai(), KetQuaXuLy::CapNhat));
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 2);

    // Cuối → CapNhat.
    let mut p = phien();
    for c in "ab".chars() {
        p.them_ky_tu(c);
    }
    assert!(matches!(p.di_trai(), KetQuaXuLy::CapNhat));
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 1);
}

/// Di phải tại mọi vị trí: rỗng (no-op), đầu, giữa, cuối (no-op).
#[test]
fn matrix_di_phai_tai_moi_vi_tri() {
    // Rỗng → KhongDoi.
    let mut p = phien();
    assert!(matches!(p.di_phai(), KetQuaXuLy::KhongDoi));

    // Đầu → CapNhat.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    assert!(matches!(p.di_phai(), KetQuaXuLy::CapNhat));
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 1);

    // Giữa → CapNhat.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    p.di_phai();
    assert!(matches!(p.di_phai(), KetQuaXuLy::CapNhat));
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 2);

    // Cuối → KhongDoi.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    assert!(matches!(p.di_phai(), KetQuaXuLy::KhongDoi));
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 3);
}

/// Ve_dau/ve_cuoi tại mọi vị trí: rỗng (no-op), đầu/cuối (no-op), giữa (CapNhat).
#[test]
fn matrix_ve_dau_cuoi_tai_moi_vi_tri() {
    // Rỗng → KhongDoi.
    let mut p = phien();
    assert!(matches!(p.ve_dau(), KetQuaXuLy::KhongDoi));
    assert!(matches!(p.ve_cuoi(), KetQuaXuLy::KhongDoi));

    // Đầu → ve_dau KhongDoi, ve_cuoi CapNhat.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    assert!(matches!(p.ve_dau(), KetQuaXuLy::KhongDoi));
    assert!(matches!(p.ve_cuoi(), KetQuaXuLy::CapNhat));
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 3);

    // Giữa → cả hai CapNhat.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    p.ve_dau();
    p.di_phai();
    assert!(matches!(p.ve_dau(), KetQuaXuLy::CapNhat));
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 0);
    assert!(matches!(p.ve_cuoi(), KetQuaXuLy::CapNhat));
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 3);

    // Cuối → ve_cuoi KhongDoi, ve_dau CapNhat.
    let mut p = phien();
    for c in "abc".chars() {
        p.them_ky_tu(c);
    }
    assert!(matches!(p.ve_cuoi(), KetQuaXuLy::KhongDoi));
    assert!(matches!(p.ve_dau(), KetQuaXuLy::CapNhat));
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 0);
}

// ---------------------------------------------------------------------------
// Backspace + re-type: tone, shape, shape+tone.
// ---------------------------------------------------------------------------

/// Backspace sau tone → nhập lại tone key → phục hồi dấu thanh.
#[test]
fn backspace_roi_nhap_lai_tone() {
    let mut p = phien();
    for c in "as".chars() {
        p.them_ky_tu(c);
    }
    assert_eq!(p.ban_chup().noi_dung(), "á");
    p.xoa_lui();
    assert_eq!(p.ban_chup().noi_dung(), "a");
    p.them_ky_tu('s');
    assert_eq!(p.ban_chup().noi_dung(), "á");
}

/// Backspace sau shape → nhập lại modifier → phục hồi hình chữ.
#[test]
fn backspace_roi_nhap_lai_shape() {
    let mut p = phien();
    for c in "aw".chars() {
        p.them_ky_tu(c);
    }
    assert_eq!(p.ban_chup().noi_dung(), "ă");
    p.xoa_lui();
    assert_eq!(p.ban_chup().noi_dung(), "a");
    p.them_ky_tu('w');
    assert_eq!(p.ban_chup().noi_dung(), "ă");
}

/// Backspace sau shape+tone → nhập lại tone → phục hồi đầy đủ.
#[test]
fn backspace_roi_nhap_lai_shape_tone() {
    let mut p = phien();
    for c in "aws".chars() {
        p.them_ky_tu(c);
    }
    assert_eq!(p.ban_chup().noi_dung(), "ắ");
    // Xóa tone → "ă".
    p.xoa_lui();
    assert_eq!(p.ban_chup().noi_dung(), "ă");
    // Nhập lại tone → "ắ".
    p.them_ky_tu('s');
    assert_eq!(p.ban_chup().noi_dung(), "ắ");
}

/// Backspace sau shape+tone hai bước → phục hồi shape rồi tone.
#[test]
fn backspace_hai_buoc_roi_nhap_lai() {
    let mut p = phien();
    for c in "aws".chars() {
        p.them_ky_tu(c);
    }
    assert_eq!(p.ban_chup().noi_dung(), "ắ");
    // Bước 1: xóa tone → "ă".
    p.xoa_lui();
    assert_eq!(p.ban_chup().noi_dung(), "ă");
    // Bước 2: xóa shape → "a".
    p.xoa_lui();
    assert_eq!(p.ban_chup().noi_dung(), "a");
    // Nhập lại shape + tone.
    p.them_ky_tu('w');
    assert_eq!(p.ban_chup().noi_dung(), "ă");
    p.them_ky_tu('s');
    assert_eq!(p.ban_chup().noi_dung(), "ắ");
}

/// Delete-forward trên Telex grapheme: shape+tone, xóa từ đầu.
///
/// `aws`→`ắ` (1 grapheme, 3 raw actions). Delete-forward ở đầu xóa raw `a`,
/// còn `ws` → re-render. `w` đơn lẻ + `s` không vowel → literal "ws".
#[test]
fn xoa_phia_truoc_tren_telex_chinh_xac() {
    let mut p = phien();
    for c in "aws".chars() {
        p.them_ky_tu(c);
    }
    assert_eq!(p.ban_chup().noi_dung(), "ắ");
    p.ve_dau();
    p.xoa_phia_truoc();
    // Xóa raw `a`, còn `ws` → w literal, s không vowel → literal.
    assert_eq!(p.ban_chup().noi_dung(), "ws");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 0);
}

/// Delete-forward trên Telex: xóa giữa hai grapheme.
#[test]
fn xoa_phia_truoc_giua_telex() {
    let mut p = phien();
    // "ba" → 'b' + 'ă' (aw).
    p.them_ky_tu('b');
    for c in "aw".chars() {
        p.them_ky_tu(c);
    }
    assert_eq!(p.ban_chup().noi_dung(), "bă");
    // Cursor ở đầu, xóa forward 'b' → "ă".
    p.ve_dau();
    p.xoa_phia_truoc();
    assert_eq!(p.ban_chup().noi_dung(), "ă");
    assert_eq!(p.ban_chup().con_tro().chi_so_grapheme(), 0);
}
