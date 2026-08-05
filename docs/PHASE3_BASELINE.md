# Baseline Phase 3 (audit thực tế)

Tài liệu này ghi kết quả audit baseline được chạy lại trước khi bắt đầu
Phase 4. Mục đích: không tin báo cáo một cách mù quáng, mà xác minh từ source
và gate thực tế.

## Repository

| Mục | Báo cáo Phase 3 | Thực tế | Khác biệt |
|---|---|---|---|
| Branch | `main` | `main` | — |
| Remote | `https://github.com/LumeWorks/Cadence.git` | đúng | — |
| Tên repo | `Cadence` (không còn `Candence`) | đúng | — |
| Package name | `cadence-ime` | đúng | — |
| Library target | `cadence` | đúng | — |
| Version | `0.1.0` | `0.1.0` | đã đặt sẵn |
| MSRV | Rust 1.85 | `rust-version = "1.85"` | — |
| Edition | 2024 | 2024 | — |
| Tổng commit | 72 | **73** | +1 |
| Phase 3 commit range | `76f605e..52b3bb4` | đúng | — |

Khác biệt duy nhất: sau commit Phase 3 cuối `52b3bb4` có thêm commit
`9c623b6` ("thay dau em (—) bang dau gach (-) trong toan bo codebase") — một
commit dọn ký tự en-dash thành hyphen. Phase 4 làm việc theo source thực tế
(73 commit).

## Gate baseline (chạy lại, xanh)

| Lệnh | Kết quả |
|---|---|
| `cargo fmt --check` | xanh |
| `cargo clippy --all-targets --all-features -- -D warnings` | xanh |
| `cargo clippy --all-targets --no-default-features -- -D warnings` | xanh |
| `cargo test --all-features` | 435 test xanh |
| `cargo test --no-default-features` | xanh |
| `cargo test --no-default-features --features serde` | xanh |
| `cargo test --no-default-features --features trace` | xanh |
| `cargo check --release` | xanh |
| `cargo check --release --no-default-features` | xanh |
| `RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps` | xanh |
| `cargo +1.85 test --all-features` | xanh |
| `cargo +1.85 test --no-default-features` | xanh |
| `cargo +1.85 check --release --no-default-features` | xanh |

## Test count all-features (theo file)

```
cau_hinh             13    chinh_sach_lua_chon   8     corpus_phase3        43
gioi_han              6    ngu_canh             22     phan_doan            10
phien_co_ban         15    phien_con_tro        14    property              9
property_phase3       8    regression            7     serde                 2
telex_am_tiet        21    telex_bien_the_dau   25    telex_con_tro         11
telex_config         10    telex_dau_thanh      32    telex_daydu_dau       15
telex_dod            19    telex_escape         15    telex_hinh_chu        19
telex_hoa            15    telex_kieu_telex     12    telex_lua_chon        14
telex_mix            10    telex_nfd             9     telex_nguyen_ban       9
telex_quy_tac_dat_dau 8    telex_round_trip     10    trace                  6
unicode              17    doctest               1
Tổng: 435
```

## Source audit (grep thực tế)

| Pattern trong `src` | Số lần | Ghi chú |
|---|---|---|
| `unsafe` | 1 | chỉ `#![forbid(unsafe_code)]` (không phải usage) |
| `unwrap()` / `unwrap_err()` | 0 | — |
| `panic!` / `unreachable!` / `unimplemented!` / `todo!` | 0 | — |
| `std::fs` / `std::net` / `std::thread` / `Mutex` / `RwLock` | 0 | — |
| `TODO` / `FIXME` / `HACK` / `XXX` trong `src` | 1 | `src/phien_go.rs:172` (khoi_phuc_nguyen_ban) |
| `expect(` trong `src` | 0 | — |
| `expect(` trong `tests/benches/examples` | nhiều | luôn kèm invariant message (cho phép) |
| `panic!` trong `tests` | 4 | test assertion (cho phép) |

## Public API thực tế (trước Phase 4)

```
Struct:     BoGo, CauHinh, PhienGo, BanChupSoan, ViTriVanBan, LoiCauHinh
Enum:       KetQuaXuLy, LoaiNoiDung, KieuTelex, QuyTacDatDau, DangUnicode,
            ChinhSachLuaChon
Trace:      TraceStep, TraceKetQua, BangChungLuaChon  (feature `trace`)
```

## Runtime dependency

```
unicode-segmentation  (default-features = false)
unicode-normalization  (default-features = false)
serde                  (optional, default-features = false, features alloc+derive)
```

## Dev dependency

```
proptest, criterion, unicode-normalization, unicode-segmentation
```

## Vấn đề được ghi nhận để xử lý Phase 4

1. `PhienGo::khoi_phuc_nguyen_ban` là no-op với `TODO(phase-3)`. Phase 3 đã
   xong nhưng chưa dọn TODO. Contract cần làm rõ: raw luôn giữ qua
   `noi_dung_goc()`, method này idempotent và tồn tại cho API completeness.
2. `BoGo::new` luôn trả `Ok` vì `CauHinh` chỉ mang giá trị hợp lệ. Giữ
   `Result` cho forward-compat (config validation tương lai), tài liệu hóa lý do.
3. Chưa có `cargo-deny` / `cargo-audit` config.
4. Chưa có soak, differential, INVARIANTS, SECURITY_MODEL, INTEGRATION, v.v.

## Kết luận

Baseline Phase 3 khớp báo cáo (trừ +1 commit dọn dash). Toàn bộ gate xanh.
Phase 4 làm việc từ commit `9c623b6` (HEAD của `main` tại thời điểm audit).
