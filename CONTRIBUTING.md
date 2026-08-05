# Đóng góp cho Cadence

Cảm ơn bạn quan tâm đến Cadence. Tài liệu này quy ước cách đóng góp.

## Quy ước phong cách

* **Identifier domain tiếng Việt không dấu** (ví dụ `PhienGo`, `them_ky_tu`).
* **Comment viết tiếng Việt có dấu.**
* Public API của Cadence dùng tiếng Việt không dấu.
* Field private; hành vi nằm trong `impl`.
* Mỗi hàm một trách nhiệm; tách điều kiện phức tạp thành method có tên rõ.
* Ưu tiên `match` đầy đủ cho enum, `Option` và các nhánh quan trọng.
* Không wildcard import; mỗi item import bằng dòng `use` riêng.
* Không getter/setter máy móc; chỉ expose method có ý nghĩa.
* Không `unwrap()`. Chỉ `expect()` khi giải thích được invariant.
* Để `rustfmt` quyết định format.
* TODO được phép nhưng phải ghi rõ thiếu gì, vì sao chưa làm và điều kiện xóa.
  Không để TODO che lỗi an toàn hoặc hành vi chưa được test.

## Lệnh kiểm tra

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
cargo check --release
cargo check --release --no-default-features
cargo doc --all-features --no-deps
rustup run 1.85 cargo check --all-features
rustup run 1.85 cargo check --no-default-features
```

Benchmark (không phải gate cứng):

```bash
cargo bench
```

## Quy tắc đóng góp

* **Mỗi bug phải có regression test.** Sửa lỗi thì thêm test chứng minh lỗi đã hết.
* **Không thêm dependency tùy tiện.** Mọi dependency phải có mục đích rõ và được
  ghi trong `docs/README.md` (dependency policy). Phase 1 dùng
  `unicode-segmentation` (runtime) và `proptest`/`criterion` (dev). Phase 2 thêm
  `unicode-normalization` (runtime, no_std compatible).
* **Không thêm `unsafe`.** `unsafe_code` bị `forbid`.
* **Không triển khai các thứ ngoài phạm vi core** (FFI, GUI, IPC, network, thread,
  async runtime, nhận diện ứng dụng) — đó là vai trò của LCand/WCand.
* **Giữ `no_std + alloc` build xanh.** Không gọi filesystem, env var, stdout hay
  API chỉ có trong `std`. Error type phải hoạt động khi tắt `std`; chỉ implement
  `std::error::Error` dưới `#[cfg(feature = "std")]`.

## Tiến trình git

* Commit message ngắn bằng tiếng Việt không dấu.
* Mỗi commit đại diện cho một bước tiến có thể giải thích.
* Không force-push, không amend/squash commit cũ, không sửa lịch sử đã tồn tại.
* Không commit file build, secret hoặc dữ liệu máy cá nhân.

## Báo lỗi

* Mỗi issue một vấn đề, có bước tái hiện và kỳ vọng.
* **Không đưa nội dung gõ thật của người dùng vào issue công khai.**
