# Tích hợp Cadence

Tài liệu này mô tả contract host tích hợp lõi Cadence. Cadence là Rust library
crate thuần, không FFI/IPC/network/thread. Host sở hữu vòng đời và luồng.

## Thêm dependency

```toml
[dependencies]
cadence = { package = "cadence-ime", version = "0.1" }
```

Package crates.io là `cadence-ime`; library target là `cadence` nên host dùng
`use cadence::BoGo;`.

```rust
use cadence::{BoGo, CauHinh, KetQuaXuLy};
```

## Feature flags

| Feature | Mặc định | Mô tả |
|---|---|---|
| `std` | có | Dùng thư viện chuẩn (`std::error::Error` cho `LoiCauHinh`). |
| `serde` | không | Derive `Serialize`/`Deserialize` cho `KetQuaXuLy`, `LoaiNoiDung`, `KieuTelex`, `QuyTacDatDau`, `DangUnicode`, `ChinhSachLuaChon`. |
| `trace` | không | `PhienGo::trace()` + `TraceStep`/`TraceKetQua`/`BangChungLuaChon`. |
| `no_std + alloc` | tắt `std` | Biên dịch cho môi trường không `std` (cần `alloc`). |

`CauHinh` **không** derive serde (tránh bypass validation trong
`dat_gioi_han_thao_tac`). Nếu host cần serialize config, phải viết
`Deserialize` tùy chỉnh có validate.

## Vòng đời cơ bản

```text
BoGo::new(CauHinh) -> Result<BoGo, LoiCauHinh>
BoGo::tao_phien()  -> PhienGo          (phiên độc lập, #[must_use])
PhienGo::them_ky_tu(char) -> KetQuaXuLy
PhienGo::ban_chup()       -> &BanChupSoan   (snapshot hiện tại)
PhienGo::chap_nhan()      -> KetQuaXuLy::ChapNhan { noi_dung } rồi reset
PhienGo::dat_lai()        -> xóa toàn bộ state
```

`BoGo` là factory bất biến (chỉ chứa `CauHinh: Copy`). Mỗi `tao_phien()` tạo
phiên độc lập; hai phiên không chia sẻ state.

## Snapshot và ownership

- `PhienGo::ban_chup(&self) -> &BanChupSoan` mượn snapshot từ phiên. Reference
  hợp lệ đến **mutable call tiếp theo** trên phiên đó (`them_ky_tu`, `xoa_lui`,
  `di_trai`, `chap_nhan`, `dat_lai`, v.v.). Đừng giữ reference qua mutable call.
- Nếu host cần giữ snapshot lâu hơn, `clone` `BanChupSoan` (`Clone`) hoặc copy
  `String`/`&str` ra khỏi reference.
- `ViTriVanBan` là `Copy`. Host có thể giữ bản sao (`chi_so_byte`/`chi_so_utf16`/
  `chi_so_grapheme`) mà không mượn phiên.

## Cursor

`ViTriVanBan` trả ba đơn vị vị trí:

- `chi_so_byte`: UTF-8 byte offset, luôn là char boundary (kiểm chứng invariant).
- `chi_so_utf16`: số UTF-16 code unit tính từ đầu (cho LSP/JS bridge).
- `chi_so_grapheme`: số grapheme cluster hiển thị (cho cursor người dùng).

Cursor không bao giờ nằm giữa một code point hay giữa grapheme cluster. Khi
di chuyển vào grapheme phân rã, cursor snap về ranh giới cluster.

## Commit / reset

- `chap_nhan` phiên rỗng → `KetQuaXuLy::KhongDoi` (không đổi state).
- `chap_nhan` phiên có nội dung → `KetQuaXuLy::ChapNhan { noi_dung }` rồi reset
  phiên về rỗng hoàn toàn (lịch sử, cursor, snapshot).
- `dat_lai` xóa toàn bộ state (như `chap_nhan` nhưng không trả nội dung).
- `khoi_phuc_nguyen_ban` là **no-op idempotent** (trả `KhongDoi`). Raw luôn có
  qua `BanChupSoan::noi_dung_goc()`; không có chế độ xem raw riêng để giữ
  invariant "lịch sử là nguồn sự thật duy nhất" (RFC 0002).

## Giới hạn thao tác

`CauHinh::gioi_han_thao_tac` (mặc định 128, phạm vi `1..=4096`). Khi phiên đạt
giới hạn, `them_ky_tu`/`them_nguyen_ban` trả `KhongDoi` và giữ nguyên state
(không tự xóa đầu buffer). Host có thể `chap_nhan` rồi tiếp tục.

## Threading

`BoGo`, `CauHinh`, `ViTriVanBan`, `BanChupSoan`, `PhienGo` đều `Send + Sync`
(tự động; `PhienGo` kiểm chứng trong `tests/contract.rs`). Cadence không tạo
thread, không lock, không global state. Trách nhiệm threading thuộc host:

- Một `PhienGo` chỉ nên truy cập từ **một luồng tại một thời điểm**. Muốn dùng
  chung nhiều luồng, host bọc bằng `Mutex`/channel (Cadence cố ý không kéo
  lock vào core).
- `BoGo` có thể `Clone` và chia sẻ miễn phí (chỉ chứa `CauHinh: Copy`).

## no_std

Tắt `std` (`--no-default-features` hoặc `default-features = false`), Cadence biên
dịch cho `no_std + alloc`. `LoiCauHinh` chỉ implement `std::error::Error` dưới
`#[cfg(feature = "std")]`; khi tắt `std`, host dùng `Display` của `LoiCauHinh`.

## Bảo mật / quyền riêng tư khi tích hợp

Cadence không log, không network, không I/O. **Nội dung người dùng gõ là dữ
liệu nhạy cảm.** Host tích hợp nên:

1. Không log `noi_dung_goc()` hay `chuoi_raw` mặc định (xem `docs/TRACE_PRIVACY.md`).
2. Trace (`feature = "trace"`) chỉ chứa token hiện tại per-đoạn, không chứa
   pointer/timing. Chỉ bật khi debug; tắt trong production build.
3. Redact nội dung thật khi báo bug (dùng chuỗi mẫu).
4. `CauHinh` không derive `Deserialize` — nếu host nhận config không tin cậy,
   phải validate trước khi `dat_gioi_han_thao_tac`.

## Không thuộc core

Cadence không có FFI, GUI, CLI sản phẩm, IPC, D-Bus, nhận diện ứng dụng, async
runtime. Đó là vai trò của CadenceRuntime — repository riêng.
