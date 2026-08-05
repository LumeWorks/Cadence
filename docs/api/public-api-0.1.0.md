# Public API 0.1.0 — inventory

Bản kê toàn bộ public item của crate `cadence` tại bản `0.1.0`, phân loại
mức ổn định. Đây là bề mặt public được "khóa" sau commit khóa API.

Quy ước phân loại:

| Nhãn | Ý nghĩa |
|---|---|
| `Ổn định 0.1` | Cam kết giữ source-compat trong `0.1.x`. Thay đổi là breaking. |
| `Thử nghiệm` | Chỉ tồn tại dưới feature; có thể đổi trong `0.1.x` nếu ghi rõ. |
| `Nội bộ lộ` | Field/variant đang `pub` nhưng nên `pub(crate)` — sẽ thu hẹp. |
| `Cần đổi 0.1` | Cần sửa trước commit khóa API. |

Tất cả item dưới đây đã được xem và **không** có nhãn `Cần đổi 0.1` trừ khi
ghi rõ. Bề mặt đã nhỏ và không lộ parser/rule/buffer nội bộ.

## Module public

| Module | Trạng thái | Ghi chú |
|---|---|---|
| `cadence::ban_chup` | Ổn định 0.1 | `BanChupSoan` |
| `cadence::bo_go` | Ổn định 0.1 | `BoGo` |
| `cadence::cau_hinh` | Ổn định 0.1 | config enums + `CauHinh` |
| `cadence::ket_qua` | Ổn định 0.1 | `KetQuaXuLy` |
| `cadence::loai_noi_dung` | Ổn định 0.1 | `LoaiNoiDung` |
| `cadence::phien_go` | Ổn định 0.1 | `PhienGo` |
| `cadence::vi_tri` | Ổn định 0.1 | `ViTriVanBan` |
| `cadence::trace` | Thử nghiệm | chỉ khi feature `trace` |

Re-export tại `lib.rs` cũng ổn định: `BoGo`, `CauHinh`, `ChinhSachLuaChon`,
`DangUnicode`, `KieuTelex`, `LoiCauHinh`, `QuyTacDatDau`, `KetQuaXuLy`,
`LoaiNoiDung`, `PhienGo`, `BanChupSoan`, `ViTriVanBan`. Trace re-export
`BangChungLuaChon`, `TraceKetQua`, `TraceStep` (feature `trace`).

## Struct

### `BoGo`
- `#[derive(Debug, Clone)]`, `Send + Sync` (tự động, chỉ chứa `CauHinh` Copy).
- `new(cau_hinh: CauHinh) -> Result<Self, LoiCauHinh>` — Ổn định 0.1.
  Trả `Ok` vì `CauHinh` chỉ mang giá trị hợp lệ; giữ `Result` cho
  forward-compat (config validation tương lai).
- `tao_phien(&self) -> PhienGo` — Ổn định 0.1. `#[must_use]`.
- `cau_hinh(&self) -> &CauHinh` — Ổn định 0.1. `#[must_use]`.

### `CauHinh`
- `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`, `Send + Sync`.
- Field private; thay đổi qua method có validate.
- `mac_dinh() -> Self` — Ổn định 0.1. `#[must_use]`.
- `gioi_han_thao_tac(self) -> usize` — Ổn định 0.1.
- `dat_gioi_han_thao_tac(&mut self, usize) -> Result<(), LoiCauHinh>` — Ổn định 0.1.
- `kieu_telex(self) -> KieuTelex` / `dat_kieu_telex(&mut self, KieuTelex)` — Ổn định 0.1.
- `quy_tac_dat_dau(self) -> QuyTacDatDau` / `dat_quy_tac_dat_dau(&mut self, QuyTacDatDau)` — Ổn định 0.1.
- `dang_unicode(self) -> DangUnicode` / `dat_dang_unicode(&mut self, DangUnicode)` — Ổn định 0.1.
- `chinh_sach_lua_chon(self) -> ChinhSachLuaChon` / `dat_chinh_sach_lua_chon(&mut self, ChinhSachLuaChon)` — Ổn định 0.1.
- Không derive serde (tránh bypass validation — xem `docs/README.md`).

### `PhienGo`
- Không derive trait công khai; `Send + Sync` (tự động, đã kiểm chứng trong
  `tests/contract.rs`); rustdoc ghi cam kết threading.
- `ban_chup(&self) -> &BanChupSoan` — Ổn định 0.1. `#[must_use]`.
- `trace(&self) -> &[TraceStep]` — Thử nghiệm (feature `trace`). `#[must_use]`.
- `dang_trong(&self) -> bool` — Ổn định 0.1. `#[must_use]`.
- `them_ky_tu(&mut self, char) -> KetQuaXuLy` — Ổn định 0.1.
- `them_nguyen_ban(&mut self, char) -> KetQuaXuLy` — Ổn định 0.1.
- `di_trai(&mut self) -> KetQuaXuLy` — Ổn định 0.1.
- `di_phai(&mut self) -> KetQuaXuLy` — Ổn định 0.1.
- `xoa_lui(&mut self) -> KetQuaXuLy` — Ổn định 0.1.
- `xoa_phia_truoc(&mut self) -> KetQuaXuLy` — Ổn định 0.1.
- `ve_dau(&mut self) -> KetQuaXuLy` — Ổn định 0.1.
- `ve_cuoi(&mut self) -> KetQuaXuLy` — Ổn định 0.1.
- `khoi_phuc_nguyen_ban(&mut self) -> KetQuaXuLy` — Ổn định 0.1 (no-op, idempotent; raw luôn có qua `noi_dung_goc()`).
- `chap_nhan(&mut self) -> KetQuaXuLy` — Ổn định 0.1.
- `dat_lai(&mut self)` — Ổn định 0.1.

### `BanChupSoan`
- `#[derive(Debug, Clone, PartialEq, Eq)]`.
- Constructor `rong()` và `dung()` là `pub(crate)` (không lộ).
- `noi_dung(&self) -> &str` — Ổn định 0.1. `#[must_use]`.
- `noi_dung_goc(&self) -> &str` — Ổn định 0.1. `#[must_use]`.
- `con_tro(&self) -> ViTriVanBan` — Ổn định 0.1. `#[must_use]`.
- `loai_noi_dung(&self) -> LoaiNoiDung` — Ổn định 0.1. `#[must_use]`.
- `dang_trong(&self) -> bool` — Ổn định 0.1. `#[must_use]`.

### `ViTriVanBan`
- `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`, `Send + Sync`.
- `chi_so_byte(self) -> usize` — Ổn định 0.1. `#[must_use]`.
- `chi_so_utf16(self) -> usize` — Ổn định 0.1. `#[must_use]`.
- `chi_so_grapheme(self) -> usize` — Ổn định 0.1. `#[must_use]`.

## Enum

### `KetQuaXuLy` — Ổn định 0.1
- `KhongDoi`, `CapNhat`, `ChapNhan { noi_dung: String }`.
- `#[derive(Debug, Clone, PartialEq, Eq)]`; serde khi feature `serde`.
- Semantic:
  - `KhongDoi`: thao tác không đổi state (vượt giới hạn, xóa khi rỗng).
  - `CapNhat`: state đổi; lấy snapshot qua `ban_chup()`.
  - `ChapNhan`: phiên commit; trả nội dung; phiên reset về rỗng.

### `LoaiNoiDung` — Ổn định 0.1
- `Trong`, `NguyenBan`, `BienDoiTelex`, `AmTietTiengViet`.
- `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`; serde khi feature `serde`.

### `KieuTelex` — Ổn định 0.1
- `CanBang`, `DayDu`. serde khi feature `serde`.

### `QuyTacDatDau` — Ổn định 0.1
- `HienDai`, `TruyenThong`. serde khi feature `serde`.

### `DangUnicode` — Ổn định 0.1
- `Nfc`, `Nfd`. serde khi feature `serde`.

### `ChinhSachLuaChon` — Ổn định 0.1
- `TuNhien`, `UuTienTiengViet`, `UuTienNguyenBan`. serde khi feature `serde`.

### `LoiCauHinh` — Ổn định 0.1
- `GioiHanThaoTacKhongHopLe { gioi_han, toi_thieu, toi_da }`.
- `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`; `Display`; `std::error::Error` khi `std`.

### `TraceStep` — Thử nghiệm (feature `trace`)
- Field `pub`: `doan_bat_dau`, `doan_ket_thuc`, `bang_chung`, `ket_qua`,
  `chuoi_raw`, `chuoi_ra`. `#[derive(Debug, Clone)]`.
- Field public vì trace là snapshot chỉ đọc cho host tooling.

### `TraceKetQua` — Thử nghiệm (feature `trace`)
- `Telex`, `NguyenBan`. `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`.

### `BangChungLuaChon` — Thử nghiệm (feature `trace`)
- 12 variant: `AmTietTiengVietHoanChinh`, `BienDoiHinhChuRoRang`,
  `PhimDauHopLe`, `PhanCachIdentifier`, `CauTrucUrl`, `CauTrucEmail`,
  `CauTrucDuongDan`, `CauTrucCommand`, `ChuoiSoKyThuat`, `KyTuLapChat`,
  `Emoticon`, `NguyenBanDoNguoiGoiYeuCau`.
- `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`.

## Hằng số / type alias

Không có public constant hay type alias.

## Trait impl công khai

- `LoiCauHinh: std::error::Error` (feature `std`).
- `LoiCauHinh: Display`.
- Debug cho mọi struct/enum công khai.
- `Clone` cho mọi struct/enum công khai.
- serde `Serialize`/`Deserialize` cho `KetQuaXuLy`, `LoaiNoiDung`,
  `KieuTelex`, `QuyTacDatDau`, `DangUnicode`, `ChinhSachLuaChon` (feature `serde`).
- `Send + Sync` cho `CauHinh`, `BoGo`, `ViTriVanBan`, `BanChupSoan`,
  `PhienGo` (tự động); `PhienGo: Send + Sync` đã kiểm chứng trong
  `tests/contract.rs`.

## Item không lộ (cố ý `pub(crate)`)

- `am_tiet`, `anh_xa`, `chu_viet`, `lua_chon`, `ngu_canh`, `phan_doan`,
  `render`, `telex`, `thao_tac` — toàn bộ nội bộ.
- `ThaoTacNhap`, `CachNhap`, `DonViRender`, `NoiDungDonVi`, `Doan`, `LoaiDoan`,
  `ChuGoc`, `DauChu`, `DauThanh`, `KieuHoa`, `ChuCaiViet`, `KetQuaTelex`,
  `KetQuaNhanDien`, `KetQuaLuaChon`, `KetQuaRender`, `RenderDoan`, `MucHopLe`.
- Không public hóa parser, rule table, candidate nội bộ, provenance nội bộ,
  segment state, mutable buffer, raw `Vec`, internal index, transport action.

## Nhận xét

- Bề mặt public đã nhỏ và nhất quán: 6 struct + 6 enum (ổn định) + 3 item
  trace (thử nghiệm).
- Không có item `Nội bộ lộ` hay `Cần đổi 0.1` — bề mặt sạch trước khóa.
- `BoGo::new` trả `Result` dù luôn `Ok`: giữ cho forward-compat, đã tài liệu.
- `PhienGo::khoi_phuc_nguyen_ban` no-op idempotent: đã tài liệu trong rustdoc
  và `INVARIANTS.md`.
