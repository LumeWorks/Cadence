# Nguồn nghiên cứu VNI

Tài liệu này ghi lại hành vi black-box và quy tắc VNI công khai được tham
khảo khi triển khai VNI trong Cadence. Không copy implementation GPL. Không
dịch source engine khác sang Rust.

## Nguồn công khai

### Quy tắc VNI chuẩn

VNI (Vietnamese Number Input) dùng chữ số làm modifier. Quy tắc công khai:

| Digit | Tác dụng |
|-------|----------|
| 1 | sắc (acute) |
| 2 | huyền (grave) |
| 3 | hỏi (hook above) |
| 4 | ngã (tilde) |
| 5 | nặng (dot below) |
| 6 | mũ (circumflex): a→â, e→ê, o→ô |
| 7 | móc (horn): o→ơ, u→ư |
| 8 | trăng (breve): a→ă |
| 9 | gach (stroke): d→đ |

Digit `0` không phải modifier.

### Thứ tự kết hợp

VNI phổ biến chấp nhận cả hai thứ tự shape + tone:
* `a61` → ấ (shape rồi tone).
* `a16` → ấ (tone rồi shape).

Cadence hỗ trợ cả hai thứ tự (xem RFC 0021).

### ươ đặc biệt

`uo` + `7` → `ươ` (cả `u`→`ư` và `o`→`ơ`). Tương tự `ow` trong Telex.
Cadence triển khai cùng hành vi (xem RFC 0021).

### Escape

VNI phổ biến không có contract escape thống nhất như Telex. Một số engine
dùng lặp digit để thoát, một số không. Cadence chọn lặp digit modifier để
escape (giống Telex) vì:
* Nhất quán với Telex.
* Deterministic.
* Raw không mất.

### Xóa dấu

VNI phổ biến không có digit xóa dấu riêng. Thay dấu bằng digit mới là cách
chuẩn. Cadence theo contract này.

## Hành vi black-box quan sát

* Digit mà không có nguyên âm phù hợp → giữ nguyên digit (literal).
* Digit `0` → luôn literal.
* `i`/`y` không nhận mũ/móc/trăng.
* `e` không nhận móc.
* `u` không nhận mũ/trăng.

## Giới hạn

Cadence không sao chép implementation từ engine GPL. Quy tắc được triển
khai từ mô hình domain (`ChuCaiViet`, `DauChu`, `DauThanh`) và `BoDatDau`
dùng chung, không từ code engine khác.
