# Phát hành Cadence

Tài liệu này mô tả quy trình phát hành tự động và gate kiểm tra cho Cadence.

Phát hành được tự động hóa qua `.github/workflows/release.yml`. Workflow
chạy toàn bộ gate, đóng gói, tạo checksum, publish lên crates.io (tùy chọn),
và tạo GitHub Release với release notes trích từ `CHANGELOG.md`.

## Version

Cadence dùng hệ calendar/change/patch (xem `docs/VERSIONING.md`):

```
<năm>.<số phiên bản thay đổi>.<số phiên bản vá>
```

Ví dụ: `2026.1.0`, `2026.1.1`, `2026.2.0`.

Version nằm trong `Cargo.toml` (`[package] version`) và `CHANGELOG.md`. Tag
phải khớp `v<version>` (vd `v2026.1.0`).

## MSRV

Rust 1.85 (xem `docs/MSRV.md`). Tăng MSRV là breaking change (tăng thành phần
thứ hai). CI chạy matrix `stable` và `1.85` trên Linux/Windows/macOS (xem
`.github/workflows/ci.yml`).

## Gate phát hành (chạy tự động trong workflow)

Release workflow chạy các gate sau trong job `prepare`:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
cargo test --no-default-features --features serde,trace
cargo check --release --no-default-features
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
```

Ngoài ra workflow kiểm tra:
- Tag là annotated tag (không phải lightweight).
- Version trong `Cargo.toml` khớp tag.
- Package name là `cadence-ime`, lib target là `cadence`.
- Repository URL là `https://github.com/LumeWorks/Cadence`.
- VCS SHA trong `.cargo_vcs_info.json` khớp commit của tag.
- Package không chứa file cấm (`.git`, `target`, `*.secret`, `*.token`, `*.pem`, `*.key`, `*.log`).

## Source safety (chạy thủ công trước tag)

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

## Dependency / supply chain (chạy thủ công trước tag)

```bash
cargo deny check     # advisories/bans/licenses/sources
cargo audit          # security advisory theo Cargo.lock
cargo tree --duplicates
```

`deny.toml` cấu hình license allowlist + ban advisories + chỉ crates.io. Không
thêm `allow` rộng. Mọi allow phải có comment lý do + điều kiện xóa.

## Package (chạy tự động trong workflow)

```bash
cargo package --locked
```

Workflow tạo file `.crate`, tính SHA-256 checksum, và upload làm artifact.

## Tài liệu phát hành

Phải tồn tại và đúng trước tag:

- `CHANGELOG.md` có section `## [<version>] - <date>` với hạn chế đã biết.
- `docs/VERSIONING.md`, `docs/MSRV.md`, `docs/SECURITY_MODEL.md`.
- `docs/API_STABILITY.md`, `docs/TRACE_PRIVACY.md`, `docs/INVARIANTS.md`.
- `docs/DEPENDENCIES.md`, `docs/INTEGRATION.md`.
- `docs/api/public-api-<version>.md` khớp public API thật.

Release notes được trích tự động từ `CHANGELOG.md` — section giữa `## [<version>]`
và `##` tiếp theo.

## Tạo tag (thủ công, duy nhất một lần)

Khi toàn bộ gate xanh và working tree sạch:

```bash
git status --short          # phải rỗng
git tag -a v2026.1.0 -m "Cadence 2026.1.0"
git push origin main
git push origin v2026.1.0
git ls-remote --tags origin refs/tags/v2026.1.0   # xác minh remote
```

Push tag kích hoạt release workflow tự động.

Quy tắc:
- Chỉ tạo annotated tag (`-a`), không lightweight tag.
- Không force-push, không rewrite history, không xóa tag.
- Tag `v0.1.0` (mốc nội bộ) không bao giờ di chuyển hoặc xóa.
- Nếu push tag thất bại, không xóa tag local, ghi lỗi.

## Backfill (phát hành lại tag đã tồn tại)

Khi cần chạy lại release workflow cho tag đã tồn tại (vd workflow cũ thất bại,
cần cập nhật release notes hoặc asset):

```bash
gh workflow run release.yml \
  --ref main \
  -f tag=v2026.1.0 \
  -f publish_crate=false
```

Workflow dùng `workflow_dispatch` với input `tag` và `publish_crate` (mặc định
`true`). Đặt `publish_crate=false` nếu version đã publish trên crates.io để bỏ
qua job `publish-crate` mà vẫn tạo/cập nhật GitHub Release.

## crates.io publish

Workflow tự động publish lên crates.io (job `publish-crate`):

- **Lần đầu** (`cadence-ime` chưa tồn tại): cần secret `CRATES_IO_TOKEN` (token
  từ https://crates.io/settings/tokens với scope `publish-update`). Workflow sẽ
  báo lỗi nếu secret chưa cấu hình.
- **Các lần sau** (crate đã tồn tại): dùng Trusted Publishing qua
  `rust-lang/crates-io-auth-action` (OIDC, không cần long-lived token).
- **Idempotent**: nếu version đã publish, workflow bỏ qua và tiếp tục tạo
  GitHub Release.

Chi tiết đầy đủ: `docs/AUTOMATED_RELEASES.md`.

## GitHub Release

Job `github-release` tạo hoặc cập nhật GitHub Release:

- Tựa: `Cadence <version>`.
- Release notes: trích từ `CHANGELOG.md`.
- Assets: file `.crate` + file `.sha256`.
- `--latest` đánh dấu release mới nhất.
- Idempotent: nếu release đã tồn tại, cập nhật assets và notes (`--clobber`).
