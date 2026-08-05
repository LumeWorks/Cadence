# RFC 0006 — Telex engine: hình chữ và dấu thanh

Trạng thái: Chấp thuận — Phase 2.

## Vấn đề

Cadence cần biến đổi raw keystroke thành chữ tiếng Việt có dấu theo phương pháp
Telex. Engine phải xác định khi nào áp dụng biến đổi, khi nào giữ nguyên ký tự
raw, và khi nào thoát (escape) bằng cách lặp phím modifier.

## Quyết định

### Hình chữ (shape transforms)

Bảng biến đổi hình chữ (modifier → `DauChu`):

| Phím | Nguyên âm | Dấu chữ     | Kết quả      |
|------|-----------|-------------|--------------|
| `w`  | `a`       | Breve       | ă            |
| `a`  | `a`       | Circumflex  | â            |
| `e`  | `e`       | Circumflex  | ê            |
| `o`  | `o`       | Circumflex  | ô            |
| `w`  | `o`       | Horn        | ơ            |
| `w`  | `u`       | Horn        | ư            |
| `d`  | `d`       | Stroke      | đ            |

Trường hợp đặc biệt: `uo` + `w` → `ươ` (w biến đổi cả `u`→`ư` lẫn `o`→`ơ`).

### Dấu thanh (tone marks)

Bảng dấu thanh (key → `DauThanh`):

| Phím | Dấu     | Tên        |
|------|---------|------------|
| `s`  | Sắc     | Sắc        |
| `f`  | Huyền   | Huyền      |
| `r`  | Hỏi     | Hỏi        |
| `x`  | Ngã     | Ngã        |
| `j`  | Nặng    | Nặng       |
| `z`  | Không   | Xóa dấu    |

### Nguyên âm chính (bo đặt dấu)

Khi áp dụng dấu thanh trên một chuỗi nhiều nguyên âm (diphthong/triphthong),
dấu thanh đặt trên **nguyên âm chính**:

* Nếu nguyên âm cuối là `i` hoặc `u` không dấu hình chữ và có nguyên âm khác
  trước nó → dấu đặt trên nguyên âm trước đó.
* Ngược lại → dấu đặt trên nguyên âm cuối.

Ví dụ: `uơ` + `f` → dấu huyền đặt trên `ư` → `ườ`.

### Escape

Lặp đúng phím modifier đang hoạt động sẽ thoát biến đổi và hiện literal:

* **Escape hình chữ**: `ass` → `as` (lặp `s` nếu `s` là tone key cuối; `aww`
  → `aw` lặp `w` shape; `ddd` → `dd` lặp `d` shape).
* **Escape dấu thanh**: `ass` → `as` (lặp `s` tone key).

Escape luôn giữ kết quả Telex (ý định người dùng), không fallback về raw.

## Lý do

* Telex là phương pháp gõ phổ biến nhất cho tiếng Việt; engine phải xử lý cả
  trường hợp đặc biệt `ươ` (tam nguyên âm).
* Quy tắc nguyên âm chính theo quy ước chính tả tiếng Việt: dấu thanh đặt trên
  nguyên âm tròn môi hoặc mở, không phải bán âm cuối (`i`/`u`).
* Escape bằng lặp phím là hành vi quen thuộc từ VietKey/unikey.

## Phương án bị loại

* **Đặt dấu trên nguyên âm cuối bất kể**: bị loại — sai chính tả (`uơì` thay vì
  `ười`).
* **Bảng vần đầy đủ**: bị loại cho Phase 2 — phức tạp, dành cho Phase 3.
* **Regex cho pattern matching**: bị loại — policy không dùng regex.

## Bất biến

* Mỗi raw position thuộc nhiều nhất một `DonViRender` (không trùng lặp).
* Escape hoàn tác đúng một biến đổi, không ảnh hưởng đơn vị khác.
* `z` (xóa dấu) chỉ consume khi có dấu để xóa; không có → literal.

## Tác động tới Phase sau

* Phase 3 có thể thêm bảng vần đầy đủ để kiểm tra `MucHopLe::HoanChinh`.
* Phase 3 có thể thêm biến đổi VNI (kiểu số).
