# RFC 0023 - Escape và hoàn tác VNI

Trạng thái: Chấp thuận - 2026.1.0 (đã triển khai).

## Vấn đề

VNI cần contract rõ cho escape (thoát modifier) và hoàn tác (backspace).
Không được bịa số đặc biệt; phải dựa trên hành vi VNI phổ biến và semantic
raw history.

## Bất biến

* Escape: lặp đúng digit modifier đang hoạt động → hoàn tác modifier đó.
* Escape không mất raw: digit đầu hiện literal, digit thứ hai consumed.
* Backspace hoàn tác đúng một raw action.
* Restore raw phục hồi chính xác từ raw history, không suy ngược Unicode.
* `them_nguyen_ban` chặn modifier, không biến đổi.
* Mọi hành vi deterministic và replay được.

## Quyết định

### Escape

Lặp đúng digit modifier đang hoạt động → hoàn tác modifier đó:
* `a11` → `a1` (apply sắc, escape → literal `1`).
* `a66` → `a6` (apply mũ, escape → literal `6`).
* `d99` → `d9` (apply gach, escape → literal `9`).

Escape hoàn tác trên tất cả target của modifier (xử lý ươ: `uo77` → `uo7`).

### Backspace

Backspace hoàn tác một raw action:
* `ấ` (raw `a61`) → xóa lùi → `â` (raw `a6`) → xóa lùi → `a` → rỗng.
* `đ` (raw `d9`) → xóa lùi → `d` → rỗng.

Cursor không mắc ở modifier số vô hình.

### Restore raw

`noi_dung_goc()` trả byte-for-byte raw (`a61`, `d9`, `tieng61`).
Không phục hồi bằng cách suy ngược Unicode.

### `them_nguyen_ban`

`them_ky_tu('a')` + `them_nguyen_ban('1')` → `a1` (không phải `á`).
Ký tự nguyên bản chặn modifier nối xuyên.

## Rule table

| Input | Output | Lý do |
|-------|--------|-------|
| `a11` | a1 | escape sắc |
| `a66` | a6 | escape mũ |
| `a1111` | a11 | apply, escape, apply, escape |
| `a61` + bs | â | backspace hoàn tác 1 raw |
| `a61` + bs + bs | a | backspace 2 raw |
| `a` + raw `1` | a1 | nguyen_ban chặn |

## Ví dụ

```
a61 → ấ → bs → â → bs → a → bs → rỗng
d9 → đ → bs → d → bs → rỗng
uo77 → uo7 (escape ươ)
```

## Phản ví dụ

* Số đặc biệt để xóa dấu — bị loại, VNI phổ biến không dùng.
* Suy ngược Unicode để restore raw — bị loại, không deterministic.

## Phương án bị loại

* **Digit 0 xóa dấu**: bị loại — VNI phổ biến không có contract này.
* **Suy ngược Unicode**: bị loại — không reliable, mất raw.
* **Nhấn lặp modifier để xóa (không phải escape)**: escape đã đủ.

## Tác động public API

Không thêm API mới.

## Tác động hiệu năng / serde / no_std

Không thay đổi.

## Điều kiện xem xét lại

* Khi có bằng chứng black-box về contract xóa dấu khác.
