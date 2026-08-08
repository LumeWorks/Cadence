# SPDX-License-Identifier: MPL-2.0
# Copyright (c) 2026 Lê Hùng Quang Minh
#
# Pull request template. Điền checklist + mô tả + kiểm tra để merge nhanh.

## Liên kết issue

Closes #<!-- số issue, vd #12 -->

## Mô tả

<!-- Mô tả ngắn gọn thay đổi này giải quyết vấn đề gì. Nếu là fix, nêu nguyên nhân gốc. -->

## Loại thay đổi

<!-- Đánh dấu x mục phù hợp. -->

- [ ] 🐞 Sửa lỗi (tương thích ngược, không phá code)
- [ ] ✨ Tính năng mới (tương thích ngược, có thể cần RFC)
- [ ] 💥 Phá code (API đổi / migration cần thiết)
- [ ] 📚 Tài liệu / RFC
- [ ] ♻️ Refactor (không đổi hành vi)
- [ ] ⚡ Hiệu năng

## Checklist

- [ ] `cargo fmt --check` xanh
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` xanh
- [ ] `cargo clippy --all-targets --no-default-features -- -D warnings` xanh
- [ ] `cargo test --all-features` xanh
- [ ] `cargo test --no-default-features` xanh
- [ ] `cargo test --no-default-features --features serde,trace` xanh
- [ ] `RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps` xanh
- [ ] Test mới cho hành vi mới (nếu có)
- [ ] CHANGELOG.md cập nhật (nếu là phát hành)
- [ ] RFC cập nhật (nếu thay đổi quy tắc/hành vi)

## Ghi chú cho reviewer

<!-- Điểm cần chú ý, quyết định thiết kế, phương án bị loại. -->
