# Cadence

Cadence - Gõ mọi thứ bạn cần.

Cadence là lõi gõ tiếng Việt thế hệ mới viết bằng Rust, kiến trúc hiện đại và an toàn, có thể nhúng vào nhiều môi trường (Linux, Windows, mobile, trình soạn thảo, công cụ terminal, dự án Rust khác, binding FFI).

## Trạng thái 0.1.0

Cadence hiện có Telex engine đầy đủ **và** phân đoạn ngữ cảnh (Phase 3),
đã được ổn định và kiểm tra cho phát hành `0.1.0` (Phase 4):
biến đổi hình chữ (â, ă, ê, ô, ơ, ư, đ), dấu thanh (sắc, huyền, hỏi, ngã,
nặng), escape (lặp phím modifier), phân tích âm tiết để lựa chọn raw/Telex,
và Unicode NFC/NFD output. Lịch sử thao tác raw là nguồn sự thật;
`noi_dung_goc()` trả byte-for-byte raw.

Phase 3 thêm triết lý "Gõ mọi thứ bạn cần": lịch sử được chia thành đoạn theo
loại ký tự, mỗi đoạn quyết định Telex hay raw độc lập. Code, URL, email, đường
dẫn, namespace `::`, phép gán `=`, emoticon, teencode lặp được nhận diện và
giữ nguyên bản; tiếng Việt hợp lệ được biến đổi. Không cần bật/tắt bộ gõ khi
chuyển context trong cùng phiên.

Phase 4 ổn định API, thêm tài liệu bảo mật/MSRV/bất biến, rule matrix tests,
editing/Unicode matrix, property/serde tests, soak tests, và sửa một bug
cursor. 655 tests across all feature combinations.

Xem [`docs/rfc/0013-triet-ly-go-moi-thu.md`](docs/rfc/0013-triet-ly-go-moi-thu.md)
cho triết lý đầy đủ và RFC 0014–0019 cho chi tiết từng phần.

## Phạm vi của core

Cadence chỉ là lõi xử lý nhập liệu thuần Rust.

### Những thứ thuộc Cadence

* Quản lý phiên soạn thảo.
* Lịch sử thao tác không phá hủy.
* Telex engine: hình chữ, dấu thanh, escape.
* Phân tích âm tiết và lựa chọn raw/Telex.
* Phân đoạn và nhận diện ngữ cảnh kỹ thuật (Phase 3).
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

Đó là vai trò của LCand (Linux) và WCand (Windows) - ba repository độc lập.

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
