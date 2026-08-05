# RFC 0010 - Ánh xạ con trỏ raw ↔ byte, grapheme

Trạng thái: Chấp thuận - Phase 2 (đã triển khai).

## Vấn đề

Cadence lưu lịch sử raw thao tác (`Vec<ThaoTacNhap>`). Telex engine biến đổi raw
thành output có dấu (longer/shorter). Con trỏ người dùng cần được ánh xạ đúng
giữa raw position và byte offset trong output, và phải snap về ranh giới
grapheme cluster.

## Quyết định

### Con trỏ raw

`con_tro` trong `PhienGo` là raw position (index trong `Vec<ThaoTacNhap>`).
Navigation (`di_trai`, `di_phai`) di chuyển theo ranh giới đơn vị render
(grapheme), nhưng cập nhật raw position.

### Ánh xạ raw → byte

Module `anh_xa.rs` xây dựng `raw_to_byte: Vec<usize>` - mỗi raw position maps
to byte offset trong output. Mapping này được tính lại mỗi khi rebuild
pipeline (`xay_lai`).

### Snap cho snapshot

Khi tạo `BanChupSoan`, `chi_so_byte` được snap:
* `snap_raw(raw_to_byte, raw_pos)` → byte offset.
* Byte offset luôn nằm ở ranh giới UTF-8 hợp lệ.

### Navigation

* `di_trai_raw`: tìm raw position của đơn vị render trước con trỏ.
* `di_phai_raw`: tìm raw position của đơn vị render sau con trỏ.
* `tinh_navigable`: xây dựng danh sách ranh giới grapheme để navigation.

### Backspace

`xoa_lui` hoàn tác đúng một raw action. Vì raw là source of truth, backspace
chỉ cần `pop()` thao tác cuối cùng.

## Lý do

* Raw là source of truth; con trỏ raw tránh mất vị trí khi rebuild.
* Snap chỉ cho snapshot byte offset, không thay đổi `con_tro` nội bộ.
* Grapheme boundary đảm bảo con trỏ không nằm giữa combining mark và base char.

## Phương án bị loại

* **Con trỏ theo byte**: bị loại - byte offset thay đổi khi rebuild, khó duy
  trì.
* **Con trỏ theo grapheme**: bị loại - grapheme count thay đổi khi rebuild.
* **Con trỏ snap permanent**: bị loại - navigation không tuyến tính khi con trỏ
  luôn snap.

## Bất biến

* `con_tro` (raw) luôn nằm trong `[0, thao_tac.len()]`.
* `chi_so_byte` trong snapshot luôn là char boundary.
* `chi_so_grapheme` trong snapshot luôn là grapheme boundary.
* Backspace chỉ xóa một raw action, không xóa theo byte/grapheme.

## Tác động tới Phase sau

* Phase 3 có thể thêm cursor movement theo word/line.
* Phase 3 có thể thêm selection range (highlight).
