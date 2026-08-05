# Bảo mật Cadence

## Phạm vi bảo mật

Cadence là lõi xử lý nhập liệu thuần Rust. Nó **không xử lý dữ liệu tin cậy** mặc
định: không network, không filesystem, không IPC, không thread nền, không async
runtime. Toàn bộ state nằm trong `PhienGo` do caller sở hữu.

## Giới hạn token

`CauHinh.gioi_han_thao_tac` (mặc định 128, tối đa 4096) bảo vệ host application
khỏi token vô hạn. Khi đạt giới hạn, thao tác thêm trả `KetQuaXuLy::KhongDoi` và
không sửa state. Host không cần tự phòng thủ tràn bộ nhớ do token dài.

## An toàn bộ nhớ

* `unsafe_code` bị `forbid` toàn crate.
* Không `unsafe`, không FFI trong repo này.
* `unwrap()` bị deny; chỉ dùng `expect()` khi giải thích được invariant.

## Không log raw input

Cadence **không log raw input** trong core. Không có logging framework, không
stdout, không trace nội dung người dùng gõ. Lịch sử thao tác tồn tại trong bộ nhớ
phiên và bị xóa khi `dat_lai`/`chap_nhan`.

## Báo lỗi bảo mật

* Mở issue riêng và mô tả tác động. **Không dán nội dung gõ thật của người dùng
  vào issue công khai** - dùng chuỗi mẫu (ASCII, ký tự Unicode công khai).
* Nếu báo lỗ hổng bộ nhớ, ghi rõ phiên bản Rust, feature flags và chuỗi tái hiện.

## Rò rỉ state

Mỗi `PhienGo` độc lập; `chap_nhan` và `dat_lai` xóa toàn bộ state. Nếu phát hiện
state cũ rò sang token mới, đó là bug nghiêm trọng - vui lòng báo kèm regression
test.
