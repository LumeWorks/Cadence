# RFC 0021 - Quy tắc VNI

Trạng thái: Chấp thuận - 2026.1.0 (đã triển khai).

## Vấn đề

VNI dùng chữ số làm modifier. Cần quy tắc rõ ràng cho dấu thanh, dấu hình
chữ, kết hợp, thay dấu, escape, và vị trí dấu — dùng chung `BoDatDau` với
Telex.

## Bất biến

* Digit `1..=9` là modifier VNI; `0` không phải.
* Dấu thanh (`1..=5`) thay dấu cũ, không cộng.
* Dấu hình chữ (`6/7/8/9`) thay dấu chữ cũ.
* Shape và tone độc lập → thứ tự đảo (`a16`/`a61`) cùng kết quả.
* Dùng chung `BoDatDau::tim_nguyen_am_chinh` cho vị trí dấu thanh.
* Escape (lặp digit) hoàn tác đúng một modifier.
* `them_nguyen_ban` chặn modifier.
* Raw history giữ đầy đủ digit.

## Quyết định

### Dấu thanh (1-5)

| Digit | Dấu |
|-------|-----|
| 1 | sắc |
| 2 | huyền |
| 3 | hỏi |
| 4 | ngã |
| 5 | nặng |

Áp dụng lên nguyên âm chính (BoDatDau). Không có nguyên âm → literal.

### Dấu hình chữ (6-9)

| Digit | Dấu | Chữ tương thích |
|-------|-----|-----------------|
| 6 | mũ | a, e, o |
| 7 | móc | o, u |
| 8 | trăng | a |
| 9 | gach (đ) | d |

Áp dụng lên nguyên âm cuối cùng tương thích (không phải nguyên âm chính,
vì shape cần vowel tương thích, không phải bán âm rule).

### ươ đặc biệt

`7` trên `u` khi đơn vị kế tiếp là `o` không dấu → biến đổi cả `u`→`ư`
và `o`→`ơ`. Tương tự `7` trên `o` khi đơn vị trước là `u` không dấu.
Giống `uo`+`w` Telex.

### Kết hợp shape + tone

`a61` → ấ (mũ rồi sắc). `a16` → ấ (sắc rồi mũ). Shape và tone độc lập.

### Thay dấu

Digit mới thay digit cũ: `a12` → à (huyền thay sắc). `a68` → ă (trăng
thay mũ).

### Vị trí dấu thanh

Dùng chung `BoDatDau::tim_nguyen_am_chinh`:
* Bán âm cuối `i/u/o/y` → tone trên nguyên âm trước.
* Ngoại lệ `qu`+`y` → tone trên `y` (vd `quý`).
* On-glide `oa`/`oe` → HienDai trên `o`, TruyenThong trên `a`/`e`.

### Escape

Lặp đúng digit modifier đang hoạt động → hoàn tác modifier đó, hiện digit
đầu thành literal, consume digit thứ hai. Giống escape Telex.

## Rule table

| Input | Output | Lý do |
|-------|--------|-------|
| `a1` | á | sắc |
| `a6` | â | mũ |
| `a61` | ấ | mũ + sắc |
| `a16` | ấ | sắc + mũ (đảo) |
| `a12` | à | huyền thay sắc |
| `a11` | a1 | escape sắc |
| `a66` | a6 | escape mũ |
| `d9` | đ | gach |
| `uo7` | ươ | ươ đặc biệt |

## Ví dụ

```
tieng61 → tiếng
nguo7i2 → người
d9uo7ng2 → đường
thuy3 → thủy
quy1 → quý
khuyu3 → khuỷu
```

## Phản ví dụ

* `i6` → `i6` (i không nhận mũ → literal).
* `e7` → `e7` (e không nhận móc → literal).
* `a0` → `a0` (0 không phải modifier).

## Phương án bị loại

* **VNI tự đặt dấu bằng index**: bị loại — phải dùng chung BoDatDau.
* **Bảng vần đầy đủ**: bị loại — policy không dùng từ điển.
* **Thứ tự cố định shape-then-tone**: bị loại — engine phổ biến chấp nhận cả hai.

## Tác động public API

Không thêm API mới ngoài `KieuGo::Vni`.

## Tác động hiệu năng

VNI modifier loop cùng độ phức tạp Telex. Không allocation bậc hai.

## Tác động serde

Không thay đổi.

## Tác động no_std

Không thay đổi.

## Migration

Không cần — VNI là tính năng mới.

## Điều kiện xem xét lại

* Khi thêm digit modifier mới (không có trong VNI chuẩn).
* Khi cần policy thay dấu khác (vd toggle thay replace).
