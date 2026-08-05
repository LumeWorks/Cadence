# Cadence

Cadence — Gõ mọi thứ bạn cần.

Cadence là lõi gõ tiếng Việt thế hệ mới viết bằng Rust, kiến trúc hiện đại và an toàn, có thể nhúng vào nhiều môi trường (Linux, Windows, mobile, trình soạn thảo, công cụ terminal, dự án Rust khác, binding FFI).

## Trạng thái Phase 2

Cadence hiện có Telex engine đầy đủ: biến đổi hình chữ (â, ă, ê, ô, ơ, ư, đ), dấu
thanh (sắc, huyền, hỏi, ngã, nặng), escape (lặp phím modifier), phân tích âm tiết
để lựa chọn raw/Telex, và Unicode NFC/NFD output. Lịch sử thao tác raw là nguồn
sự thật; `noi_dung_goc()` trả byte-for-byte raw.

## Phạm vi của core

Cadence chỉ là lõi xử lý nhập liệu thuần Rust.

### Những thứ thuộc Cadence

* Quản lý phiên soạn thảo.
* Lịch sử thao tác không phá hủy.
* Telex engine: hình chữ, dấu thanh, escape.
* Phân tích âm tiết và lựa chọn raw/Telex.
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

Đó là vai trò của LCand (Linux) và WCand (Windows) — ba repository độc lập.

## Sử dụng

```toml
[dependencies]
cadence = { package = "cadence-ime", version = "0.1" }
```

```rust
use cadence::{BoGo, CauHinh, KetQuaXuLy};

let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("cau hinh mac dinh luon hop le");
let mut phien = bo_go.tao_phien();

// Telex: gõ "tieengs" → "tiếng"
for c in "tieengs".chars() {
    phien.them_ky_tu(c);
}
assert_eq!(phien.ban_chup().noi_dung(), "tiếng");

// Raw history là nguồn sự thật
assert_eq!(phien.ban_chup().noi_dung_goc(), "tieengs");

// Commit
if let KetQuaXuLy::ChapNhan { noi_dung } = phien.chap_nhan() {
    assert_eq!(noi_dung, "tiếng");
}
```

## Tính năng

| Feature | Mặc định | Mô tả |
|---|---|---|
| `std` | có | Dùng thư viện chuẩn. |
| `no_std + alloc` | — | Biên dịch cho môi trường không có `std`. |
| `serde` | — | Derive serde cho một số public data type. |
| `trace` | — | Dành cho Phase sau, hiện chưa có hành vi. |

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
cargo test --all-features
cargo test --no-default-features
cargo check --release
cargo check --release --no-default-features
cargo doc --all-features --no-deps
```
