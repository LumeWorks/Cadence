# RFC 0005 - Giới hạn phiên

Trạng thái: Chấp thuận - Phase 1.

## Vấn đề

Host application cần bảo vệ khỏi token vô hạn (người dùng giữ phím, lỗi tích lũy).
Cadence không được panic hay trả lỗi generic khi vượt giới hạn.

## Quyết định

`CauHinh` giữ `gioi_han_thao_tac` với:

* Mặc định: 128 thao tác.
* Phạm vi hợp lệ: `1..=4096`.
* Validation qua `dat_gioi_han_thao_tac`, trả `LoiCauHinh::GioiHanThaoTacKhongHopLe`.

Hành vi khi phiên đã đạt giới hạn:

* `them_ky_tu` / `them_nguyen_ban` trả `KetQuaXuLy::KhongDoi`.
* Snapshot không thay đổi.
* Không mất hay sửa state cũ.
* Không tự động xóa đầu buffer.

## Lý do

* Trả `KhongDoi` giữ API đơn giản (không cần `Result` cho thao tác thường), và rõ
  nghĩa: "không có gì đổi" thay vì lỗi.
* Giữ state cũ bảo toàn nội dung người dùng đã gõ; host có thể quyết định commit
  rồi mở phiên mới.
* Không tự xóa đầu buffer vì sẽ âm thầm mất nội dung - nguy hiểm cho thư viện nhúng.

## Phương án bị loại

* **Panic khi vượt giới hạn:** bị loại - thư viện nhúng không được panic vì input.
* **Tự động xóa đầu (ring buffer):** bị loại - mất dữ liệu âm thầm, khó debug.
* **Trả `Result` lỗi cho `them_ky_tu`:** bị loại - vượt giới hạn không phải lỗi
  lập trình, là trạng thái; `KhongDoi` hợp lý hơn.
* **Giới hạn theo byte thay vì thao tác:** bị loại - thao tác ổn định, dễ suy luận
  cho host; byte phụ thuộc encoding.

## Bất biến

* `dat_gioi_han_thao_tac` chỉ thay đổi giá trị khi nhập hợp lệ; lỗi giữ nguyên giá
  trị cũ.
* Số thao tác trong lịch sử không vượt `gioi_han_thao_tac` sau bất kỳ chuỗi thao tác
  nào.
* Sau khi xóa, có thể thêm lại cho đến khi chạm giới hạn.

## Tác động tới Phase sau

* Phase 2 có thể tách giới hạn riêng cho "thao tác hiển thị" vs "thao tác raw" nếu
  cần, nhưng mặc định vẫn dùng `gioi_han_thao_tac`.
* Nếu Phase sau cho phép commit tự động khi đầy, phải làm rõ để không phá bất biến
  "không tự xóa".
