# Phát hành Cadence

Tài liệu này mô tả quy trình phát hành và gate kiểm tra cho Cadence. Phát hành
= khóa source ở một commit, tạo tag SemVer, tạo package tái lập được. Cadence
**không** tự chạy `cargo publish` trong CI; publish là thao tác thủ công có chủ
đích.

## Version

Cadence dùng SemVer. Trước `1.0.0`:

- `0.1.x`: giữ source compatibility (xem `docs/API_STABILITY.md`).
- `0.2.0`: có thể có breaking change có lý do, ghi CHANGELOG + RFC.
- `1.0.0`: cam kết SemVer đầy đủ.

Version nằm trong `Cargo.toml` (`[package] version`) và `CHANGELOG.md`. Phát
hành `0.1.0` phải có `version = "0.1.0"` (không `-rc`/`-dev`).

## MSRV

Rust 1.85 (xem `docs/MSRV.md`). Tăng MSRV là breaking change. CI chạy matrix
`stable` và `1.85` trên Linux/Windows/macOS (xem `.github/workflows/ci.yml`).

## Gate phát hành (phải xanh trước tag)

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
cargo test --no-default-features --features serde
cargo test --no-default-features --features trace
cargo test --no-default-features --features serde,trace
cargo check --release
cargo check --release --no-default-features
cargo check --release --no-default-features --features serde
cargo check --release --no-default-features --features trace
cargo check --release --no-default-features --features serde,trace
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
cargo +1.85 fmt --check
cargo +1.85 clippy --all-targets --all-features -- -D warnings
cargo +1.85 test --all-features
cargo +1.85 test --no-default-features
cargo +1.85 test --no-default-features --features serde,trace
cargo +1.85 check --release --no-default-features
```

## Source safety

```bash
grep -RIn --exclude-dir=target --exclude-dir=.git "unsafe" src      # chỉ forbid
grep -RIn --exclude-dir=target --exclude-dir=.git -E "unwrap\(\)|unwrap_err\(\)" src
grep -RIn --exclude-dir=target --exclude-dir=.git -E "panic!|todo!|unimplemented!|unreachable!|expect\(" src
grep -RIn --exclude-dir=target --exclude-dir=.git -E "std::fs|std::net|std::thread|Mutex|RwLock|TcpStream|UdpSocket" src
grep -RIn --exclude-dir=target --exclude-dir=.git -E "static mut" src
```

Production `src` không có `unsafe` usage, `unwrap()`, `panic!`, `expect(`,
`unreachable!`, I/O/network/thread/lock, mutable static. (`panic!`/`expect(`
chỉ trong inline `#[cfg(test)]` của `src` — xem `docs/SECURITY_MODEL.md`.)

## Dependency / supply chain

```bash
cargo deny check     # advisories/bans/licenses/sources
cargo audit          # security advisory theo Cargo.lock
cargo tree --duplicates
```

`deny.toml` cấu hình license allowlist + ban advisories + chỉ crates.io. Không
thêm `allow` rộng. Mọi allow phải có comment lý do + điều kiện xóa.

## Package

```bash
cargo package --list   # kiểm danh sách file
cargo package          # verify độc lập trên working tree sạch
```

Package không chứa `target/`, `.git`, soak log, secret, dump. `cargo package`
không dùng `--allow-dirty` trong release gate cuối.

## Soak / benchmark

```bash
cargo test --release --all-features --test soak
cargo bench --all-features --bench xuyet
```

Soak không panic, không invariant failure. Benchmark không regression blocking,
không treo, worst-case trong budget đã tài liệu hóa (µs–ms).

## Tài liệu phát hành

Phải tồn tại và đúng:

- `CHANGELOG.md` có section version với ngày phát hành + hạn chế đã biết.
- `docs/INTEGRATION.md` (contract host).
- `docs/API_STABILITY.md`, `docs/SECURITY_MODEL.md`, `docs/MSRV.md`.
- `docs/TRACE_PRIVACY.md`, `docs/INVARIANTS.md`, `docs/DEPENDENCIES.md`.
- `docs/api/public-api-0.1.0.md` khớp public API thật.
- Báo cáo audit (`docs/RELEASE_CANDIDATE_REPORT.md`).

## Tag

Chỉ tạo tag khi toàn bộ gate xanh và working tree sạch:

```bash
git status --short          # phải rỗng
git tag -a v0.1.0 -m "Cadence 0.1.0"
git push origin main
git push origin v0.1.0
git ls-remote --tags origin refs/tags/v0.1.0   # xác minh remote
```

Không force-push, không rewrite history, không xóa tag. Nếu push tag thất bại,
không xóa tag local, ghi lỗi.

## Không chạy `cargo publish`

Cadence không tự publish. Khi chủ đích publish (thủ công):

```bash
cargo publish --dry-run    # kiểm tra
cargo publish              # chỉ khi chủ đích, không trong audit/CI
```
