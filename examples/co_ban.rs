// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Ví dụ cơ bản: tạo bộ gõ, mở phiên, thêm ký tự và commit.

use cadence::{BoGo, CauHinh, KetQuaXuLy};

fn main() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh luon hop le");
    let mut phien = bo_go.tao_phien();

    // Nhập "Cadence" (Phase 1: render nguyên bản, chưa có Telex).
    for ky_tu in "Cadence".chars() {
        phien.them_ky_tu(ky_tu);
    }

    // Đọc snapshot hiện tại.
    let ban_chup = phien.ban_chup();
    println!("Dang soan: {}", ban_chup.noi_dung());
    println!(
        "Con tro (byte/utf16/grapheme): {}/{}/{}",
        ban_chup.con_tro().chi_so_byte(),
        ban_chup.con_tro().chi_so_utf16(),
        ban_chup.con_tro().chi_so_grapheme(),
    );

    // Chỉnh sửa giữa đoạn: đưa con trỏ về sau 'C', chèn 'a', xóa phía trước.
    phien.ve_dau();
    phien.di_phai();
    phien.them_ky_tu('a');
    // Snapshot bây giờ: "Caadence".
    println!("Sau chen giua: {}", phien.ban_chup().noi_dung());

    // Commit — phiên trả nội dung và tự đặt lại.
    match phien.chap_nhan() {
        KetQuaXuLy::ChapNhan { noi_dung } => {
            println!("Da commit: {noi_dung}");
        }
        KetQuaXuLy::KhongDoi => println!("Phien rong, khong commit gi"),
        KetQuaXuLy::CapNhat => println!("CapNhat (khong xay ra voi chap_nhan)"),
    }

    // Phiên rỗng hoàn toàn sau commit; thêm lại không rò state cũ.
    assert!(phien.dang_trong());
    phien.them_ky_tu('!');
    println!("Token moi: {}", phien.ban_chup().noi_dung());
}
