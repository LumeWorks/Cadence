# Báo cáo audit phát hành — Cadence 0.1.0

Ngày audit: 2026-08-06. Audit thực tế (chạy lệnh, không tin báo cáo).

## 1–7. Git state và commit

| Mục | Giá trị |
|---|---|
| 1. Branch | `main` |
| 2. Repository URL | `https://github.com/LumeWorks/Cadence.git` |
| 3. HEAD trước audit | `0187c6d` (đã có tag `v0.1.0`) |
| 4. HEAD sau audit | commit báo cáo này (xem `git log -1`) |
| 5. Tổng commit trước audit | 95 |
| 6. Tổng commit sau audit | 98 |
| 7. Commit mới | `087c665` sua tai lieu phat hanh chinh xac; `bcb0bff` them tai lieu integration va release; commit báo cáo này |

Lưu ý: tag `v0.1.0` **đã tồn tại và đã push** trước audit tại `0187c6d` (remote
`63268b0`). Vì cấm force-push, tag giữ nguyên tại `0187c6d` (xem mục 51–56).

## 8. Working tree cuối

Sạch sau commit (`git status --short` rỗng).

## 9–13. Package metadata

| Mục | Giá trị |
|---|---|
| 9. Package name | `cadence-ime` (`cargo metadata`) |
| 10. Library target name | `cadence` (`[lib] name = "cadence"`) |
| 11. Version | `0.1.0` (`grep -n "^version" Cargo.toml` → `version = "0.1.0"`) |
| 12. MSRV | Rust 1.85 (`rust-version = "1.85"`) |
| 13. Feature list | `std` (default), `no_std + alloc` (tắt std), `serde` (optional), `trace` (optional) |

`use cadence::BoGo;` hoạt động nhờ `[lib] name = "cadence"`. Không đổi package naming.

## 14–15. Public API / breaking changes

14. Public API changes: **không**. Bề mặt công khai khớp `docs/api/public-api-0.1.0.md`
(6 struct + 6 enum ổn định + 3 item trace thử nghiệm, 8 module công khai, 11 re-export).
`BangChungLuaChon` đúng 12 variant. `khoi_phuc_nguyen_ban` no-op idempotent.
15. Breaking changes: **không**.

## 16. Test count theo feature

- `--all-features`: **655** pass (0 fail).
- `--no-default-features`: pass.
- `--features serde` / `trace` / `serde,trace`: pass.

## 17–27. Gate matrix

| # | Gate | Lệnh | Kết quả |
|---|---|---|---|
| 17 | fmt | `cargo fmt --check` | xanh |
| 18 | Clippy all-features | `cargo clippy --all-targets --all-features -- -D warnings` | xanh |
| 19 | Clippy no-default | `cargo clippy --all-targets --no-default-features -- -D warnings` | xanh |
| 20 | test all-features | `cargo test --all-features` | 655 pass |
| 21 | test no-default | `cargo test --no-default-features` | xanh |
| 22 | serde | `cargo test --no-default-features --features serde` | xanh |
| 23 | trace | `cargo test --no-default-features --features trace` | xanh |
| 24 | serde + trace | `cargo test --no-default-features --features serde,trace` | xanh |
| 25 | release checks | `cargo check --release` (+ no-default/serde/trace/both) | 5/5 xanh |
| 26 | rustdoc | `RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps` | xanh |
| 27 | MSRV 1.85 | `cargo +1.85 fmt/clippy/test/check` (all/no-default/serde,trace/release) | xanh |

`cargo +1.85 clippy --all-targets --all-features -- -D warnings` xanh. Rustfmt +
clippy component 1.85 đã cài trong audit.

## 28–33. Source safety

| # | Audit | Kết quả |
|---|---|---|
| 28 | grep `unsafe` trong src/tests/benches/examples | chỉ `#![forbid(unsafe_code)]` (không usage) |
| 29 | grep `unwrap()/unwrap_err()` trong src/tests/benches/examples | 0 |
| 30 | grep `TODO/FIXME/HACK/XXX` trong src/tests/benches/examples/docs | chỉ trong docs (PHASE3_BASELINE/SECURITY_MODEL mô tả dọn TODO, không phải marker thật) |
| 31 | Panic-path | production `src`: 0 `panic!`/`expect(`/`unreachable!`/`todo!`. 2 `panic!` + 1 `expect(` trong inline `#[cfg(test)]` của `render.rs`/`telex.rs` (test assertion, cho phép, không compile vào production) |
| 32 | Core I/O/thread/lock | 0 (`std::fs/std::net/std::thread/Mutex/RwLock/TcpStream/UdpSocket` trong src = 0); mutable static = 0 |

## 33–35. Dependency / supply chain

| # | Tool | Kết quả |
|---|---|---|
| 33 | `cargo deny check` | advisories/bans/licenses/sources **ok**; chỉ warning `license-not-encountered` (license allowlist không dùng, vô hại) |
| 34 | `cargo audit` | **sạch** (0 advisory, 98 crate dependency trong Cargo.lock) |
| 35 | Duplicate | chỉ dev-dep (`proptest`/`criterion` kéo `getrandom` v0.3/v0.4, `syn` v2/v3). Runtime không duplicate. Không git dependency |

`cargo-deny`/`cargo-audit` đã cài (`cargo install --locked`) và chạy trong audit;
không thêm vào runtime/dev dependency của crate.

## 36–38. Package

| # | Bước | Kết quả |
|---|---|---|
| 36 | `cargo package --list` | 117 file, không `target/`/`.git`/soak log/secret/dump |
| 37 | `cargo package` | xanh (117 file, 526.4 KiB, 132.5 KiB nén); verify độc lập compile xanh; không `--allow-dirty` |
| 38 | Package contents | `Cargo.toml` + `Cargo.toml.orig` + `.cargo_vcs_info.json` (sha `0187c6d`) + LICENSE/NOTICE/README/source. Không `target/`, không `.git`. Build độc lập trong `/tmp` xanh |

`tests/property.proptest-regressions` là seed regression proptest có chủ ý commit
(proptest khuyến nghị check-in), không phải fixture tạm.

## 39. Clean-clone verification

Clone từ remote (`git clone --branch main --single-branch`), HEAD `0187c6d`, tree
sạch. Chạy: `cargo test --all-features` (xanh), `cargo test --no-default-features`
(xanh), `cargo check --release --no-default-features` (xanh),
`RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps` (xanh),
`cargo package` (xanh, 117 file). Thư mục tạm đã xóa.

## 40–42. Benchmark

- 40. Lệnh: `cargo bench --all-features --bench xuyet -- --sample-size 10 --warm-up-time 1 --measurement-time 2`
- 41. Môi trường: Intel Core i3-4160 @ 3.60GHz (4 CPU), Linux 6.12.95 (Debian),
  11 GiB RAM, rustc 1.97.1, release mode.
- 42. Kết quả (median):

| Bench | Thời gian |
|---|---|
| them_ascii_token_ngan | 3.3 µs |
| them_unicode | 3.7 µs |
| chen_o_giua | 3.9 µs |
| xoa_lui | 4.5 µs |
| replay_token_16 | 9.5 µs |
| replay_token_128 | 76.7 µs |
| telex_shape_transform (`dduwowngf`) | 22.1 µs |
| telex_tone_mark (`tieengs`) | 17.1 µs |
| telex_escape (`aww`) | 3.6 µs |
| telex_am_tiet_dai | 51.8 µs |
| telex_nguoi (`nguowif`) | 12.9 µs |
| phase3_code_tron | 141 µs |
| phase3_url | 168 µs |
| phase3_namespace | 36.2 µs |
| phase3_teencode_lap | 24.1 µs |

Tất cả trong khoảng µs, trong budget đã tài liệu hóa (µs–ms). Không regression
blocking, không blowup, không treo. Benchmark không có bench riêng trace on/off
(emoticon chỉ trong `phase3_code_tron` `=))`); zero-overhead trace-off đã chứng
minh qua `cfg` gating + `tests/trace.rs`.

## 43–46. Soak

- 43. Seed: 1–10 (mỗi 100k) + `0xC0FFEE` (1M). PRNG xorshift deterministic.
- 44. Số bước: 10×100.000 + 1×1.000.000 = **2.000.000 thao tác** (public API,
  mặc định config limit 128, check invariant mỗi 1000 bước).
- 45. Thời gian: ~2 s mỗi 100k; ~21 s cho 1M. Tổng ~41 s.
- 46. Kết quả: **không panic, không invariant failure**. Cursor luôn ≤ len,
  luôn char boundary. Commit khi đạt giới hạn → reset sạch.

Soak commit trong repo (`cargo test --release --all-features --test soak`): 10
test xanh (0.08 s, tối đa 1000 op/test).

## 47–49. Differential testing

- 47. Differential harness: **không** (known limitation).
- 48. Reference engine thực sự chạy: **không**.
- 49. Differential result: N/A.

Tài liệu không tuyên bố differential đã hoàn tất (INVARIANTS.md ghi "sẽ bổ sung
Phase 4"; PHASE3_BASELINE ghi "chưa có differential"). Correctness dựa vào 655
test + property + soak 2M + rule matrix. Kế hoạch 0.2 (xem CHANGELOG hạn chế).

## 50. Known limitations

1. Ký tự Việt HOA dựng sẵn (`Ế`, `Đ`) không nhận diện khi gõ trực tiếp → giữ raw
   (an toàn). Workaround: Telex (`Dd` → `Đ`).
2. Chỉ Telex (chưa VNI/VIQR/từ điển/autocomplete) — phạm vi 0.1.
3. Chưa có differential harness — kế hoạch 0.2.
4. Không FFI/adapter nền tảng — LCand/WCand riêng.
5. `cargo-deny`/`cargo-audit` là CI tool (không runtime dep), đã chạy xanh audit.

## 51. Release blockers còn lại

Không còn blocker code/security/correctness/package. Tại tag `0187c6d` toàn bộ
gate CODE xanh. Audit phát hiện **thiếu tài liệu phát hành** tại tag (thiếu
`docs/INTEGRATION.md`, `docs/RELEASE.md`, `docs/RELEASE_CANDIDATE_REPORT.md`,
CHANGELOG thiếu mục hạn chế đã biết, sai số test 650→655, bảng panic audit
SECURITY_MODEL chưa phân biệt production/test). Đã sửa trong commit `087c665`,
`bcb0bff` và commit báo cáo này trên `main` ngay sau tag. Vì cấm force-push, tag
`v0.1.0` giữ nguyên tại `0187c6d`.

## 52. Release decision: **GO (code) — tài liệu hoàn tất trên main sau tag**

Toàn bộ gate code (fmt, clippy, test, doc, release, MSRV, package, deny, audit,
soak, benchmark, public API) xanh tại `0187c6d`. Tài liệu phát hành đã hoàn tất
trên `main` ngay sau tag.

## 53–56. Tag

| # | Mục | Trạng thái |
|---|---|---|
| 53 | Tag local | `v0.1.0` tồn tại tại `0187c6d` (annotated) |
| 54 | Tag remote | tồn tại `63268b0… refs/tags/v0.1.0` |
| 55 | Commit push | `main` đã push (commit `087c665`, `bcb0bff`, báo cáo sẽ push) |
| 56 | Tag push | đã push trước audit; không tạo lại (cấm force-push) |

Lưu ý nhỏ: message tag ghi "93 commits, 655 tests" nhưng thực tế 95 commit tại
tag (sau audit 98). Đây là sai số cosmetic trong annotation tag, không sửa được
không force-push, không ảnh hưởng package/code.

## 57. Không chạy `cargo publish`

Xác nhận: **không** chạy `cargo publish` trong audit. Package đã sẵn sàng
(`cargo package` xanh) nhưng publish là thao tác thủ công có chủ đích, nằm
ngoài vòng audit này.
