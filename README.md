# Cadence

Cadence - Gõ mọi thứ bạn cần.

Cadence là lõi gõ tiếng Việt thế hệ mới viết bằng Rust, kiến trúc hiện đại và an toàn, có thể nhúng vào nhiều môi trường (Linux, Windows, mobile, trình soạn thảo, công cụ terminal, dự án Rust khác, binding FFI).

## Trạng thái 2026.1.0

Cadence hiện có Telex **và** VNI engine đầy đủ, phân đoạn ngữ cảnh,
biến đổi hình chữ (â, ă, ê, ô, ơ, ư, đ), dấu thanh (sắc, huyền, hỏi, ngã,
nặng), escape (lặp phím/digit modifier), phân tích âm tiết để lựa chọn
raw/biến đổi, và Unicode NFC/NFD output. Lịch sử thao tác raw là nguồn sự
thật; `noi_dung_goc()` trả byte-for-byte raw.

Cadence giữ nguyên code/chat: `sha256`, `h264`, `v1.2.3`, `127.0.0.1`,
`user123`, `x86_64` không bị biến đổi. Tiếng Việt hợp lệ được biến đổi.
Không cần bật/tắt bộ gõ khi chuyển context trong cùng phiên.

Xem [`docs/VERSIONING.md`](docs/VERSIONING.md) cho hệ phiên bản
calendar/change/patch và RFC 0020–0024 cho chi tiết VNI.

## Sử dụng

```toml
[dependencies]
cadence = { package = "cadence-ime", version = "2026.1" }
```

```rust
use cadence::{BoGo, CauHinh, KieuGo, KetQuaXuLy};

// Telex (mặc định)
let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh luon hop le");
let mut phien = bo_go.tao_phien();
for c in "tieengs".chars() {
    phien.them_ky_tu(c);
}
assert_eq!(phien.ban_chup().noi_dung(), "tiếng");

// VNI
let mut c = CauHinh::mac_dinh();
c.dat_kieu_go(KieuGo::Vni);
let bo_vni = BoGo::new(c).expect("hop le");
let mut phien_vni = bo_vni.tao_phien();
for c in "tieng61".chars() {
    phien_vni.them_ky_tu(c);
}
assert_eq!(phien_vni.ban_chup().noi_dung(), "tiếng");
```

## Phạm vi của core

Cadence chỉ là lõi xử lý nhập liệu thuần Rust.

### Những thứ thuộc Cadence

* Quản lý phiên soạn thảo.
* Lịch sử thao tác không phá hủy.
* Telex + VNI engine: hình chữ, dấu thanh, escape.
* Phân tích âm tiết và lựa chọn raw/biến đổi.
* Phân đoạn và nhận diện ngữ cảnh kỹ thuật.
* Chính sách lựa chọn `TuNhien`/`UuTienTiengViet`/`UuTienNguyenBan`.
* Trace quyết định có cấu trúc (feature `trace`).
* Snapshot văn bản trung lập nền tảng.
* Vị trí con trỏ theo byte, UTF-16 và grapheme.
* Unicode NFC/NFD output.
* Hạn chế kích thước phiên để bảo vệ ứng dụng host.

### Những thứ KHÔNG thuộc Cadence

* Fcitx5, IBus, Wayland.
* Windows TSF, keyboard hook.
* FFI C/C++, GUI, CLI riêng, IPC, D-Bus.
* Async runtime, thread nền, network.
* Logic nhận diện ứng dụng.

Đó là vai trò của CadenceRuntime - repository runtime riêng.

## Tính năng

| Feature | Mặc định | Mô tả |
|---|---|---|
| `std` | có | Dùng thư viện chuẩn. |
| `no_std + alloc` | - | Biên dịch cho môi trường không có `std`. |
| `serde` | - | Derive serde cho một số public data type. |
| `trace` | - | Trace quyết định raw/Telex có cấu trúc qua `PhienGo::trace()`. |

## MSRV

Rust 1.85.

## Giấy phép

MPL-2.0. Xem [`LICENSE`](./LICENSE) và [`NOTICE`](./NOTICE).

## Credit

Copyright (c) 2026 Lê Hùng Quang Minh.

## Lệnh kiểm tra cho contributor

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
cargo test --features serde
cargo test --features trace
cargo check --release
cargo check --release --no-default-features
cargo check --release --no-default-features --features serde,trace
cargo doc --all-features --no-deps
```
