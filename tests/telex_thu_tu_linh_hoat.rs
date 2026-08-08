// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test thứ tự tone/hình chữ linh hoạt cho Telex (RFC 0006: shape modifier
//! reach back trong đoạn, parity VNI RFC 0021).
//!
//! Trước đây shape modifier yêu cầu adjacency (`ow`→ơ nhưng `oiw`→raw).
//! Giờ shape reach back tới base trần qua bán âm và phụ âm: `oiw`→ơi,
//! `voiws`→với, `khongo`→không.

use cadence::{BoGo, CauHinh};

fn go(raw: &str) -> String {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.ban_chup().noi_dung().to_string()
}

// --- Tone linh hoạt (đã work từ trước, regression) ---

#[test]
fn vowsi_thanh_voi() {
    assert_eq!(go("vowsi"), "với");
}

#[test]
fn ows_thanh_o_sac() {
    assert_eq!(go("ows"), "ớ");
}

// --- Hình chữ linh hoạt: w reach back qua bán âm ---

#[test]
fn oiw_thanh_oi() {
    assert_eq!(go("oiw"), "ơi");
}

#[test]
fn voiws_thanh_voi() {
    assert_eq!(go("voiws"), "với");
}

#[test]
fn moiws_thanh_moi_sac() {
    // `s` = sắc → m + ớ(sắc) + i = "mới". ("mới" = sắc, theo bảng user.)
    assert_eq!(go("moiws"), "m\u{1EDB}i");
}

#[test]
fn moiwr_thanh_hoi() {
    // `r` = hỏi → m + ở(ơ+hỏi) + i. Khác "mới" (sắc).
    assert_eq!(go("moiwr"), "m\u{1EDF}i");
}

#[test]
fn uoiw_thanh_uoi() {
    // Không có tone key → ươi (ngang, không dấu thanh).
    assert_eq!(go("uoiw"), "\u{1B0}\u{1A1}i");
}

#[test]
fn nguoiw_thanh_nguoi_khong_dau() {
    // `nguoiw` không có tone key → "ngươi" (ngang). "người" cần f (huyền).
    assert_eq!(go("nguoiw"), "ng\u{1B0}\u{1A1}i");
}

#[test]
fn nguoiwf_thanh_nguoi() {
    // `nguoiw` + `f` (huyền) → ươ + huyền trên ư → người.
    assert_eq!(go("nguoiwf"), "ng\u{1B0}\u{1EDD}i");
}

// --- Hình chữ linh hoạt: w reach back qua phụ âm (restroke) ---

#[test]
fn khongo_thanh_khong() {
    assert_eq!(go("khongo"), "không");
}

#[test]
fn uongw_thanh_uong() {
    assert_eq!(go("uongw"), "ương");
}

#[test]
fn huongw_thanh_huong() {
    assert_eq!(go("huongw"), "hương");
}

#[test]
fn trongo_thanh_trong() {
    // `oo` reach back tới `o` đầu qua phụ âm `ng` → ô + ng.
    assert_eq!(go("trongo"), "trông");
}

// --- Tác dụng phụ parity VNI (input không dấu cách collapse) ---
// Giống VNI: `di9`→đi, `an8`→ăn, `ol7`→ơl. Telex giờ hành xử tương tự.

#[test]
fn did_thanh_di() {
    // parity VNI: `di9`→đi.
    assert_eq!(go("did"), "đi");
}

#[test]
fn anw_thanh_an() {
    // parity VNI: `an8`→ăn.
    assert_eq!(go("anw"), "ăn");
}

#[test]
fn olw_thanh_ol() {
    // parity VNI: `ol7`→ơl. Âm tiết `ơl` không hợp lệ (l là coda? không) →
    // nhưng shape-far gate chặn nếu không hợp lệ. `ơl`: onset rỗng, vowel ơ,
    // coda `l` không hợp lệ → KhongHopLe → raw? Kiểm tra thực tế.
    // `ol` + w reach back: o→ơ, l phụ âm. `phan_tich_am_tiet("ơl")`: onset 0,
    // sau_dau="ơl", vowel đầu ơ OK, coda l? không match → van="ơl" chứa l
    // (không nguyên âm) → KhongHopLe. co_hinh_xa → raw.
    assert_eq!(go("olw"), "olw");
}

// --- Escape linh hoạt ---

#[test]
fn oiww_escape_thanh_oiw() {
    // `oiw`→ơi, `w` thứ 2 lặp → escape → oiw (hoàn tác ơ).
    assert_eq!(go("oiww"), "oiw");
}

#[test]
fn khongoo_escape_thanh_khongo() {
    // `khongo`→không, `o` thứ 3 lặp `o` (modifier `oo`/Mu) → escape.
    assert_eq!(go("khongoo"), "khongo");
}

#[test]
fn anww_escape_thanh_anw() {
    // `anw`→ăn, `w` thứ 2 lặp → escape → anw.
    assert_eq!(go("anww"), "anw");
}

#[test]
fn didd_escape_thanh_did() {
    // `did`→đi, `d` thứ 2 lặp → escape (d là Gach modifier) → did.
    assert_eq!(go("didd"), "did");
}

// --- Regression: adjacency vẫn hoạt động ---

#[test]
fn aa_thanh_a_mu() {
    assert_eq!(go("aa"), "â");
}

#[test]
fn ee_thanh_e_mu() {
    assert_eq!(go("ee"), "ê");
}

#[test]
fn oo_thanh_o_mu() {
    assert_eq!(go("oo"), "ô");
}

#[test]
fn dd_thanh_d_gach() {
    assert_eq!(go("dd"), "đ");
}

#[test]
fn nguowif_thanh_nguoi() {
    // Forward ươ + tone: vẫn hoạt động.
    assert_eq!(go("nguowif"), "người");
}

#[test]
fn ddm_thanh_dm() {
    // Shape liền + onset hợp lệ (đ) → Telex dù `đm` chưa phải âm tiết.
    assert_eq!(go("ddm"), "đm");
}

#[test]
fn bw_giu_nguyen() {
    // `b` không phải base cho w → w không reach back → bw raw? Hay w literal?
    // `bw`: b phụ âm, w không tìm base (b không trong cap_hinh_chu) → w là
    // shape candidate không tìm base → fall through → w là ký tự thường, không
    // phải vowel/D → literal `w`. Output `bw`.
    assert_eq!(go("bw"), "bw");
}

#[test]
fn khong_khong_bien_doi() {
    // `khong` (không có modifier) → raw, không thành `không`.
    assert_eq!(go("khong"), "khong");
}

#[test]
fn w_don_le_can_bang_literal() {
    // CanBang: `w` đơn lẻ không tìm base → literal.
    assert_eq!(go("w"), "w");
}

// --- Regression: escape lặp phím ---

#[test]
fn aaa_escape_thanh_aa() {
    assert_eq!(go("aaa"), "aa");
}

#[test]
fn ooo_escape_thanh_oo() {
    assert_eq!(go("ooo"), "oo");
}

#[test]
fn aww_escape_thanh_aw() {
    assert_eq!(go("aww"), "aw");
}

#[test]
fn ddd_escape_thanh_dd() {
    assert_eq!(go("ddd"), "dd");
}

#[test]
fn eee_escape_thanh_ee() {
    assert_eq!(go("eee"), "ee");
}

#[test]
fn aaw_khong_restroke() {
    // `aa`→â (Mu), `w` không restroke được (â đã có dau_chu) → âw.
    assert_eq!(go("aaw"), "âw");
}

// --- Regression: tone escape ---

#[test]
fn ass_escape_thanh_as() {
    assert_eq!(go("ass"), "as");
}

#[test]
fn asss_ap_lai_sac() {
    // `ass`→as (escape), `s` thứ 3 áp sắc → ás.
    assert_eq!(go("asss"), "ás");
}

#[test]
fn zz_khong_escape() {
    // z không xóa được (không có dấu) → 2 literal.
    assert_eq!(go("zz"), "zz");
}

// --- Regression: tiếng Anh không reshape (gate shape-far) ---

#[test]
fn cadence_khong_reshape() {
    // `cadence`: `e` cuối reach back tới `a`? Không: cap_hinh_chu('a','e')
    // = None. `e` reach back tới `e` đầu? `e`+`e`=Mu → nhưng `e` cuối cách
    // `e` đầu qua `ad`/`c`. co_hinh_xa + `phan_tich_am_tiet("cadênc")` =
    // KhongHopLe → raw.
    assert_eq!(go("cadence"), "cadence");
}

#[test]
fn release_khong_reshape() {
    // `release`: `e` cuối reach back tới `e` đầu qua phụ âm `l`/`e`/`a`/`s`
    // → `rêláe` không hợp lệ → raw.
    assert_eq!(go("release"), "release");
}

#[test]
fn httpserver_khong_reshape() {
    // `HTTPServer`: `e` cuối reach back tới `e` đầu → reshape nhưng không
    // hợp lệ → raw.
    assert_eq!(go("HTTPServer"), "HTTPServer");
}

#[test]
fn deadbeefcafe_khong_reshape() {
    // `deadbeefcafe`: nhiều reshape nhưng không hợp lệ → raw.
    assert_eq!(go("deadbeefcafe"), "deadbeefcafe");
}

// --- Regression: tone-only tiếng Anh vẫn reshape (không shape) ---

#[test]
fn text_thanh_tet() {
    // `text` = t + ẽ + t (tone only, no shape) → `tẽt`. Vẫn như cũ.
    assert_eq!(go("text"), "tẽt");
}

#[test]
fn use_thanh_ue() {
    // `use` = u + sắc + e → `úe`. Tone only.
    assert_eq!(go("use"), "úe");
}

#[test]
fn char_thanh_char() {
    // `char` = ch + ả + r → `chả`. Tone only.
    assert_eq!(go("char"), "chả");
}

// --- Case preservation ---

#[test]
fn ow_hoa_thanh_o_hoa() {
    assert_eq!(go("OW"), "Ơ");
}

#[test]
fn aa_hoa_thanh_a_hoa() {
    assert_eq!(go("AA"), "Â");
}

#[test]
fn dd_hoa_thanh_d_hoa() {
    assert_eq!(go("DD"), "Đ");
}

#[test]
fn khongo_hoa_thanh_khong_hoa() {
    // `KHONGO` → KHÔNG (ô hoa theo base `O` đầu).
    assert_eq!(go("KHONGO"), "KHÔNG");
}
