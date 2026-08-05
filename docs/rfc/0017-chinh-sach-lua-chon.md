# RFC 0017 — Chính sách lựa chọn raw/Telex

Trạng thái: Chấp thuận — Phase 3 (đã triển khai phần enum; wiring đang dở).

## Vấn đề

Phase 2 chỉ có một ngưỡng lựa chọn cố định. Người dùng khác nhau cần mức "hung
tham" Telex khác nhau: lập trình viên muốn code gần như luôn raw, người viết
chattit muốn tiếng Việt gần như luôn biến đổi. Cần một enum thay cho nhiều
boolean.

## Quyết định

`ChinhSachLuaChon` trong `CauHinh` (getter/setter, mặc định `TuNhien`):

```text
TuNhien           — dùng bằng chứng cấu trúc. Chế độ chính.
UuTienTiengViet   — cho phép Telex trong trường hợp mơ hồ hơn.
UuTienNguyenBan   — chỉ biến đổi khi bằng chứng rất rõ.
```

### Thứ tự ưu tiên theo chính sách

Tất cả chính sách **kểu** giữ cấu trúc kỹ thuật chắc chắn (URL, email, code
fence, đường dẫn tuyệt đối, `::`, `=`). Khác biệt nằm ở đoạn `Chu` tự do:

| Chính sách        | Đoạn Chu tự do mơ hồ           | Hình chữ rõ | Âm tiết hoàn chỉnh |
|-------------------|--------------------------------|-------------|--------------------|
| `TuNhien`         | raw nếu không đủ bằng chứng    | Telex       | Telex              |
| `UuTienTiengViet` | Telex nếu có thể là Việt       | Telex       | Telex              |
| `UuTienNguyenBan` | raw trừ khi âm tiết hoàn chỉnh | Telex       | Telex              |

### Bất biến

* Cấu trúc kỹ thuật chắc chắn **luôn** raw trong mọi chính sách.
* `them_nguyen_ban` **luôn** chặn Telex bất kể chính sách.
* `noi_dung_goc()` **luôn** giữ raw byte-for-byte.

## Tác động public API

* Thêm `ChinhSachLuaChon` enum (public).
* Thêm `CauHinh::chinh_sach_lua_chon()` getter.
* Thêm `CauHinh::dat_chinh_sach_lua_chon()` setter.
* `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]`.
* Không breaking change.

## Phương án bị loại

* **Mười boolean (`la_code`, `la_url`, ...)**: bị loại — phình API, trạng thái
  không loại trừ nhau, khó audit.
* **Điểm số ngưỡng `f64`**: bị loại — không deterministic trực quan.
* **Global mutable mode**: bị loại — vi phạm "mỗi PhienGo độc lập".
