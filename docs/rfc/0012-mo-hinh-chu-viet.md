# RFC 0012 — Mô hình chữ viết (domain model)

Trạng thái: Chấp thuận — Phase 2 (đã triển khai).

## Vấn đề

Cadence cần mô hình chữ tiếng Việt để Telex engine và render layer thao tác.
Mô hình phải đại diện được cả ký tự thường và có dấu, hỗ trợ NFC/NFD, và
phân biệt nguyên âm/phụ âm.

## Quyết định

Module `chu_viet.rs` định nghĩa:

### `ChuGoc` — chữ gốc (base letter)

```rust
enum ChuGoc {
    A, Aa, Ao, Ae, E, Ee, I, O, Oo, OoHorn, U, UoHorn, Y, D, PhuAm(char)
}
```

Mỗi variant đại diện một chữ gốc tiếng Việt. `Aa` = â, `Ao` = ă, `Ee` = ê,
`Oo` = ô, `OoHorn` = ơ, `UoHorn` = ư.

### `DauChu` — dấu chữ (shape modifier)

```rust
enum DauChu { Khong, Breve, Circumflex, Horn, Stroke }
```

`Khong` = không dấu. Dấu chữ thay đổi hình nguyên âm: `A` + `Breve` = `ă`.

### `DauThanh` — dấu thanh (tone mark)

```rust
enum DauThanh { Khong, Sac, Huyen, Hoi, Nga, Nang }
```

`Khong` = không dấu thanh (z key xóa dấu).

### `KieuHoa` — kiểu hoa/thường

```rust
enum KieuHoa { Thuong, Hoa }
```

Xác định chữ hoa hay thường. Hoa/thường không ảnh hưởng Telex rules.

### `ChuCaiViet` — chữ cái hoàn chỉnh

```rust
struct ChuCaiViet {
    chu_goc: ChuGoc,
    dau_chu: DauChu,
    dau_thanh: DauThanh,
    kieu_hoa: KieuHoa,
}
```

### Helper methods

* `chu_goc_tu_ky_tu(c)`: ánh xạ char → `ChuGoc`.
* `la_nguyen_am()`: trả `true` nếu `ChuGoc` là nguyên âm.
* `KieuHoa::tu_ky_tu(c)`: xác định hoa/thường từ char.

## Lý do

* Mô hình typed tránh string manipulation dễ lỗi.
* Tách `DauChu` và `DauThanh` rõ ràng vì Telex engine xử lý khác.
* `KieuHoa` riêng vì hoa/thường không ảnh hưởng Telex, chỉ ảnh hưởng render.

## Phương án bị loại

* **String representation**: bị loại — khó pattern match, dễ bug.
* **Gộp `DauChu` và `DauThanh`**: bị loại — ngữ nghĩa khác, xử lý khác.
* **Enum nguyên âm với tất cả combo**: bị loại — bùng nổ tổ hợp, khó duy trì.

## Bất biến

* `ChuCaiViet` với cùng `(chu_goc, dau_chu, dau_thanh, kieu_hoa)` luôn render
  ra cùng output (deterministic).
* `la_nguyen_am()` nhất quán với bảng âm đầu/âm cuối trong `am_tiet.rs`.
* `KieuHoa` không ảnh hưởng `chu_goc` hay dấu.

## Tác động tới Phase sau

* Phase 3 có thể thêm biến thể VNI (kiểu số) vào `DauChu`/`DauThanh`.
* Phase 3 có thể thêm `ChuGoc::PhuAm` đặc biệt cho âm cuối.
