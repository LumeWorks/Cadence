# MSRV — Minimum Supported Rust Version

Cadence `0.1.0` đặt MSRV là **Rust 1.85**.

## Vì sao 1.85

- **Edition 2024**: Cadence dùng `edition = "2024"`. Edition 2024 được ổn định
  từ Rust 1.85. Đây là lý do chính — không phải syntax tiện hơn, mà là edition
  chính thức.
- **`let`-chains và `if let`-or**: không dùng cú pháp yêu cầu nightly; chỉ dùng
  cú pháp ổn định trong 1.85.
- **`std::thread::scope`**: ổn định từ 1.63, dùng trong `tests/contract.rs`.
- **`#[cfg_attr]` serde**: ổn định lâu.
- Dependency (`unicode-segmentation 1`, `unicode-normalization 0.1.25`,
  `serde 1`) tương thích 1.85.

## Cách kiểm tra

CI chạy matrix `stable` và `1.85`:

```bash
cargo +1.85 test --all-features
cargo +1.85 test --no-default-features
cargo +1.85 check --release --no-default-features
```

`clippy.toml` đặt `msrv = "1.85"` để clippy không gợi ý lint yêu cầu Rust mới hơn.

## Khi nào được tăng MSRV

Tăng MSRV là **breaking change**:

- Phải ghi rõ trong `CHANGELOG.md` ở mục "Breaking" (hoặc "MSRV").
- Phải có lý do: dependency mới yêu cầu, sửa security, hoặc edition mới ổn định.
- Không tăng chỉ để dùng cú pháp tiện hơn.
- Không âm thầm tăng.

Cadence dự kiến tăng MSRV theo Rust 1.85 nhận support của rustc stable trong
vài năm (Rust có 6 phiên bản hỗ trợ, ~2.5 năm). Tăng sẽ đi cùng minor bump
(`0.2.0` trở lên).

## Dependency mới

Dependency mới phải:
1. Tương thích MSRV 1.85 (kiểm tra `Cargo.toml` của crate đó hoặc test build).
2. Có lý do rõ (xem `docs/DEPENDENCIES.md`).
3. Không âm thầm tăng MSRV của Cadence.

Nếu phải chọn giữa MSRV và security: ưu tiên security, ghi RFC, tăng MSRV có
chủ đích, ghi CHANGELOG. Không pin dependency cũ có advisory chỉ để giữ MSRV
nếu có phương án an toàn hơn.

## Bằng chứng

`docs/PHASE3_BASELINE.md` ghi kết quả `cargo +1.85 test`/`check` xanh. CI
tái lập qua matrix `toolchain: [stable, "1.85"]`.
