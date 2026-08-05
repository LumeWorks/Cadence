# Mô hình bảo mật Cadence

Tài liệu này mô tả threat model, mitigation, và panic audit của Cadence.
Cadence là lõi xử lý nhập liệu thuần Rust, nhận input không tin cậy từ host
hoặc fuzz harness.

## Phạm vi

Cadence **không** xử lý dữ liệu tin cậy mặc định:
- Không network, không filesystem, không IPC, không D-Bus.
- Không thread nền, không async runtime, không lock (`Mutex`/`RwLock`).
- Không I/O, không log raw input, không telemetry.
- Toàn bộ state nằm trong `PhienGo` do caller sở hữu; không global state.

`unsafe_code` bị `forbid` toàn crate. Không FFI trong repo này.

## Threat model

| Mối đe dọa | Mức | Mitigation |
|---|---|---|
| DoS qua token dài | cao | `gioi_han_thao_tac` (mặc định 128, tối đa 4096); vượt → `KhongDoi`, giữ state |
| CPU blowup | cao | Replay O(n) mỗi thao tác; không recursion theo input; không regex; phân đoạn tuyến tính |
| Allocation blowup | cao | Không allocation theo kích thước bên ngoài token; `Vec` grows theo thao tác, capped bởi giới hạn |
| Integer overflow | trung bình | Index/con trỏ dùng `usize`; `usize::min` cho boundary; không số học tràn trong hot path (audit) |
| Cursor out of range | trung bình | `con_tro` trong `0..=lich_su.len()`; `byte_tai` dùng `min`; snap về navigable |
| Unicode edge case | trung bình | `unicode-segmentation` chuẩn; `is_char_boundary` debug_assert + tính từ tiền tố |
| Combining-mark storm | trung bình | Render nguyên bản, không re-process; grapheme count đúng |
| Delimiter state không đóng | thấp | Code fence chưa đóng chỉ khóa backtick mở, không treo |
| Serialization input độc | trung bình | `CauHinh` không derive `Deserialize`; các enum serde đơn giản, không internal state |
| Trace vô tình giữ raw | trung bình | Trace opt-in; chỉ chứa token hiện tại; không pointer/timing; tắt → zero overhead |
| Panic vượt FFI (tương lai) | thấp (tương lai) | Core cố không panic với input hợp lệ; FFI repo riêng phải catch panic |
| Dependency advisory | trung bình | `cargo-deny` + `cargo-audit` (xem `docs/DEPENDENCIES.md`) |

## Mitigation chi tiết

### Giới hạn thao tác

`CauHinh::gioi_han_thao_tac` (mặc định 128, phạm vi `1..=4096`). Khi phiên
đạt giới hạn, `them_ky_tu`/`them_nguyen_ban` trả `KetQuaXuLy::KhongDoi` và
giữ nguyên state. Không tự xóa đầu buffer (sẽ mất dữ liệu âm thầm). Host có
thể commit rồi mở phiên mới.

### Replay có upper bound

Mỗi thao tác rebuild snapshot từ lịch sử. Replay là O(n) với n = số thao tác
(≤ 4096). Không có loop lồng phụ thuộc input ngoài phân đoạn (O(n)) và Telex
trên mỗi đoạn (tổng O(n)). Không recursion. Worst case 128 modifier vẫn trong
giới hạn µs–ms (xem benchmark Phase 4).

### Không I/O, thread, lock

`grep -RIn "std::fs|std::net|std::thread|Mutex|RwLock" src` → 0. Core không
chạm OS. Thread/lock là trách nhiệm host.

### Checked conversions

Index cursor dùng `usize::min(raw_to_byte.len() - 1)` (`byte_tai`), không
index trực tiếp không kiểm tra. `is_char_boundary` debug_assert trong
`ViTriVanBank::tai_byte` (luôn đúng khi con trỏ nội bộ nằm giữa hai `char`).

### Match đầy đủ

Toàn bộ `match` trên enum có arm `_ => return None` hoặc exhaustive. Không
`unreachable!`/`unimplemented!`/`todo!` trong `src`.

### Private fields

Tất cả struct public có field private. Truy cập qua method. Không暴露 mutable
reference tới state nội bộ.

### Trace opt-in

Feature `trace` ẩn toàn bộ code trace qua `cfg`. Khi tắt: không allocation
trace, không format `String` trace, output không đổi. Trace chỉ chứa token
hiển thị hiện tại (chuỗi raw/ra của đoạn), không chứa pointer, địa chỉ, timing,
machine-specific data. Xem `docs/TRACE_PRIVACY.md`.

### Không log

Không logging framework, không `println!`, không `eprintln!` trong core.
Lịch sử thao tác tồn tại trong bộ nhớ phiên và bị xóa khi `dat_lai`/`chap_nhan`.

## Panic audit

Audit `src` cho các pattern gây panic:

| Pattern | Số trong `src` | Ghi chú |
|---|---|---|
| `panic!` | 0 | — |
| `unreachable!` | 0 | — |
| `unimplemented!` | 0 | — |
| `todo!` | 0 | — |
| `unwrap()` | 0 | — |
| `expect(` | 0 | — |
| `unsafe` | 1 | chỉ `#![forbid(unsafe_code)]` (không phải usage) |
| `TODO`/`FIXME` | 0 | (đã dọn TODO duy nhất ở commit contract) |
| indexing trực tiếp `a[i]` | có | chỉ trên index đã kiểm tra boundary (`doan.bat_dau..doan.ket_thuc` từ phân đoạn, `usize::min` cho cursor) |

`debug_assert!` trong `vi_tri.rs::tai_byte` kiểm tra `is_char_boundary` — đây
là invariant, không phải panic trên input; nếu vi phạm là bug nội bộ.

Trong hot path public, Cadence ưu tiên không panic. `them_ky_tu` với input độc
(quá giới hạn, ký tự lạ, combining mark storm) không panic — trả `KhongDoi`
hoặc render nguyên bản.

### `expect()` trong tests/benches/examples

`expect()` chỉ xuất hiện trong `tests/`, `benches/`, `examples/` với message
mô tả invariant (vd `"cau hinh mac dinh luon hop le"`). Điều này cho phép theo
`CONTRIBUTING.md` (chỉ `expect()` khi giải thích được invariant). Core không
dùng `expect()`/`unwrap()`.

## Serialization

- `CauHinh` **không** derive `Deserialize` (tránh bypass validation trong
  `dat_gioi_han_thao_tac`). Nếu sau này cần serialize config, phải viết
  `Deserialize` tùy chỉnh có validate.
- `KetQuaXuLy`, `LoaiNoiDung`, `KieuTelex`, `QuyTacDatDau`, `DangUnicode`,
  `ChinhSachLuaChon` derive serde — là data type đơn giản, không ràng buộc
  validation, không phải snapshot nội bộ.
- Không serialize snapshot (`BanChupSoan`, `ViTriVanBank`) — chưa có use case
  và sẽ rò rỉ raw input nếu log.

## Báo lỗi bảo mật

- Mở issue riêng, mô tả tác động, ghi phiên bản Rust + feature flags + chuỗi
  tái hiện.
- **Không dán nội dung gõ thật của người dùng** vào issue công khai — dùng
  chuỗi mẫu (ASCII, ký tự Unicode công khai).
- Nếu báo lỗ hổng bộ nhớ, ghi rõ feature flags (vd `--no-default-features`).

## Còn phải làm (tương lai)

- FFI repository riêng phải catch `panic::catch_unwind` ở boundary Rust/C dù
  Cadence cố không panic với input hợp lệ. Không triển khai FFI trong Phase này.
- `cargo-deny`/`cargo-audit` CI gate (xem `docs/DEPENDENCIES.md`).
