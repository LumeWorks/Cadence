# RFC 0001 - Kiến trúc lõi

Trạng thái: Chấp thuận - Phase 1.

## Vấn đề

Cadence phải là một library crate duy nhất, nhúng được vào nhiều môi trường
(LCand, WCand, desktop, mobile, trình soạn thảo, terminal, dự án Rust, binding
FFI) mà không kéo theo phụ thuộc nền tảng. Lõi phải tách bạch khỏi mọi thứ không
phải xử lý nhập liệu.

## Quyết định

Cadence là **một crate duy nhất**, không workspace, không crate con, source đặt
ngay tại `src/`. Kiến trúc phân lớp:

```text
BoGo (factory bất biến)
  → PhienGo (stateful, giữ lịch sử thao tác làm nguồn sự thật)
    → BanChupSoan (snapshot trung lập nền tảng)
```

Lịch sử thao tác là nguồn sự thật duy nhất; snapshot luôn dựng lại từ lịch sử.

## Lý do

* Một crate duy nhất giảm chi phí nhúng và binding.
* Phân lớp factory → phiên → snapshot giữ ranh giới trách nhiệm rõ.
* Lịch sử làm nguồn sự thật tránh bug đồng bộ ngược giữa `String` và state.

## Phương án bị loại

* **Workspace nhiều crate:** bị loại vì Phase 1 chưa cần và tăng độ phức tạp nhúng.
* **Đồng bộ trực tiếp `String`:** bị loại vì dễ sinh trạng thái không nhất quán.
* **Chứa FFI/IPC trong repo này:** bị loại rõ ràng - đó là vai trò của LCand/WCand.

## Bất biến

* Repo chỉ có một crate tại root.
* Không FFI, GUI, IPC, network, thread, async runtime, nhận diện ứng dụng.
* `unsafe` bị forbid; `unwrap()` bị deny.

## Tác động tới Phase sau

* Phase 2 thêm Telex bên trong `PhienGo` mà không đập lại public API.
* Binding FFI build ở repo riêng, dùng public API của crate này.
