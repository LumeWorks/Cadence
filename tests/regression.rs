// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Regression suite — khóa các case lỗi cụ thể đã phát hiện trong Phase 1.
//!
//! Mỗi test tại đây ghi lại một tình huống đã từng sai, với input tối thiểu,
//! để chặn quay lại. Các bất biến tổng quát nằm ở `property.rs`, test chức
//! năng ở các file khác; file này chỉ giữ các regression tối thiểu.

use cadence::{BoGo, CauHinh, KetQuaXuLy};

fn tao_phien() -> cadence::PhienGo {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh hop le");
    bo_go.tao_phien()
}

/// Regression: ký tự tiếng Việt dựng sẵn trong BMP (U+1Exx) dài 3 byte
/// UTF-8, không phải 2 byte. Lỗi này làm test Unicode ban đầu fail khi
/// giả định sai độ dài byte.
#[test]
fn reg_ky_tu_dung_san_ba_byte() {
    let mut phien = tao_phien();
    phien.them_ky_tu('ế');
    let ban_chup = phien.ban_chup();
    assert_eq!(ban_chup.con_tro().chi_so_byte(), 3);
    assert_eq!(ban_chup.con_tro().chi_so_utf16(), 1);
    assert_eq!(ban_chup.con_tro().chi_so_grapheme(), 1);
}

/// Regression: khi con trỏ nội bộ nằm giữa hai `char` thuộc cùng grapheme
/// cluster ('e' + combining mark), vị trí grapheme công bố phải là ranh
/// giới cluster, không phải nằm giữa cluster.
#[test]
fn reg_con_tro_giua_cluster_snap_ve_ranh_gioi() {
    let mut phien = tao_phien();
    phien.them_ky_tu('e');
    phien.them_ky_tu('\u{0301}');
    // Con trỏ giữa 'e' và combining mark (trong cluster).
    phien.ve_dau();
    phien.di_phai();
    let ban_chup = phien.ban_chup();
    let idx = ban_chup.con_tro().chi_so_grapheme();
    // Toàn bộ chuỗi chỉ có 1 grapheme; index hợp lệ là 0 hoặc 1.
    assert!(
        idx == 0 || idx == 1,
        "grapheme index phai la ranh gioi, duoc {idx}"
    );
    // Byte index vẫn là ranh giới UTF-8 hợp lệ.
    assert!(
        ban_chup
            .noi_dung()
            .is_char_boundary(ban_chup.con_tro().chi_so_byte())
    );
}

/// Regression: `chap_nhan` phải phân biệt phiên rỗng vs có nội dung. Với
/// chuỗi thao tác chứa `Reset` rồi `chap_nhan`, phiên rỗng phải trả
/// `KhongDoi`. Lỗi này làm property test `commit_roi_reset_sach` fail với
/// input tối thiểu `[Reset]`.
#[test]
fn reg_commit_sau_reset_tra_khong_doi() {
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    phien.dat_lai(); // Reset -> phiên rỗng.
    let ket_qua = phien.chap_nhan();
    assert!(matches!(ket_qua, KetQuaXuLy::KhongDoi));
    assert!(phien.dang_trong());
}

/// Regression: commit phiên có nội dung phải trả đúng chuỗi và không rò
/// state; input `[Reset]` cũng phải an toàn khi commit sau khi có nội dung.
#[test]
fn reg_commit_co_noi_dung_khong_ro_state() {
    let mut phien = tao_phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('b');
    match phien.chap_nhan() {
        KetQuaXuLy::ChapNhan { noi_dung } => assert_eq!(noi_dung, "ab"),
        _ => panic!("commit phai tra ChapNhan"),
    }
    // Token mới không rò state cũ.
    phien.them_ky_tu('c');
    assert_eq!(phien.ban_chup().noi_dung(), "c");
}

/// Regression: xóa phía trước ở cuối phiên (con trỏ ở cuối) không thay đổi
/// state — chặn quay lại trường hợp xóa sai index khi con trỏ bằng độ dài.
#[test]
fn reg_xoa_phia_truoc_o_cuoi_khong_doi() {
    let mut phien = tao_phien();
    for ky_tu in "abc".chars() {
        phien.them_ky_tu(ky_tu);
    }
    let ket_qua = phien.xoa_phia_truoc();
    assert!(matches!(ket_qua, KetQuaXuLy::KhongDoi));
    assert_eq!(phien.ban_chup().noi_dung(), "abc");
}

/// Regression: đạt giới hạn rồi xóa rồi thêm lại được — chặn trường hợp
/// giới hạn bị "khoá vĩnh viễn" sau khi chạm ngưỡng.
#[test]
fn reg_gioi_han_xoa_roi_them_lai() {
    let mut cau_hinh = CauHinh::mac_dinh();
    cau_hinh.dat_gioi_han_thao_tac(2).expect("2 phai hop le");
    let bo_go = BoGo::new(cau_hinh).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    phien.them_ky_tu('a');
    phien.them_ky_tu('b');
    assert!(matches!(phien.them_ky_tu('c'), KetQuaXuLy::KhongDoi));
    phien.xoa_lui();
    assert!(matches!(phien.them_ky_tu('d'), KetQuaXuLy::CapNhat));
    assert_eq!(phien.ban_chup().noi_dung(), "ad");
}

/// Regression: `khoi_phuc_nguyen_ban` hiện là no-op (Phase 2). Pipeline luôn
/// rebuild từ raw, nên method trả `KhongDoi` và không thay đổi snapshot.
/// Phase 3 sẽ triển khai toggle thực sự.
#[test]
fn reg_khoi_phuc_nguyen_ban_la_no_op_phase2() {
    let mut phien = tao_phien();
    for c in "tieengs".chars() {
        phien.them_ky_tu(c);
    }
    let truoc = phien.ban_chup().noi_dung().to_string();
    let ket_qua = phien.khoi_phuc_nguyen_ban();
    assert!(matches!(ket_qua, KetQuaXuLy::KhongDoi));
    assert_eq!(phien.ban_chup().noi_dung(), truoc);
}
