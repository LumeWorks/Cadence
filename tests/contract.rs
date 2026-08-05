// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Compile-time trait contract cho public type.
//!
//! Các hàm generic dưới đây chỉ biên dịch nếu type implement trait tương
//! ứng. Nếu một type mất trait (vd thêm `Rc` nội bộ), file này fail compile.
//! Không runtime overhead - đây là assertion ở tầng type.

#![cfg(feature = "std")]

use cadence::{
    BanChupSoan, BoGo, CauHinh, ChinhSachLuaChon, DangUnicode, KetQuaXuLy, KieuTelex, LoaiNoiDung,
    LoiCauHinh, PhienGo, QuyTacDatDau, ViTriVanBan,
};

/// Khẳng định `T: Send` - có thể chuyển quyền sở hữu sang thread khác.
fn khang_dinh_send<T: Send>() {}

/// Khẳng định `T: Sync` - `&T` có thể chia sẻ giữa các thread.
fn khang_dinh_sync<T: Sync>() {}

/// Khẳng định `T: Clone` - snapshot/config có thể clone.
fn khang_dinh_clone<T: Clone>() {}

/// Khẳng định `T: 'static` - không chứa reference có lifetime.
fn khang_dinh_static<T: 'static>() {}

#[test]
fn cau_hinh_send_sync() {
    khang_dinh_send::<CauHinh>();
    khang_dinh_sync::<CauHinh>();
    khang_dinh_clone::<CauHinh>();
    khang_dinh_static::<CauHinh>();
}

#[test]
fn bo_go_send_sync() {
    khang_dinh_send::<BoGo>();
    khang_dinh_sync::<BoGo>();
    khang_dinh_clone::<BoGo>();
    khang_dinh_static::<BoGo>();
}

#[test]
fn phien_go_send_sync() {
    // Send: phiên có thể chuyển sang thread khác sở hữu độc quyền.
    khang_dinh_send::<PhienGo>();
    // Sync: &PhienGo có thể chia sẻ cho đọc (ban_chup, trace, dang_trong).
    khang_dinh_static::<PhienGo>();
    // Sync chỉ đọc - khẳng định dưới đây chứng minh type không chứa cell nội bộ.
    khang_dinh_sync::<PhienGo>();
}

#[test]
fn ban_chup_soan_send_sync() {
    khang_dinh_send::<BanChupSoan>();
    khang_dinh_sync::<BanChupSoan>();
    khang_dinh_clone::<BanChupSoan>();
    khang_dinh_static::<BanChupSoan>();
}

#[test]
fn vi_tri_van_ban_send_sync() {
    khang_dinh_send::<ViTriVanBan>();
    khang_dinh_sync::<ViTriVanBan>();
    khang_dinh_clone::<ViTriVanBan>();
    khang_dinh_static::<ViTriVanBan>();
}

#[test]
fn ket_qua_xu_ly_send_sync() {
    khang_dinh_send::<KetQuaXuLy>();
    khang_dinh_sync::<KetQuaXuLy>();
    khang_dinh_clone::<KetQuaXuLy>();
    khang_dinh_static::<KetQuaXuLy>();
}

#[test]
fn loai_noi_dung_send_sync() {
    khang_dinh_send::<LoaiNoiDung>();
    khang_dinh_sync::<LoaiNoiDung>();
    khang_dinh_clone::<LoaiNoiDung>();
}

#[test]
fn cac_enum_send_sync() {
    khang_dinh_send::<KieuTelex>();
    khang_dinh_sync::<KieuTelex>();
    khang_dinh_send::<QuyTacDatDau>();
    khang_dinh_sync::<QuyTacDatDau>();
    khang_dinh_send::<DangUnicode>();
    khang_dinh_sync::<DangUnicode>();
    khang_dinh_send::<ChinhSachLuaChon>();
    khang_dinh_sync::<ChinhSachLuaChon>();
    khang_dinh_send::<LoiCauHinh>();
    khang_dinh_sync::<LoiCauHinh>();
}

/// Khẳng định `T: Send + Sync` thực sự bằng cách spawn thread.
/// Đây là runtime smoke test bổ sung cho contract compile-time.
#[test]
fn phien_go_send_thuc_su_chuyen_thread() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh hop le");
    let mut phien = bo_go.tao_phien();
    phien.them_ky_tu('a');
    let out = std::thread::spawn(move || {
        // Phiên được chuyển sang thread khác, thao tác tiếp, trả kết quả.
        phien.them_ky_tu('b');
        let out = phien.ban_chup().noi_dung().to_string();
        let _ = phien.chap_nhan();
        out
    })
    .join()
    .expect("thread khong panic");
    // `a` từ thread chính + `b` từ thread phụ.
    assert_eq!(out, "ab");
}

/// `&BoGo` chia sẻ giữa thread trong scope: clone phiên độc lập từ cùng
/// reference. Dùng `std::thread::scope` để mượn non-`'static`. Chứng minh
/// `BoGo: Sync` thực sự.
#[test]
fn bo_go_sync_thuc_su_chia_se() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh hop le");
    let ket_qua = std::thread::scope(|s| {
        let t1 = s.spawn(|| {
            let mut phien = bo_go.tao_phien();
            phien.them_ky_tu('x');
            phien.ban_chup().noi_dung().to_string()
        });
        let t2 = s.spawn(|| {
            let mut phien = bo_go.tao_phien();
            phien.them_ky_tu('y');
            phien.ban_chup().noi_dung().to_string()
        });
        (t1.join().expect("t1"), t2.join().expect("t2"))
    });
    assert_eq!(ket_qua.0, "x");
    assert_eq!(ket_qua.1, "y");
}

/// `&PhienGo` chia sẻ chỉ đọc giữa thread trong scope. Chứng minh
/// `PhienGo: Sync` cho đọc (ban_chup, dang_trong).
#[test]
fn phien_go_sync_thuc_su_chia_se_doc() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in "abc".chars() {
        phien.them_ky_tu(c);
    }
    let phien = &phien;
    let ket_qua = std::thread::scope(|s| {
        let t1 = s.spawn(|| phien.ban_chup().noi_dung().to_string());
        let t2 = s.spawn(|| phien.dang_trong());
        (t1.join().expect("t1"), t2.join().expect("t2"))
    });
    assert_eq!(ket_qua.0, "abc");
    assert!(!ket_qua.1);
}
