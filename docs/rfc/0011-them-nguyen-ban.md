# RFC 0011 — `them_nguyen_ban` bypass Telex

Trạng thái: Chấp thuận — Phase 2 (đã triển khai).

## Vấn đề

Host cần cách chèn text không qua Telex engine (paste, speech-to-text, undo
redo text block). Telex không được can thiệp vào nội dung này.

## Quyết định

`them_nguyen_ban(ky_tu)` chèn ký tự vào lịch sử raw với loại thao tác
`NguyenBan`. Khi rebuild pipeline:

* Mọi ký tự `them_nguyen_ban` được render literal — không áp dụng shape, tone,
  hay escape.
* `them_nguyen_ban` chặn Telex rules: sau `them_nguyen_ban`, các raw action
  Telex tiếp theo không được biến đổi ký tự trước `them_nguyen_ban`.
* `noi_dung_goc()` trả byte-for-byte raw, bao gồm cả `them_nguyen_ban`.

### Biểu diễn trong pipeline

Telex engine chia raw thành các đoạn (token). Mỗi `them_nguyen_ban` tạo ranh
giới đoạn: Telex rules chỉ áp dụng trong đoạn, không xuyên qua `them_nguyen_ban`.

## Lý do

* Paste text có dấu tiếng Việt (đã NFC) không cần Telex biến đổi.
* Undo/redo text block phải giữ nguyên nội dung, không re-process.
* Tách ranh giới đoạn đơn giản hơn marker phức tạp.

## Phương án bị loại

* **Telex xử lý `them_nguyen_ban` như `them_ky_tu`**: bị loại — Telex sẽ cố
  biến đổi ký tự có dấu, tạo kết quả sai.
* **Lưu `them_nguyen_ban` ở tầng riêng, không trong raw history**: bị loại —
  phá bất biến "raw là source of truth", khó undo.
* **Marker trong output**: bị loại — phức tạp, dễ bug.

## Bất biến

* `them_nguyen_ban` chars luôn xuất hiện literal trong output (nếu được giữ).
* `them_nguyen_ban` tạo ranh giới đoạn Telex; Telex không xuyên qua.
* Backspace trên `them_nguyen_ban` xóa đúng một raw action.

## Tác động tới Phase sau

* Phase 3 có thể thêm `them_nguyen_ban_chuoi(&str)` cho paste nhanh.
* Phase 3 có thể thêm tùy chọn "re-process sau paste" nếu người dùng muốn.
