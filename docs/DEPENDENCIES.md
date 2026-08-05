# Dependency và supply chain

Cadence giữ runtime dependency tối thiểu. Mục tiêu Phase 4: **không thêm
runtime dependency mới**. Tài liệu này audit dependency và cấu hình CI gate.

## Runtime dependency

| Crate | Version | License | Mục đích | Default features |
|---|---|---|---|---|
| `unicode-segmentation` | `1` | MIT OR Apache-2.0 | Grapheme cluster boundary | `false` (no_std) |
| `unicode-normalization` | `0.1.25` | MIT OR Apache-2.0 | NFD output + canonical equivalence check | `false` (no_std) |
| `serde` | `1` (optional) | MIT OR Apache-2.0 | Derive Serialize/Deserialize cho data type | `false` + `alloc` + `derive` |

`serde` là optional (`dep:serde`): người dùng bình thường không kéo serde.
Bật feature `serde` mới kéo.

## Dev dependency (không vào runtime)

| Crate | Version | License | Mục đích |
|---|---|---|---|
| `proptest` | `1` | MIT OR Apache-2.0 | Property test bất biến |
| `criterion` | `0.5` | Apache-2.0 OR MIT | Benchmark |
| `unicode-normalization` | `0.1.25` | MIT OR Apache-2.0 | Test canonical equivalence |
| `unicode-segmentation` | `1` | MIT OR Apache-2.0 | Test grapheme |

## CI tool (không phải dependency của Cadence)

| Tool | Mục đích | Cài đặt |
|---|---|---|
| `cargo-deny` | License/advisory/ban/source gate | `cargo install cargo-deny` |
| `cargo-audit` | Security advisory | `cargo install cargo-audit` |

Đây là tool CI, không vào `Cargo.toml`. `deny.toml` config sẵn. Nếu môi
trường không cài tool, ghi lỗi, không claim gate xanh, giữ config.

## Audit license

Tất cả runtime + dev dependency là **MIT OR Apache-2.0**, tương thích MPL-2.0
của Cadence. Không dependency GPL/AGPL/LGPL.

## no_std

`unicode-segmentation` và `unicode-normalization` đều `default-features = false`
để dùng được `no_std + alloc`. `serde` `default-features = false` +
`features = ["alloc", "derive"]`.

## MSRV

Tất cả dependency tương thích MSRV 1.85 (xem `docs/MSRV.md`).

## Transitive dependency

`cargo metadata` liệt kê transitive (aho-corasick, regex (qua proptest?),
criterion kéo nhiều). Runtime transitive của Cadence (không tính dev):
- `unicode-segmentation` → `tinyvec` (no_std).
- `unicode-normalization` → `tinyvec`.
- `serde` (optional) → `serde_derive` (proc-macro, chỉ compile-time).

Không transitive nặng (không `tokio`, `reqwest`, `regex` runtime, v.v.).

## Duplicate version

`cargo-deny` `multiple-versions = "warn"` — criterion có thể kéo duplicate
transitive (dev-only). Runtime không có duplicate nghiêm trọng. Nếu phát hiện
duplicate runtime, ưu tiên giải quyết.

## cargo-deny

`deny.toml` cấu hình:
- **advisories**: `yanked = "deny"`, không ignore advisory nào.
- **licenses**: allow MIT/Apache-2.0/Unicode/Zlib/BSD; exception `cadence-ime`
  allow MPL-2.0.
- **bans**: `multiple-versions = "warn"` (không deny để criterion dev OK).
- **sources**: chỉ crates.io, không git dependency, không path ngoài repo.

Chạy: `cargo deny check` (cần cài `cargo-deny`).

## cargo-audit

`cargo audit` quét `Cargo.lock` theo advisory database. Chạy: `cargo audit`
(cần cài `cargo-audit` + network cho advisory db).

## Trạng thái môi trường audit hiện tại

- `cargo-deny`: **chưa cài** trong môi trường audit này. Config `deny.toml`
  sẵn; CI sẽ chạy khi tool khả dụng. Không claim gate đã xanh.
- `cargo-audit`: **chưa cài**. Sẽ chạy trong CI scheduled (xem `.github/workflows`).

## Chính sách thêm dependency

Không thêm dependency trừ khi:
1. Có use case thật không giải quyết được bằng std/alloc/core.
2. Tương thích MSRV 1.85, no_std, MPL-2.0-compatible license.
3. Có lý do rõ trong `docs/DEPENDENCIES.md`.
4. Không tăng runtime dependency nếu chưa có bằng chứng cần thiết.

Không thêm: regex, async runtime, logging framework, parser framework,
collection crate, error framework, network crate.
