# Chính sách ổn định API

Cadence `0.1.0` là bản phát hành đầu tiên có bề mặt public được tài liệu hóa
và "khóa". Tài liệu này nêu mức ổn định của từng phần và cam kết cho các bản
`0.1.x`.

## Phiên bản

Cadence dùng SemVer. Trước `1.0.0`, các bản `0.x.y` có thể có breaking change
theo SemVer, nhưng Cadence tự áp dụng kỷ luật cao hơn:

- `0.1.x`: **cố giữ source compatibility**. Mọi thay đổi phải không bắt
  downstream phải sửa code trừ khi có lý do correctness blocking.
- `0.2.0`: có thể có breaking change có lý do, phải ghi trong CHANGELOG và RFC.
- `1.0.0`: cam kết SemVer đầy đủ.

## Bề mặt ổn định `0.1`

Xem `docs/api/public-api-0.1.0.md` cho danh sách đầy đủ. Tóm tắt mức ổn định:

| Nhóm | Mức |
|---|---|
| `BoGo`, `CauHinh`, `PhienGo`, `BanChupSoan`, `ViTriVanBan`, `LoiCauHinh` | Ổn định 0.1 |
| `KetQuaXuLy`, `LoaiNoiDung`, `KieuTelex`, `QuyTacDatDau`, `DangUnicode`, `ChinhSachLuaChon` | Ổn định 0.1 |
| `TraceStep`, `TraceKetQua`, `BangChungLuaChon` (feature `trace`) | Thử nghiệm |

## Breaking change

Trong `0.1.x`, các thay đổi sau được coi là **breaking** và phải tăng minor
(`0.2.0`) trừ khi là sửa lỗi correctness blocking:

- Đổi chữ ký method public (thêm/xa parameter, đổi kiểu trả về).
- Đổi semantic của `KetQuaXuLy` variant (`KhongDoi`/`CapNhat`/`ChapNhan`).
- Đổi semantic của `LoaiNoiDung` variant.
- Xóa method/variant/field public.
- Đổi giá trị mặc định của `CauHinh::mac_dinh()` theo cách làm downstream phụ
  thuộc vào output cũ.
- Tăng MSRV (xem `docs/MSRV.md`).
- Tăng runtime dependency.

## Thêm enum variant

Thêm variant public vào enum **ổn định** (`KetQuaXuLy`, `LoaiNoiDung`,
`KieuTelex`, `QuyTacDatDau`, `DangUnicode`, `ChinhSachLuaChon`, `LoiCauHinh`)
được coi là breaking cho người dùng match đầy đủ (non-exhaustive chưa được bật).
Cadence **không ép** người dùng dùng wildcard match.

Quy tắc cho `0.1.x`:

- Không thêm variant vào enum ổn định trừ khi correctness blocking.
- Nếu buộc thêm, tăng `0.2.0` và ghi rõ trong CHANGELOG.
- Không bật `#[non_exhaustive]` retroactive trên enum ổn định trong `0.1.x`
  (đó cũng là breaking cho constructor).

## Feature `trace`

`trace` là feature **thử nghiệm** trong `0.1.x`:

- `TraceStep`, `TraceKetQua`, `BangChungLuaChon`, `PhienGo::trace()` có thể
  đổi field/variant/semantic trong `0.1.x` nếu có lý do, ghi trong CHANGELOG.
- Khi `trace` tắt: không code trace nào compile, không overhead, output không
  đổi. Đây là cam kết ổn định (không phụ thuộc vào `trace`).
- Cadence sẽ nâng `trace` lên ổn định ở bản sau khi bề mặt trace đã ổn định qua
  sử dụng thật.

## Sửa lỗi correctness

Sửa lỗi correctness **không được âm thầm đổi contract**. Nếu sửa lỗi yêu cầu
đổi output cho một input đã tài liệu hóa, phải:

1. Ghi rõ trong CHANGELOG: input nào đổi output, cũ → mới, lý do.
2. Thêm regression test.
3. Nếu là breaking cho downstream đã phụ thuộc output cũ, tăng `0.2.0`.

## MSRV

Xem `docs/MSRV.md`. Tóm tắt: MSRV `0.1` là Rust 1.85. Tăng MSRV là breaking
phải ghi CHANGELOG.

## Deprecation

Cadence không có cơ chế deprecation trong `0.1.0`. Nếu một item cần deprecate
trong `0.1.x`:

- Thêm `#[deprecated(note = "...")]` với hướng dẫn thay thế.
- Giữ item hoạt động trong toàn `0.1.x`.
- Xóa ở `0.2.0`.

## Snapshot và ownership

- `BanChupSoan` chỉ đọc; không trả mutable reference.
- `PhienGo::ban_chup()` mượn snapshot từ phiên; hợp lệ đến mutable call tiếp
  theo (xem `docs/INTEGRATION.md`).
- `ViTriVanBan` là `Copy`; host có thể giữ bản sao.

## Cam kết không

Cadence không cam kết:

- Output cụ thể cho input mơ hồ (vd `hoaf.com` → `hòa.com` là hành vi hiện
  tại nhưng có thể đổi nếu sửa selection). Chỉ cam kết bất biến trong
  `INVARIANTS.md`.
- Thứ tự field trong struct public (không phải `#[non_exhaustive]` nhưng field
  private nên không ảnh hưởng).
- Đặt dấu thanh theo thứ tự không tài liệu hóa.
