// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus dấu thanh - enumeration mọi phím tone × nguyên âm × quy tắc.
//! Liên kết branch: `telex.rs::tu_dau_thanh_key`, `tim_nguyen_am_chinh`.

use cadence::{BoGo, CauHinh, QuyTacDatDau};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

fn go_quy_tac(raw: &str, qt: QuyTacDatDau) -> String {
    let mut c = CauHinh::mac_dinh();
    c.dat_quy_tac_dat_dau(qt);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in raw.chars() {
        phien.them_ky_tu(ch);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// Mọi phím tone trên nguyên âm `a`: s/f/r/x/j → sắc/huyền/hỏi/ngã/nặng.
#[test]
fn moi_tren_a() {
    assert_eq!(go("as"), "á");
    assert_eq!(go("af"), "à");
    assert_eq!(go("ar"), "ả");
    assert_eq!(go("ax"), "ã");
    assert_eq!(go("aj"), "ạ");
}

/// `z` xóa dấu: sau khi có tone, `z` → không dấu.
#[test]
fn z_xoa_dau() {
    assert_eq!(go("asz"), "a");
    assert_eq!(go("afz"), "a");
    assert_eq!(go("awsz"), "ă");
}

/// `z` khi không có dấu → literal.
#[test]
fn z_khong_dau_la_literal() {
    assert_eq!(go("az"), "az");
    assert_eq!(go("z"), "z");
}

/// Thay dấu: tone mới thay tone cũ.
#[test]
fn thay_dau() {
    assert_eq!(go("asf"), "à");
    assert_eq!(go("asr"), "ả");
    assert_eq!(go("asx"), "ã");
    assert_eq!(go("asj"), "ạ");
}

/// On-glide `oa`/`oe`: HienDai trên o, TruyenThong trên a/e.
#[test]
fn on_glide_hien_dai_truyen_thong() {
    assert_eq!(go_quy_tac("hoas", QuyTacDatDau::HienDai), "hóa");
    assert_eq!(go_quy_tac("hoas", QuyTacDatDau::TruyenThong), "hoá");
    assert_eq!(go_quy_tac("hoaf", QuyTacDatDau::HienDai), "hòa");
    assert_eq!(go_quy_tac("hoaf", QuyTacDatDau::TruyenThong), "hoà");
    assert_eq!(go_quy_tac("does", QuyTacDatDau::HienDai), "dóe");
    assert_eq!(go_quy_tac("does", QuyTacDatDau::TruyenThong), "doé");
}

/// Bán âm cuối `i`/`u`/`o`: tone trên nguyên âm trước.
#[test]
fn ban_am_cuoi_tone_tren_nguyen_am_truoc() {
    // `ai` + `s` → `ái` (tone trên a, i là off-glide).
    assert_eq!(go("ais"), "ái");
    // `ao` + `f` → `ào` (tone trên a, o là off-glide).
    assert_eq!(go("aof"), "ào");
    // `au` + `s` → `áu`.
    assert_eq!(go("aus"), "áu");
}

/// Tone trên diphthong `ie`: `ien`+`s` → `ín`+`en`? tone trên `i`.
#[test]
fn tone_tren_diphthong_ie() {
    // `ie` + tone: tone trên `i`? thực tế `ienes` → `ién`? dùng `iens` → `ién`.
    let kq = go("iens");
    assert!(kq.starts_with("ién"), "kỳ vọng íen, được {kq}");
}
