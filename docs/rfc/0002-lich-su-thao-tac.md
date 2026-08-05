# RFC 0002 - Lịch sử thao tác

Trạng thái: Chấp thuận - Phase 1.

## Vấn đề

Phiên soạn thảo cần chỉnh sửa giữa đoạn (chèn, xóa, di chuyển con trỏ) mà vẫn giữ
nguồn sự thật duy nhất và khả năng khôi phục nguyên bản. Nếu render là nguồn sự
thật, việc đồng bộ ngược vào lịch sử sẽ sinh bug.

## Quyết định

Lịch sử thao tác là nguồn sự thật:

```rust
pub(crate) struct ThaoTacNhap {
    ky_tu: char,
    cach_nhap: CachNhap,
}

pub(crate) enum CachNhap {
    TuDong,
    NguyenBan,
}
```

Con trỏ nội bộ nằm **giữa các thao tác** (`0..=lich_su.len()`):

```text
[a, b, |, c, d]
```

Pipeline replay Phase 1:

```text
lịch sử → render nguyên bản → tính vị trí con trỏ → cập nhật snapshot
```

Replay toàn đoạn sau mỗi thay đổi là thiết kế chính thức Phase 1 (không cache
incremental).

## Lý do

* Mỗi thao tác là một đơn vị rõ; chỉnh sửa giữa đoạn chỉ là `insert`/`remove` trên
  `Vec`.
* Render lại toàn bộ sau mỗi thay đổi đơn giản, đúng, dễ kiểm chứng bằng property
  test.
* `cach_nhap` tách `TuDong`/`NguyenBan` để Phase 2 quyết định Telex mà không phá
  raw input.

## Phương án bị loại

* **Gap buffer / piece table:** bị loại cho Phase 1 - overkill khi độ dài token
  bị giới hạn bởi `gioi_han_thao_tac` (≤ 4096) và replay O(n) đã đủ.
* **Cache incremental:** bị loại vì tăng phức tạp và rủi ro sai lệch; benchmark
  cho thấy replay 128 thao tác vẫn cỡ µs, chấp nhận được.
* **Lưu `String` thay vì `Vec<ThaoTacNhap>`:** bị loại vì mất phân biệt
  `cach_nhap` và khó khôi phục nguyên bản.

## Bất biến

* `noi_dung_goc` của snapshot luôn bằng đúng chuỗi `ky_tu` của các thao tác hiện
  còn.
* Con trỏ luôn nằm trong `0..=lich_su.len()`.
* Thao tác không expose `Vec` ra public API.

## Tác động tới Phase sau

* Phase 2 duyệt lịch sử để áp dụng Telex; `cach_nhap == NguyenBan` bỏ qua biến đổi.
* Nếu Phase sau cần undo nhiều cấp, lịch sử thao tác là cơ sở tự nhiên.
