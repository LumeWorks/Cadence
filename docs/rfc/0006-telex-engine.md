# RFC 0006 - Telex engine: hình chữ và dấu thanh

Trạng thái: Chấp thuận - Phase 2 (đã triển khai).

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

### Thứ tự linh hoạt (shape reach back)

Phím hình chữ (`w`/`a`/`e`/`o`/`d`) reach back tới **base trần** (chưa có
dấu hình chữ) gần nhất trong đoạn, không nhất thiết ngay sau base. Người gõ
thường chèn dấu ở khắp nơi vì tiện tay; engine theo chứ không ép thứ tự. Parity
với VNI (RFC 0021: `a16`/`a61`, `toi6`→tôi).

| Gõ      | Kết quả | Diễn giải                                  |
|---------|---------|--------------------------------------------|
| `oiw`   | `ơi`    | `w` cách base `o` qua bán âm `i`            |
| `voiws` | `với`   | `w` cách `o` qua `i`, `s` sắc               |
| `uoiw`  | `ươi`   | `w` horn cả `u` và `o` (ươ) dù `i` xen giữa |
| `khongo`| `không` | `o` cuối restroke `o` đầu qua phụ âm `ng`  |
| `uongw` | `ương`  | `w` horn cặp `uo` qua phụ âm `ng`          |

Chỉ reach back tới base **trần** (`dau_chu == Khong`), nên `aaw`→`âw` (â đã có
Mu, `w` không restroke thành ă).

### Chặn reshape tiếng Anh (shape ở xa + âm tiết không hợp lệ)

Shape reach back qua ký tự khác ("ở xa") có thể biến đổi tiếng Anh thành rác
(vd `cadence`→`cadênc`, `release`→`rêláe`). `lua_chon` Rule 2 chặn: shape ở xa
+ âm tiết không hợp lệ → raw. Shape liền base (adjacency) cho gõ dở (`ddm`→`đm`)
không bị chặn. Tone-only (`text`→`tẽt`, `use`→`úe`) không bị ảnh hưởng.

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

* **Adjacency cứng (shape chỉ khi modifier ngay sau base)**: bị loại - lệch
  VNI (RFC 0021 cho phép `toi6`→tôi, `di9`→đi), và người gõ không bao giờ tuân
  thủ thứ tự chính xác (`voiws` thay vì `vowsi`). Forward-peek `i+1` ép raw cho
  mọi input chèn dấu giữa.
* **Đặt dấu trên nguyên âm cuối bất kể**: bị loại - sai chính tả (`uơì` thay vì
  `ười`).
* **Bảng vần đầy đủ**: bị loại cho Phase 2 - phức tạp, dành cho Phase 3.
* **Regex cho pattern matching**: bị loại - policy không dùng regex.

## Bất biến

* Mỗi raw position thuộc nhiều nhất một `DonViRender` (không trùng lặp).
* Escape hoàn tác đúng một biến đổi, không ảnh hưởng đơn vị khác.
* `z` (xóa dấu) chỉ consume khi có dấu để xóa; không có → literal.
* Shape modifier reach back tới base trần trong đoạn (`segment_start`), không
  xuyên ranh giới `them_nguyen_ban`.
* Shape ở xa (reach back qua ký tự khác) + âm tiết không hợp lệ → raw (chặn
  reshape tiếng Anh); shape liền base không bị chặn (hỗ trợ gõ dở).

## Tác động tới Phase sau

* Phase 3 có thể thêm bảng vần đầy đủ để kiểm tra `MucHopLe::HoanChinh`.
* Phase 3 có thể thêm biến đổi VNI (kiểu số).

## Ghi chú triển khai

* `tim_base_hinh_chu` trong `telex.rs` tìm ngược base trần gần nhất tương thích
  với modifier (`w`/`a`/`e`/`o`/`d`), mutate `dau_chu` tại chỗ (không tạo đơn
  vị mới). ươ đặc biệt horn cả `u` và `o` khi cặp liền nhau.
* `tim_nguyen_am_chinh` trong `bo_dat_dau.rs` triển khai quy tắc nguyên âm
  chính với 3 trường hợp: bán âm cuối (`i`/`u`/`o`), on-glide (`o`+`a`/`e`), và
  mặc định (nguyên âm cuối).
* Bán âm cuối mở rộng `o` (không chỉ `i`/`u`) để xử lý `ao`, `eo`, `ưo`.
* `segment_start` theo dõi ranh giới `them_nguyen_ban` để chặn tone/shape xuyên.
* `lua_chon` Rule 2: shape ở xa + âm tiết không hợp lệ → raw (chặn reshape
  tiếng Anh); `co_hinh_xa` từ engine đánh dấu shape reach back qua ký tự khác.
* 757 tests xác minh hành vi (xem `tests/telex_*.rs`,
  `tests/telex_thu_tu_linh_hoat.rs`).
