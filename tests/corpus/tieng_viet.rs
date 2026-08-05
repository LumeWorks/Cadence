// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus tiếng Việt - enumeration có hệ thống: mọi âm đầu, âm cuối, nguyên
//! âm, dấu thanh, quy tắc đặt dấu, NFC/NFD input, viết hoa, chỉnh sửa giữa.
//!
//! Liên kết branch: `am_tiet.rs` (bảng âm đầu/âm cuối), `telex.rs` (hình chữ,
//! dấu thanh, nguyên âm chính), `render.rs` (NFC/NFD), `lua_chon.rs` (selection).

use cadence::{BoGo, CauHinh, ChinhSachLuaChon, DangUnicode, KieuTelex, LoaiNoiDung, QuyTacDatDau};
use unicode_normalization::UnicodeNormalization;

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

fn go_with(raw: &str, kieu: KieuTelex, quy_tac: QuyTacDatDau, dang: DangUnicode, cs: ChinhSachLuaChon) -> String {
    let mut c = CauHinh::mac_dinh();
    c.dat_kieu_telex(kieu);
    c.dat_quy_tac_dat_dau(quy_tac);
    c.dat_dang_unicode(dang);
    c.dat_chinh_sach_lua_chon(cs);
    let bo_go = BoGo::new(c).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for ch in raw.chars() {
        phien.them_ky_tu(ch);
    }
    phien.ban_chup().noi_dung().to_string()
}

/// Mọi âm đầu (onset) tiếng Việt + nguyên âm `a` → raw (không Telex modifier).
/// Phân đoạn: onset+`a` là một đoạn Chu, không tone → raw. Bất biến: raw giữ.
#[test]
fn moi_am_dau_voi_a() {
    let onsets = [
        "b", "c", "d", "đ", "g", "h", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "x",
        "ch", "gh", "gi", "kh", "ng", "ngh", "nh", "ph", "qu", "th", "tr",
    ];
    for o in onsets {
        let raw = format!("{o}a");
        assert_eq!(go(&raw), raw, "onset {o}");
    }
}

/// Mọi âm cuối (coda) sau nguyên âm `a` → raw.
#[test]
fn moi_am_cuoi_sau_a() {
    let codas = ["c", "m", "n", "p", "t", "ch", "ng", "nh"];
    for cd in codas {
        let raw = format!("a{cd}");
        assert_eq!(go(&raw), raw, "coda {cd}");
    }
}

/// Nguyên âm đơn + 6 dấu thanh → đúng ký tự dựng sẵn.
#[test]
fn nguyen_am_don_sau_thanh() {
    // (vowel, tone_key, expected_nfc)
    let cases = [
        ('a', 's', 'á'), ('a', 'f', 'à'), ('a', 'r', 'ả'), ('a', 'x', 'ã'), ('a', 'j', 'ạ'),
        ('e', 's', 'é'), ('e', 'f', 'è'), ('e', 'r', 'ẻ'), ('e', 'x', 'ẽ'), ('e', 'j', 'ẹ'),
        ('i', 's', 'í'), ('i', 'f', 'ì'), ('i', 'r', 'ỉ'), ('i', 'x', 'ĩ'), ('i', 'j', 'ị'),
        ('o', 's', 'ó'), ('o', 'f', 'ò'), ('o', 'r', 'ỏ'), ('o', 'x', 'õ'), ('o', 'j', 'ọ'),
        ('u', 's', 'ú'), ('u', 'f', 'ù'), ('u', 'r', 'ủ'), ('u', 'x', 'ũ'), ('u', 'j', 'ụ'),
        ('y', 's', 'ý'), ('y', 'f', 'ỳ'), ('y', 'r', 'ỷ'), ('y', 'x', 'ỹ'), ('y', 'j', 'ỵ'),
    ];
    for (v, t, exp) in cases {
        let raw = format!("{v}{t}");
        assert_eq!(go(&raw), exp.to_string(), "{v}+{t}");
    }
}

/// Hình chữ + 6 dấu thanh → đúng tổ hợp dựng sẵn.
#[test]
fn hinh_chu_sau_thanh() {
    // (shape_raw, expected_nfc)
    let cases = [
        ("aws", "ắ"), ("awf", "ằ"), ("awr", "ẳ"), ("awx", "ẵ"), ("awj", "ặ"),
        ("aas", "ấ"), ("aaf", "ầ"), ("aar", "ẩ"), ("aax", "ẫ"), ("aaj", "ậ"),
        ("ows", "ớ"), ("owf", "ờ"), ("owr", "ở"), ("owx", "ỡ"), ("owj", "ợ"),
        ("ees", "ế"), ("eef", "ề"), ("eer", "ể"), ("eex", "ễ"), ("eej", "ệ"),
        ("oos", "ố"), ("oof", "ồ"), ("oor", "ổ"), ("oox", "ỗ"), ("ooj", "ộ"),
        ("uws", "ứ"), ("uwf", "ừ"), ("uwr", "ử"), ("uwx", "ữ"), ("uwj", "ự"),
    ];
    for (raw, exp) in cases {
        assert_eq!(go(raw), exp, "{raw}");
    }
}

/// Cụm đôi `ua`/`uo`/`ie`/`ưa`/`ưo` và tone placement.
#[test]
fn cum_doi_tone() {
    // `hoa` → `hoa`; `hoas` (HienDai mặc định) → `hóa` (tone trên o).
    assert_eq!(go("hoas"), "hóa");
    assert_eq!(go("hoaf"), "hòa");
    // TruyenThong: tone trên `a` → `hoá`.
    assert_eq!(
        go_with("hoas", KieuTelex::CanBang, QuyTacDatDau::TruyenThong, DangUnicode::Nfc, ChinhSachLuaChon::TuNhien),
        "hoá"
    );
    // `uow` → `ươ` (tam nguyên âm, không tone). `ươ` = ư(U+1B0) + ơ(U+01A1).
    assert_eq!(go("uow"), "\u{1B0}\u{01A1}");
    // Triphthong `nguowif` → `người`.
    assert_eq!(go("nguowif"), "người");
    // `dduwowngf` → `đường`.
    assert_eq!(go("dduwowngf"), "đường");
}

/// `qu` + vowel: `quyen` → `quyen`, `quyens` → `quyén` (tone trên y).
#[test]
fn qu_vowel() {
    assert_eq!(go("quyen"), "quyen");
    assert_eq!(go("quyens"), "quyén");
}

/// `gi` + vowel: `gien` → `gien`, `giens` → `gién`.
#[test]
fn gi_vowel() {
    assert_eq!(go("gien"), "gien");
    assert_eq!(go("giens"), "gién");
}

/// `gh`/`ngh` trước nguyên âm lùi: `ghi`/`nghia` → raw.
#[test]
fn gh_ngh() {
    assert_eq!(go("ghi"), "ghi");
    assert_eq!(go("nghia"), "nghia");
}

/// Viết hoa: đầu câu, toàn bộ, mixed-case.
#[test]
fn viet_hoa() {
    assert_eq!(go("Tieengs"), "Tiếng");
    assert_eq!(go("TIEENGS"), "TIẾNG");
    assert_eq!(go("AA"), "Â");
    assert_eq!(go("DD"), "Đ");
    assert_eq!(go("OW"), "Ơ");
    assert_eq!(go("UW"), "Ư");
}

/// Input NFC (chữ dựng sẵn) qua them_ky_tu → giữ nguyên, AmTietTiengViet.
#[test]
fn input_nfc_dung_san() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in "tiếng".chars() {
        phien.them_ky_tu(c);
    }
    assert_eq!(phien.ban_chup().noi_dung(), "tiếng");
    // Raw giữ nguyên.
    assert_eq!(phien.ban_chup().noi_dung_goc(), "tiếng");
}

/// Input NFD (combining mark) → canonical equivalent với NFC input.
#[test]
fn input_nfd_tuong_duong_nfc() {
    let nfd_input: String = "tiếng".nfd().collect();
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien_nfd = bo_go.tao_phien();
    for c in nfd_input.chars() {
        phien_nfd.them_ky_tu(c);
    }
    let out_nfd = phien_nfd.ban_chup().noi_dung().to_string();
    // NFC output của cùng semantic phải canonical equivalent.
    assert_eq!(out_nfd.nfd().collect::<String>(), "tiếng".nfd().collect::<String>());
    // Raw giữ byte-for-byte.
    assert_eq!(phien_nfd.ban_chup().noi_dung_goc(), nfd_input);
}

/// Direct Vietnamese input và Telex tương đương khi cùng output semantic.
#[test]
fn direct_va_telex_tuong_duong() {
    let direct = go_with("tiếng", KieuTelex::CanBang, QuyTacDatDau::HienDai, DangUnicode::Nfc, ChinhSachLuaChon::TuNhien);
    let telex = go_with("tieengs", KieuTelex::CanBang, QuyTacDatDau::HienDai, DangUnicode::Nfc, ChinhSachLuaChon::TuNhien);
    assert_eq!(direct.nfd().collect::<String>(), telex.nfd().collect::<String>());
}

/// Output NFC idempotent dưới NFC; NFD idempotent dưới NFD.
#[test]
fn nfc_nfd_idempotent() {
    let nfc = go_with("tieengs", KieuTelex::CanBang, QuyTacDatDau::HienDai, DangUnicode::Nfc, ChinhSachLuaChon::TuNhien);
    let nfd = go_with("tieengs", KieuTelex::CanBang, QuyTacDatDau::HienDai, DangUnicode::Nfd, ChinhSachLuaChon::TuNhien);
    assert_eq!(nfc.nfc().collect::<String>(), nfc, "NFC idempotent");
    assert_eq!(nfd.nfd().collect::<String>(), nfd, "NFD idempotent");
}

/// LoaiNoiDung cho tiếng Việt hoàn chỉnh là AmTietTiengViet.
#[test]
fn loai_am_tiet_viet() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in "tieengs".chars() {
        phien.them_ky_tu(c);
    }
    assert_eq!(phien.ban_chup().loai_noi_dung(), LoaiNoiDung::AmTietTiengViet);
}

/// Chỉnh sửa giữa âm tiết: `tieengs` → về giữa → chèn → xóa → phục hồi.
#[test]
fn chinh_sua_giua_am_tiet() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in "tieengs".chars() {
        phien.them_ky_tu(c);
    }
    let truoc = phien.ban_chup().noi_dung().to_string();
    // Về giữa, chèn 'x' nguyên bản (chặn Telex), rồi xóa lùi.
    phien.ve_dau();
    phien.di_phai();
    phien.them_nguyen_ban('x');
    phien.xoa_lui();
    assert_eq!(phien.ban_chup().noi_dung(), truoc, "xoa chen phuc hoi");
}
