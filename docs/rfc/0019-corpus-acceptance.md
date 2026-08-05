# RFC 0019 — Corpus và acceptance tests Phase 3

Trạng thái: Chấp thuận — Phase 3 (chưa triển khai).

## Vấn đề

Phase 3 thay đổi pipeline render (phân đoạn + nhận diện + lựa chọn cục bộ).
Cần regression tests bao phủ các DoD case và property tests đảm bảo bất biến.

## Quyết định

### Corpus tests (`tests/corpus_phase3.rs`)

Test end-to-end qua `PhienGo::them_ky_tu` cho từng DoD category:

```text
identifier:     async, class, struct, String, user_id, HTTPServer
URL/email/path: https://example.com, user@x.com, ~/docs, C:\Users
code:           foo::bar, let mut buf = String::new();
emoticon:       =))), :v, ???, !!!!!!!
teencode:       brooooo, vcl, ko, dc
tiếng Việt:     tieengs→tiếng, nguowif→người, dduwowngf→đường
trộn:           cargo build lỗi rồi =)), user_id của m là gì?
```

### Property tests (`tests/property_phase3.rs`)

Bất biến kiểm chứng qua `proptest`:

1. **Noi_dung_goc nguyên vẹn**: `noi_dung_goc()` == raw input, byte-for-byte.
2. **Round-trip raw**: `them_nguyen_ban` cho mỗi char → `noi_dung()` == raw.
3. **Cấu trúc kỹ thuật raw**: URL/email/path/code fence luôn raw.
4. **Tone không xuyên đoạn**: dấu thanh không lan sang đoạn khác.
5. **Deterministic**: cùng cấu hình + history → cùng snapshot.

### Benchmark (`benches/phase3.rs`)

Criterion microbench:
* `bench_chu_thuan` — "tieengs" (Telex path)
* `bench_code_tron` — "cargo build lỗi rồi =))" (segment + recognize path)
* `bench_url` — "https://example.com/path" (all-raw path)
* So sánh Phase 2 baseline vs Phase 3 không chậm hơn 2x.

## Tác động `no_std`

Corpus/property/benchmark tests là `std`-only (dev-dependencies). Không ảnh
hưởng `no_std` build.

## Tác động public API

Không thay đổi. Tests dùng public API.
