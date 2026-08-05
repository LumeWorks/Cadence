# Phase 4 Report - Ổn định và kiểm tra cho phát hành 0.1.0

Ngày hoàn thành: 2026-08-06

## Tổng quan

Phase 4 ổn định API, thêm tài liệu bảo mật/bất biến/MSRV, rule matrix tests,
editing/Unicode matrix, property/serde tests, soak tests, sửa một bug
cursor, và chuẩn bị phát hành `0.1.0`.

## Commit baseline

- Baseline Phase 3: **73 commits**, **435 tests** (audit thực tế, không phải 72)
- Phase 4 thêm: **16+ commits**, **650 tests** (34 test files)
- Tổng: **89+ commits**, **650 tests**

## Tài liệu Phase 4 (11 file mới)

| File | Mục đích |
|---|---|
| `docs/PHASE3_BASELINE.md` | Audit baseline: commit count, test count, gate results |
| `docs/INVARIANTS.md` | 11 bất biến nền tảng, liên kết test chứng minh |
| `docs/api/public-api-0.1.0.md` | Public API inventory, phân loại ổn định |
| `docs/API_STABILITY.md` | Chính sách compat 0.1.x, breaking change rules |
| `docs/SECURITY_MODEL.md` | Threat model, mitigation, panic audit |
| `docs/MSRV.md` | Rust 1.85 policy |
| `docs/TRACE_PRIVACY.md` | Trace opt-in, privacy commitments |
| `docs/DEPENDENCIES.md` | Dependency audit, cargo-deny config |
| `deny.toml` | cargo-deny: license allowlist, ban advisories |
| `tests/contract.rs` | Compile-time Send/Sync/Clone/Static contracts |
| `tests/soak.rs` | Soak tests chịu tải dài |

## Test matrix Phase 4

### Corpus Phase 4 (120 tests, 14 module)

`tieng_viet`, `hinh_chu`, `dau_thanh`, `escape`, `am_tiet`, `code`,
`command`, `url_email_path`, `teencode`, `emoticon`, `unicode`,
`context_mix`, `editing`, `adversarial`.

### Rule matrix unit tests (36 tests)

- `src/am_tiet.rs`: AM_DAU/AM_CUOI shadow check, entry match, longest-prefix
  priority, `phan_tich_am_tiet` edge cases, `la_nguyen_am` consistency.
- `src/chu_viet.rs`: `chu_goc_tu_ky_tu` mapping, `la_nguyen_am`,
  `ky_tu_thuong`, `KieuHoa` `tu_ky_tu`/`ap_dung` (đ→Đ),
  `ChuCaiViet::thuong`, `DauChu`/`DauThanh` pairwise distinct.
- `src/render.rs`: `nguyen_am_nfc` base/invalid/none, `phan_tich_ky_tu`
  round-trip (65 chars) + case + non-Vietnamese, `tu_dau_thanh` mapping,
  NFD decomposition.
- `src/telex.rs`: `cap_hinh_chu` valid/invalid/case-insensitive,
  `tu_dau_thanh_key` mapping, `la_phim_dau_thanh` consistency,
  DayDu/CanBang `w` behavior, raw char literal verification.

### Editing matrix (17 tests)

- 7 matrix tests: `them`/`xoa_lui`/`xoa_phia_truoc`/`di_trai`/`di_phai`/
  `ve_dau`/`ve_cuoi` tại mỗi vị trí (rỗng, đầu, giữa, cuối).
- 4 backspace+retype tests: tone, shape, shape+tone, hai bước.
- 2 delete-forward trên Telex tests.
- 4 NFD cursor movement tests: vào grapheme phân rã, qua nhiều grapheme,
  backspace trên grapheme phân rã, NFC/NFD equivalence matrix (35 combos).

### Property tests (8 new)

Navigation không đổi nội dung, cursor round-trip, boundary navigation
`KhongDoi`, `loai_noi_dung` ổn định, `chap_nhan` trả đúng, hai phiên cùng
`loai`, `xoa_lui`/`xoa_phia_truoc` ở boundary `KhongDoi`.

### Serde tests (9 tests)

4 type derive tests + 5 round-trip tests (serde_json serialize →
deserialize → equals) cho `KetQuaXuLy`, `LoaiNoiDung`, `KieuTelex`,
`QuyTacDatDau`, `DangUnicode`, `ChinhSachLuaChon`.

### Regression tests (3 new)

Bug `di_phai_raw` ở cuối lịch sử khi raw cuối là tone key (không navigable).

### Soak tests (10 tests)

1000 ký tự Telex liên tục, xen kẻ `them_ky_tu`/`them_nguyen_ban`,
navigation 200 bước, chèn/xóa lặp 200 vòng, mọi tổ hợp cấu hình (24),
commit/reset lặp 100, emoji+combining+Telex trộn, giới hạn thấp (10),
xóa đến rỗng.

## Bug fix

**`di_phai_raw` ở cuối lịch sử**: khi raw cuối là tone key (không
navigable), `di_phai` trả `CapNhat` sai thay vì `KhongDoi`. Nguyên nhân:
`snap_raw` snap về navigable gần nhất, trả snapped value ≠ r, khiến
caller thấy `moi != r`. Fix: trả `r` gốc (không snap) khi ở hoặc vượt
navigable cuối (`src/anh_xa.rs:365-377`).

## Known limitations (0.1.0)

1. **Ký tự Việt HOA dựng sẵn** (vd `Ế`, `Đ`) không được `phan_tich_ky_tu`
   nhận diện vì `to_ascii_lowercase` không đổi non-ASCII. Khi gõ trực tiếp,
   ký tự đó được giữ raw (an toàn), không parse. Phụ âm hoa `Đ` tương tự.
   Workaround: dùng Telex để tạo chữ hoa (vd `Dd` → `Đ`).

2. **`cargo-deny`/`cargo-audit`** không cài trong môi trường hiện tại;
   config `deny.toml` sẵn sàng, không claim gate xanh cho hai công cụ này.

## Gate results

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | clean |
| `cargo clippy --all-features --all-targets -- -D clippy::all` | clean |
| `cargo clippy --no-default-features --all-targets -- -D clippy::all` | clean |
| `cargo test --all-features` | 650 tests pass |
| `cargo test --no-default-features` | pass |
| `cargo test --features serde` | pass |
| `cargo test --features trace` | pass |
| `cargo check --release --no-default-features --features serde,trace` | pass |
| `cargo doc --all-features --no-deps` | clean, no warnings |
| `cargo bench --all-features --no-run` | compiles |
| Examples (all, no-default, serde, trace) | all run |

## Examples Phase 4

- `examples/co_ban.rs`: cơ bản (Phase 1).
- `examples/go_moi_thu.rs`: Phase 3 trộn code/URL/tiếng Việt/teencode.
- `examples/truy_vet.rs`: trace quyết định (feature `trace`).
- `examples/xuat_nhap.rs`: serde round-trip (feature `serde`).

## Tóm tắt bất biến Phase 4

- `di_phai` ở cuối luôn `KhongDoi`; `di_trai` ở đầu luôn `KhongDoi`.
- `xoa_lui` ở đầu luôn `KhongDoi`; `xoa_phia_truoc` ở cuối luôn `KhongDoi`.
- Navigation không thay đổi nội dung (raw và rendered).
- Mọi tổ hợp cấu hình (2×2×2×3=24) engine ổn định.
- NFC/NFD canonical equivalent cho mọi shape × tone.
- Serde round-trip cho mọi public data type.
- 1000 ký tự Telex liên tục: không panic, cursor hợp lệ.
