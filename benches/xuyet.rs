// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Benchmark nền cho các thao tác phổ biến của phiên.
//!
//! Không phải acceptance gate cứng theo thời gian trên CI vì máy chạy
//! khác nhau. Mục tiêu: theo dõi throughput, thời gian mỗi thao tác và
//! so sánh token ngắn với token tối đa.

// Bench là target benchmark, không phải public API; macro criterion_group
// sinh hàm main không có rustdoc nên tắt missing_docs cục bộ ở đây.
#![allow(missing_docs)]

use cadence::{BoGo, CauHinh, PhienGo};
use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};

fn tao_phien(gioi_han: usize) -> PhienGo {
    let mut cau_hinh = CauHinh::mac_dinh();
    cau_hinh
        .dat_gioi_han_thao_tac(gioi_han)
        .expect("gioi han hop le");
    let bo_go = BoGo::new(cau_hinh).expect("cau hinh hop le");
    bo_go.tao_phien()
}

/// Phiên đã có sẵn `so_op` ký tự ASCII, con trỏ ở cuối.
fn phien_co_san(so_op: usize) -> PhienGo {
    let mut phien = tao_phien(4096);
    for _ in 0..so_op {
        phien.them_ky_tu('a');
    }
    phien
}

/// Thêm ký tự ASCII vào token ngắn (8 thao tác).
fn them_ascii_token_ngan(c: &mut Criterion) {
    c.bench_function("them_ascii_token_ngan", |b| {
        b.iter_batched(
            || phien_co_san(8),
            |mut phien| {
                phien.them_ky_tu('a');
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Thêm ký tự Unicode (emoji ngoài BMP) vào token ngắn.
fn them_unicode(c: &mut Criterion) {
    c.bench_function("them_unicode", |b| {
        b.iter_batched(
            || phien_co_san(8),
            |mut phien| {
                phien.them_ky_tu('\u{1F600}');
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Chèn ký tự ở giữa token (con trỏ giữa).
fn chen_o_giua(c: &mut Criterion) {
    c.bench_function("chen_o_giua", |b| {
        b.iter_batched(
            || {
                let mut phien = phien_co_san(16);
                // Đưa con trỏ về giữa.
                phien.ve_dau();
                phien.di_phai();
                phien
            },
            |mut phien| {
                phien.them_ky_tu('x');
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Xóa lùi ở cuối token.
fn xoa_lui(c: &mut Criterion) {
    c.bench_function("xoa_lui", |b| {
        b.iter_batched(
            || phien_co_san(16),
            |mut phien| {
                phien.xoa_lui();
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Replay token 16 thao tác: cặp thêm/xóa lùi giữ độ dài ~16, mỗi cặp
/// thực hiện hai lần replay đầy đủ.
fn replay_token_16(c: &mut Criterion) {
    c.bench_function("replay_token_16", |b| {
        b.iter_batched(
            || phien_co_san(16),
            |mut phien| {
                // Hai replay: thêm (17) rồi xóa lùi (16).
                phien.them_ky_tu('a');
                phien.xoa_lui();
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Replay token 128 thao tác.
fn replay_token_128(c: &mut Criterion) {
    c.bench_function("replay_token_128", |b| {
        b.iter_batched(
            || phien_co_san(128),
            |mut phien| {
                phien.them_ky_tu('a');
                phien.xoa_lui();
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Telex shape transform: gõ `dduwowngf` → `đường`.
fn telex_shape_transform(c: &mut Criterion) {
    c.bench_function("telex_shape_transform", |b| {
        b.iter_batched(
            || tao_phien(4096),
            |mut phien| {
                for c in "dduwowngf".chars() {
                    phien.them_ky_tu(c);
                }
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Telex tone mark: gõ `tieengs` → `tiếng`.
fn telex_tone_mark(c: &mut Criterion) {
    c.bench_function("telex_tone_mark", |b| {
        b.iter_batched(
            || tao_phien(4096),
            |mut phien| {
                for c in "tieengs".chars() {
                    phien.them_ky_tu(c);
                }
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Telex escape: gõ `aww` → `aw`.
fn telex_escape(c: &mut Criterion) {
    c.bench_function("telex_escape", |b| {
        b.iter_batched(
            || tao_phien(4096),
            |mut phien| {
                for c in "aww".chars() {
                    phien.them_ky_tu(c);
                }
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Telex âm tiết dài 20 ký tự: build pipeline đầy đủ.
fn telex_am_tiet_dai(c: &mut Criterion) {
    c.bench_function("telex_am_tiet_dai", |b| {
        b.iter_batched(
            || tao_phien(4096),
            |mut phien| {
                for c in "nguyenthithuydung".chars() {
                    phien.them_ky_tu(c);
                }
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Telex `nguowif` → `người`: triphthong + tone placement.
fn telex_nguoi(c: &mut Criterion) {
    c.bench_function("telex_nguoi", |b| {
        b.iter_batched(
            || tao_phien(4096),
            |mut phien| {
                for c in "nguowif".chars() {
                    phien.them_ky_tu(c);
                }
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Phase 3: gõ code trộn tiếng Việt — "cargo build lỗi rồi =))".
/// Đo phân đoạn + nhận diện ngữ cảnh + render per-segment.
fn phase3_code_tron(c: &mut Criterion) {
    c.bench_function("phase3_code_tron", |b| {
        b.iter_batched(
            || tao_phien(4096),
            |mut phien| {
                for c in "cargo build lỗi rồi =))".chars() {
                    phien.them_ky_tu(c);
                }
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Phase 3: gõ URL dài — "https://example.com/path?query=1".
/// Đo nhận diện URL + toàn bộ raw path.
fn phase3_url(c: &mut Criterion) {
    c.bench_function("phase3_url", |b| {
        b.iter_batched(
            || tao_phien(4096),
            |mut phien| {
                for c in "https://example.com/path?query=1".chars() {
                    phien.them_ky_tu(c);
                }
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Phase 3: gõ namespace Rust — "foo::bar::baz".
/// Đo nhận diện `::` adjacency trên nhiều đoạn.
fn phase3_namespace(c: &mut Criterion) {
    c.bench_function("phase3_namespace", |b| {
        b.iter_batched(
            || tao_phien(4096),
            |mut phien| {
                for c in "foo::bar::baz".chars() {
                    phien.them_ky_tu(c);
                }
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Phase 3: gõ teencode lặp — "brooooooo".
/// Đo phát hiện teencode-lap trước Telex.
fn phase3_teencode_lap(c: &mut Criterion) {
    c.bench_function("phase3_teencode_lap", |b| {
        b.iter_batched(
            || tao_phien(4096),
            |mut phien| {
                for c in "brooooooo".chars() {
                    phien.them_ky_tu(c);
                }
                black_box(phien.ban_chup().noi_dung());
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    them_ascii_token_ngan,
    them_unicode,
    chen_o_giua,
    xoa_lui,
    replay_token_16,
    replay_token_128,
    telex_shape_transform,
    telex_tone_mark,
    telex_escape,
    telex_am_tiet_dai,
    telex_nguoi,
    phase3_code_tron,
    phase3_url,
    phase3_namespace,
    phase3_teencode_lap,
);
criterion_main!(benches);
