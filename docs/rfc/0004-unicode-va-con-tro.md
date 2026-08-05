# RFC 0004 - Unicode và con trỏ

Trạng thái: Chấp thuận - Phase 1.

## Vấn đề

Snapshot phải trung lập nền tảng: host dùng UTF-16 (Windows TSF) hoặc grapheme
(trình soạn thảo) không được tự tính lại vị trí con trỏ. `char` không bằng ký tự
hiển thị (emoji, combining mark, ZWJ).

## Quyết định

`ViTriVanBan` cung cấp vị trí theo ba đơn vị:

```rust
pub struct ViTriVanBan {
    chi_so_byte: usize,
    chi_so_utf16: usize,
    chi_so_grapheme: usize,
}
```

Vị trí được tính từ **byte offset** của con trỏ nội bộ (tổng `len_utf8` của các
ký tự trước con trỏ). `chi_so_byte` luôn là ranh giới UTF-8 vì con trỏ nằm giữa hai
`char`. `chi_so_utf16` và `chi_so_grapheme` tính từ tiền tố
`van_ban[..chi_so_byte]`:

```rust
chi_so_utf16   = tien_to.encode_utf16().count();
chi_so_grapheme = tien_to.graphemes(true).count();
```

Dùng `unicode-segmentation` để tính grapheme cluster (grapheme cluster boundary,
mặc định extended).

## Lý do

* Tính từ byte offset (ranh giới char) đảm bảo `chi_so_byte` luôn hợp lệ để cắt
  chuỗi UTF-8.
* Khi con trỏ nội bộ nằm giữa hai `char` thuộc cùng grapheme (ví dụ giữa `e` và
  combining mark, hoặc giữa `👨` và `Z` của `👨‍👩`), grapheme index "snap" về ranh
  giới cluster gần nhất (đếm cluster tiền tố, coi cluster đang cắt một phần là một
  cluster trọn). Vị trí public không bao giờ báo grapheme index nằm giữa cluster.
* Phương án này đơn giản, đúng, dễ property test.

## Phương án bị loại

* **Chỉ trả `char` index:** bị loại - sai cho emoji ZWJ và combining mark.
* **Snap byte về ranh giới grapheme (làm tròn):** bị loại vì làm mất vị trí char
  thật mà host có thể cần; giữ byte tại ranh giới char chính xác hơn.
* **Tự viết segmenter:** bị loại - dùng `unicode-segmentation` chuẩn, đã no_std.

## Bất biến

* `chi_so_byte` luôn là ranh giới UTF-8 (`is_char_boundary`).
* `chi_so_grapheme` luôn là ranh giới grapheme cluster.
* `chi_so_utf16` và `chi_so_byte` đều không vượt độ dài tương ứng.
* Lịch sử vẫn lưu từng `char`; vị trí public phản ánh text hiển thị.

## Tác động tới Phase sau

* Phase 2: khi Telex biến đổi `noi_dung`, `noi_dung_goc` vẫn giữ raw; con trỏ tính
  trên `noi_dung` (hiển thị) bằng cùng cơ chế.
* Nếu Phase sau cần caret theo "cluster thực" giữa chặp, bổ sung method riêng, không
  phá method hiện tại.
