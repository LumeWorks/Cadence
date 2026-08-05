// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Test trace API (feature `trace`).

#![cfg(feature = "trace")]

use cadence::{BangChungLuaChon, BoGo, CauHinh, TraceKetQua};

fn trace(raw: &str) -> Vec<cadence::TraceStep> {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let mut phien = bo_go.tao_phien();
    for c in raw.chars() {
        phien.them_ky_tu(c);
    }
    phien.trace().to_vec()
}

/// `tieengs` → `tiếng`: một đoạn Telex, ket_qua Telex.
#[test]
fn trace_telex_bien_doi() {
    let t = trace("tieengs");
    assert_eq!(t.len(), 1);
    assert_eq!(t[0].ket_qua, TraceKetQua::Telex);
    assert_eq!(t[0].chuoi_raw, "tieengs");
    assert_eq!(t[0].chuoi_ra, "tiếng");
}

/// `cargo build` → 3 đoạn raw, tất cả NguyenBan.
#[test]
fn trace_cargo_build_raw() {
    let t = trace("cargo build");
    assert_eq!(t.len(), 3);
    assert!(t.iter().all(|s| s.ket_qua == TraceKetQua::NguyenBan));
    assert_eq!(t[0].chuoi_raw, "cargo");
    assert_eq!(t[1].chuoi_raw, " ");
    assert_eq!(t[2].chuoi_raw, "build");
}

/// `https://example.com` → raw, bang_chung CauTrucUrl.
#[test]
fn trace_url_bang_chung() {
    let t = trace("https://example.com");
    assert!(t.iter().all(|s| s.ket_qua == TraceKetQua::NguyenBan));
    assert!(
        t.iter()
            .any(|s| s.bang_chung == BangChungLuaChon::CauTrucUrl)
    );
}

/// `foo::bar` → cả 3 đoạn raw, bang_chung CauTrucDuongDan.
#[test]
fn trace_namespace_bang_chung() {
    let t = trace("foo::bar");
    assert!(t.iter().all(|s| s.ket_qua == TraceKetQua::NguyenBan));
    assert!(
        t.iter()
            .any(|s| s.bang_chung == BangChungLuaChon::CauTrucDuongDan)
    );
}

/// Trace rỗng khi phiên rỗng.
#[test]
fn trace_rong() {
    let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh hop le");
    let phien = bo_go.tao_phien();
    assert!(phien.trace().is_empty());
}

/// Trace deterministic: cùng input → cùng trace.
#[test]
fn trace_deterministic() {
    let t1 = trace("tieengs cargo");
    let t2 = trace("tieengs cargo");
    assert_eq!(t1.len(), t2.len());
    for (a, b) in t1.iter().zip(t2.iter()) {
        assert_eq!(a.bang_chung, b.bang_chung);
        assert_eq!(a.ket_qua, b.ket_qua);
        assert_eq!(a.chuoi_raw, b.chuoi_raw);
        assert_eq!(a.chuoi_ra, b.chuoi_ra);
    }
}
