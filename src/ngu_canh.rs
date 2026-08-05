// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Nhận diện ngữ cảnh kỹ thuật xuyên đoạn và bằng chứng lựa chọn.
//!
//! Phase 3 tách nhận diện cấu trúc (URL, email, đường dẫn, code span/fence,
//! namespace `::`, phép gán `=`) ra khỏi Telex. Các cấu trúc chắc chắn buộc
//! đoạn chữ liên quan giữ nguyên bản, ngăn Telex biến đổi một segment thành
//! âm tiết Việt hợp lệ trong bối cảnh kỹ thuật (vd `bar` sau `::`, `buf`
//! trước `=`, `as` trong `http://x/as`).
//!
//! Không dùng regex, từ điển lớn hay parser framework. Chỉ match bảng ký tự
//! ASCII và cấu trúc đoạn tuyến tính.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use crate::phan_doan::{Doan, LoaiDoan};
use crate::thao_tac::ThaoTacNhap;

/// Bằng chứng lựa chọn cho một đoạn. Dùng cho trace và quyết định.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BangChungLuaChon {
    /// Âm tiết tiếng Việt hoàn chỉnh (onset + nucleus + coda hợp lệ).
    AmTietTiengVietHoanChinh,
    /// Biến đổi hình chữ rõ (aa/ee/oo/dd/aw/ow/uw).
    BienDoiHinhChuRoRang,
    /// Phím dấu thanh hợp lệ áp dụng lên nguyên âm.
    PhimDauHopLe,
    /// Phân cách identifier (`_`, `-`, ranh giới CamelCase).
    PhanCachIdentifier,
    /// Cấu trúc URL (`://` hoặc scheme).
    CauTrucUrl,
    /// Cấu trúc email (`local@domain`).
    CauTrucEmail,
    /// Cấu trúc đường dẫn (`/`, `~/`, `./`, `../`, `X:\`).
    CauTrucDuongDan,
    /// Cấu trúc command (token bắt đầu bằng `-`/`--`).
    CauTrucCommand,
    /// Chuỗi số/version/hash/UUID.
    ChuoiSoKyThuat,
    /// Ký tự lặp thể hiện cảm xúc (teencode, emoticon).
    KyTuLapChat,
    /// Emoticon (`=)`, `:)`, `:D`, ...).
    Emoticon,
    /// Người gọi yêu cầu nguyên bản (`them_nguyen_ban`).
    NguyenBanDoNguoiGoiYeuCau,
}

/// Kết quả nhận diện mỗi đoạn: có buộc raw không, và bằng chứng.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct KetQuaNhanDien {
    /// `true` nếu đoạn phải giữ nguyên bản (bỏ Telex).
    pub(crate) bat_buoc_raw: bool,
    /// Bằng chứng quyết định (dùng cho trace).
    pub(crate) bang_chung: BangChungLuaChon,
}

/// Trả chuỗi raw của một đoạn.
fn raw_doan(thao_tac: &[ThaoTacNhap], doan: &Doan) -> String {
    thao_tac[doan.bat_dau..doan.ket_thuc]
        .iter()
        .map(|t| t.ky_tu)
        .collect()
}

/// Trả `true` nếu chuỗi `s` chứa ký tự `c`.
fn chua(s: &str, c: char) -> bool {
    s.chars().any(|x| x == c)
}

/// Tìm segment KyThuat tiếp theo sau `i`, bỏ qua KhoangTrang. Trả index.
fn tim_ky_thuat_sau(cac_doan: &[Doan], i: usize) -> Option<usize> {
    let mut j = i + 1;
    while j < cac_doan.len() {
        match cac_doan[j].loai {
            LoaiDoan::KhoangTrang => j += 1,
            LoaiDoan::KyThuat => return Some(j),
            _ => return None,
        }
    }
    None
}

/// Tìm segment KyThuat ngay trước `i`, bỏ qua KhoangTrang. Trả index.
fn tim_ky_thuat_truoc(cac_doan: &[Doan], i: usize) -> Option<usize> {
    i.checked_sub(1).and_then(|mut j| {
        loop {
            match cac_doan[j].loai {
                LoaiDoan::KhoangTrang if j > 0 => j -= 1,
                LoaiDoan::KhoangTrang => return None,
                LoaiDoan::KyThuat => return Some(j),
                _ => return None,
            }
        }
    })
}

/// Nhận diện URL dạng `scheme://...`: Chu + KyThuat(`://`) + (non-ws)+.
/// Dùng tín hiệu mạnh `://` để tránh false-positive trên `hoaf.com`.
fn nhan_url(raws: &[String], cac_doan: &[Doan], i: usize) -> Option<usize> {
    let n = cac_doan.len();
    if cac_doan[i].loai != LoaiDoan::Chu {
        return None;
    }
    if i + 1 < n && cac_doan[i + 1].loai == LoaiDoan::KyThuat && raws[i + 1] == "://" {
        let mut j = i + 2;
        while j < n && cac_doan[j].loai != LoaiDoan::KhoangTrang {
            j += 1;
        }
        return Some(j - i);
    }
    None
}

/// Nhận diện email: (Chu/So/DauCau`.`/KyThuat`+`)+ + KyThuat(`@`) + (Chu/So/`.`)+.
fn nhan_email(raws: &[String], cac_doan: &[Doan], i: usize) -> Option<usize> {
    let n = cac_doan.len();
    if !matches!(cac_doan[i].loai, LoaiDoan::Chu | LoaiDoan::So) {
        return None;
    }
    // Tìm '@' phía sau, bỏ qua local part (Chu/So/dot/plus).
    let mut j = i + 1;
    let mut co_a_cong = false;
    while j < n {
        match cac_doan[j].loai {
            LoaiDoan::Chu | LoaiDoan::So => j += 1,
            LoaiDoan::DauCau if raws[j] == "." || raws[j] == "+" || raws[j] == "-" => j += 1,
            LoaiDoan::KyThuat if raws[j] == "@" => {
                co_a_cong = true;
                j += 1;
                break;
            }
            _ => break,
        }
    }
    if !co_a_cong {
        return None;
    }
    // Domain: Chu/So/dots, ít nhất một Chu/So.
    let mut co_domain = false;
    while j < n {
        match cac_doan[j].loai {
            LoaiDoan::Chu | LoaiDoan::So => {
                co_domain = true;
                j += 1;
            }
            LoaiDoan::DauCau if raws[j] == "." => j += 1,
            _ => break,
        }
    }
    if co_domain { Some(j - i) } else { None }
}

/// Nhận diện đường dẫn tuyệt đối/relative: bắt đầu bằng `/`, `~/`, `./`,
/// `../`, hoặc drive letter `X:\`.
fn nhan_duong_dan(raws: &[String], cac_doan: &[Doan], i: usize) -> Option<usize> {
    let n = cac_doan.len();
    let bat_dau = |loai: LoaiDoan, raw: &str, prefix: &str| {
        loai == LoaiDoan::KyThuat && raw.starts_with(prefix)
    };
    // Unix path `/...`
    if bat_dau(cac_doan[i].loai, &raws[i], "/") {
        return Some(ket_thuc_duong_dan(cac_doan, i, n));
    }
    // `~/`, `./`, `../`
    if bat_dau(cac_doan[i].loai, &raws[i], "~/")
        || bat_dau(cac_doan[i].loai, &raws[i], "./")
        || bat_dau(cac_doan[i].loai, &raws[i], "../")
    {
        return Some(ket_thuc_duong_dan(cac_doan, i, n));
    }
    // Windows drive `X:\` — Chu(1) + KyThuat(":\" )
    if cac_doan[i].loai == LoaiDoan::Chu
        && raws[i].chars().count() == 1
        && i + 1 < n
        && cac_doan[i + 1].loai == LoaiDoan::KyThuat
        && raws[i + 1].starts_with(":\\")
    {
        return Some(ket_thuc_duong_dan(cac_doan, i, n));
    }
    None
}

/// Tiêu thụ các segment cho đến KhoangTrang (phần đường dẫn còn lại).
fn ket_thuc_duong_dan(cac_doan: &[Doan], i: usize, n: usize) -> usize {
    let mut j = i + 1;
    while j < n && cac_doan[j].loai != LoaiDoan::KhoangTrang {
        j += 1;
    }
    j - i
}

/// Nhận diện code span (`` `...` ``) hoặc code fence (``` ```...``` ```).
/// Trả số segment tiêu thụ nếu khớp, bao gồm backtick đóng.
fn nhan_code(raws: &[String], cac_doan: &[Doan], i: usize) -> Option<usize> {
    let n = cac_doan.len();
    if cac_doan[i].loai != LoaiDoan::KyThuat {
        return None;
    }
    let so_backtick = raws[i].chars().filter(|&c| c == '`').count();
    if so_backtick == 0 {
        return None;
    }
    // Tìm backtick đóng cùng số lượng (span = 1, fence = 3).
    let mut j = i + 1;
    while j < n {
        if cac_doan[j].loai == LoaiDoan::KyThuat {
            let bj = raws[j].chars().filter(|&c| c == '`').count();
            if bj == so_backtick {
                return Some(j - i + 1);
            }
        }
        j += 1;
    }
    // Chưa đóng: chỉ giữ backtick mở raw, không khóa content.
    Some(1)
}

/// Nhận diện emoticon bắt đầu tại `i`. Trả số segment tiêu thụ.
/// Mẫu: `=)`+`)`*, `:)`, `:(`, `:D`, `:P`, `:v`, `:3`, `;)`, `^^`, `-_`,
/// `???`, `!!!`, `?!` lặp, `...` lặp. Các ký tự không phải chữ → đã raw sẵn;
/// recognizer chủ yếu phụ trợ trace và khóa các chữ trong `:v`/`:D`/`:P`.
fn nhan_emoticon(raws: &[String], cac_doan: &[Doan], i: usize) -> Option<usize> {
    let n = cac_doan.len();
    if i >= n {
        return None;
    }
    let loai = cac_doan[i].loai;
    let raw = &raws[i];
    // Bắt đầu bằng `=` hoặc `:` hoặc `;` theo sau `)` / `(` / `D` / `P` /
    // `v` / `3` / `-` / `^`, hoặc chuỗi `?`/`!`/`.` lặp.
    let bat_dau_emoticon =
        matches!(raw.chars().next(), Some('=' | ':' | ';')) && !raw.is_empty() && {
            // Ký tự thứ hai (nếu có) hoặc segment kế tiếp là mặt emoticon.
            raw.chars()
                .nth(1)
                .is_some_and(|c| matches!(c, ')' | '(' | 'D' | 'P' | 'v' | '3' | '-' | '^'))
                || (i + 1 < n
                    && matches!(cac_doan[i + 1].loai, LoaiDoan::DauCau | LoaiDoan::Chu)
                    && matches!(
                        raws[i + 1].chars().next(),
                        Some(')' | '(' | 'D' | 'P' | 'v' | '3' | '-' | '^')
                    ))
        };
    if bat_dau_emoticon {
        // Tiêu thụ các segment mặt emoticon kế tiếp (DauCau/Chu mặt) cho đến
        // khi gặp segment không phải mặt.
        let mut j = i + 1;
        while j < n {
            let r = &raws[j];
            let la_mat = matches!(
                cac_doan[j].loai,
                LoaiDoan::DauCau | LoaiDoan::KyThuat | LoaiDoan::Chu
            ) && r.chars().all(|c| {
                matches!(
                    c,
                    ')' | '(' | 'D' | 'P' | 'v' | '3' | '-' | '^' | '?' | '!' | '.'
                )
            });
            if !la_mat {
                break;
            }
            j += 1;
        }
        return Some(j - i);
    }
    // Chuỗi `?`/`!`/`.` lặp (≥3 ký tự) — emoticon cảm xúc.
    if matches!(loai, LoaiDoan::DauCau)
        && raw.len() >= 3
        && raw.chars().all(|c| matches!(c, '?' | '!' | '.'))
    {
        return Some(1);
    }
    None
}

/// Tính `bat_buoc_raw` và bằng chứng cho mỗi đoạn.
pub(crate) fn nhan_dien(cac_doan: &[Doan], thao_tac: &[ThaoTacNhap]) -> Vec<KetQuaNhanDien> {
    let n = cac_doan.len();
    let raws: Vec<String> = cac_doan.iter().map(|d| raw_doan(thao_tac, d)).collect();
    let mut ket_qua = vec![
        KetQuaNhanDien {
            bat_buoc_raw: false,
            bang_chung: BangChungLuaChon::PhanCachIdentifier,
        };
        n
    ];

    // Pass 1: nhận diện cấu trúc span (URL, email, đường dẫn, code, emoticon).
    let mut i = 0;
    while i < n {
        let span = nhan_url(&raws, cac_doan, i)
            .map(|x| (x, BangChungLuaChon::CauTrucUrl))
            .or_else(|| nhan_email(&raws, cac_doan, i).map(|x| (x, BangChungLuaChon::CauTrucEmail)))
            .or_else(|| {
                nhan_duong_dan(&raws, cac_doan, i).map(|x| (x, BangChungLuaChon::CauTrucDuongDan))
            })
            .or_else(|| {
                nhan_code(&raws, cac_doan, i).map(|x| (x, BangChungLuaChon::CauTrucCommand))
            })
            .or_else(|| nhan_emoticon(&raws, cac_doan, i).map(|x| (x, BangChungLuaChon::Emoticon)));
        if let Some((len, chung)) = span {
            for kd in &mut ket_qua[i..i + len] {
                kd.bat_buoc_raw = true;
                kd.bang_chung = chung;
            }
            i += len;
        } else {
            i += 1;
        }
    }

    // Pass 2: per-segment `::` precedent và `=` adjacent cho đoạn Chu còn lại.
    for i in 0..n {
        if !matches!(cac_doan[i].loai, LoaiDoan::Chu) || ket_qua[i].bat_buoc_raw {
            continue;
        }
        // `::` precedent → namespace → raw.
        if let Some(p) = tim_ky_thuat_truoc(cac_doan, i) {
            if raws[p] == "::" {
                ket_qua[i].bat_buoc_raw = true;
                ket_qua[i].bang_chung = BangChungLuaChon::CauTrucDuongDan;
                continue;
            }
            // `=`-containing precedent (vd `x = buf`).
            if chua(&raws[p], '=') {
                ket_qua[i].bat_buoc_raw = true;
                ket_qua[i].bang_chung = BangChungLuaChon::CauTrucCommand;
                continue;
            }
        }
        // `=`-containing sau (vd `buf = x`).
        if let Some(s) = tim_ky_thuat_sau(cac_doan, i) {
            if chua(&raws[s], '=') {
                ket_qua[i].bat_buoc_raw = true;
                ket_qua[i].bang_chung = BangChungLuaChon::CauTrucCommand;
                continue;
            }
            // `::` sau (vd `foo::bar` — `foo` trước `::` cũng raw).
            if raws[s] == "::" {
                ket_qua[i].bat_buoc_raw = true;
                ket_qua[i].bang_chung = BangChungLuaChon::CauTrucDuongDan;
            }
        }
    }

    ket_qua
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cau_hinh::KieuTelex;
    use crate::phan_doan::phan_doan;
    use crate::thao_tac::ThaoTacNhap;

    fn raw(s: &str) -> Vec<ThaoTacNhap> {
        s.chars().map(ThaoTacNhap::tu_dong).collect()
    }

    fn nd(thao_tac: &[ThaoTacNhap]) -> Vec<KetQuaNhanDien> {
        let d = phan_doan(thao_tac, KieuTelex::CanBang);
        nhan_dien(&d, thao_tac)
    }

    fn bat_buoc(kq: &[KetQuaNhanDien]) -> Vec<bool> {
        kq.iter().map(|x| x.bat_buoc_raw).collect()
    }

    #[test]
    fn url_phai_raw() {
        let kq = nd(&raw("https://example.com/as"));
        assert!(bat_buoc(&kq).iter().all(|x| *x), "toàn bộ URL phải raw");
    }

    #[test]
    fn email_phai_raw() {
        let kq = nd(&raw("test@as.com"));
        assert!(bat_buoc(&kq).iter().all(|x| *x));
    }

    #[test]
    fn duong_dan_unix_phai_raw() {
        let kq = nd(&raw("/home/as"));
        assert!(bat_buoc(&kq).iter().all(|x| *x));
    }

    #[test]
    fn duong_dan_tilde_phai_raw() {
        let kq = nd(&raw("~/Documents/as"));
        assert!(bat_buoc(&kq).iter().all(|x| *x));
    }

    #[test]
    fn duong_dan_windows_phai_raw() {
        let kq = nd(&raw(r"C:\Users\as"));
        assert!(bat_buoc(&kq).iter().all(|x| *x));
    }

    #[test]
    fn namespace_2_ngoac_raw_chu_sau() {
        // "foo::bar" → "bar" (Chu cuối, index 2) phải raw.
        let kq = nd(&raw("foo::bar"));
        assert!(bat_buoc(&kq)[2]);
    }

    #[test]
    fn gan_bang_raw_chu_truoc() {
        // "buf = x" → "buf" (đoạn 0) phải raw.
        let kq = nd(&raw("buf = x"));
        assert!(bat_buoc(&kq)[0]);
    }

    #[test]
    fn code_span_raw() {
        let kq = nd(&raw("`as`"));
        assert!(bat_buoc(&kq).iter().all(|x| *x));
    }

    #[test]
    fn code_fence_raw() {
        let kq = nd(&raw("```as```"));
        assert!(bat_buoc(&kq).iter().all(|x| *x));
    }

    #[test]
    fn code_fence_khong_dong_khong_khoa_content() {
        // "```as" chưa đóng → chỉ backtick mở raw, "as" không khóa.
        let kq = nd(&raw("```as"));
        assert!(bat_buoc(&kq)[0]);
        assert!(!bat_buoc(&kq)[1]);
    }

    #[test]
    fn chu_thuong_khong_bat_buoc_raw() {
        let kq = nd(&raw("tieengs"));
        assert!(!bat_buoc(&kq)[0]);
    }

    #[test]
    fn emoticon_raw() {
        let kq = nd(&raw(":v"));
        assert!(bat_buoc(&kq).iter().all(|x| *x));
    }
}
