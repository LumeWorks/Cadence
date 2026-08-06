# Phát hành tự động Cadence

Tài liệu này mô tả chi tiết workflow phát hành tự động trong
`.github/workflows/release.yml`.

## Tổng quan

Release workflow là pipeline 3 job: `prepare` → `publish-crate` (tùy chọn) →
`github-release`. Workflow được kích hoạt bởi:

1. **Push tag** `v*.*.*` — phát hành bình thường.
2. **`workflow_dispatch`** — backfill tag đã tồn tại, với input `tag` và
   `publish_crate` (mặc định `true`).

### Version scheme

Tag phải khớp regex `^v[0-9]{4}\.[1-9][0-9]*\.[0-9]+$` (calendar/change/patch).
Tag không hợp lệ bị từ chối ở bước đầu tiên.

### Concurrency

Mỗi tag chỉ chạy một workflow tại một thời gian (`release-<tag>`), không hủy
workflow đang chạy.

## Job 1: `prepare`

Kiểm tra toàn bộ gate và đóng gói crate.

### Bước

1. **Resolve tag** — xác định tag từ `github.ref_name` (push) hoặc `inputs.tag`
   (dispatch), validate regex, trích version.

2. **Checkout tag** — `actions/checkout` với `ref: refs/tags/<tag>`,
   `fetch-depth: 0`. Checkout commit chính xác của tag, không phải `main`.

3. **Install Rust toolchain** — `dtolnay/rust-toolchain` với `rustfmt`, `clippy`.

4. **Validate tag và package metadata**:
   - Tag tồn tại local.
   - Tag là annotated (objecttype `tag`, không phải `commit`).
   - Working tree sạch sau checkout.
   - `Cargo.toml` version khớp tag.
   - Package name là `cadence-ime`.
   - Lib target là `cadence`.
   - Repository URL là `https://github.com/LumeWorks/Cadence`.

5. **Gate**:
   - `cargo fmt --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo clippy --all-targets --no-default-features -- -D warnings`
   - `cargo test --all-features`
   - `cargo test --no-default-features`
   - `cargo test --no-default-features --features serde,trace`
   - `cargo check --release --no-default-features`
   - `cargo doc --all-features --no-deps` (`RUSTDOCFLAGS=-D warnings`)

6. **Package** — `cargo package --locked`.

7. **Verify package**:
   - File `.crate` tồn tại.
   - VCS SHA trong `.cargo_vcs_info.json` khớp tag commit SHA.
   - Không có file cấm (`.git`, `target`, `*.secret`, `*.token`, `*.pem`,
     `*.key`, `*.log`).

8. **Create checksum** — `sha256sum` file `.crate`.

9. **Upload artifact** — `actions/upload-artifact` với file `.crate` + `.sha256`,
   giữ 14 ngày.

### Outputs

- `tag` — tag name (vd `v2026.1.0`)
- `version` — version không tiền tố `v` (vd `2026.1.0`)
- `tag_commit` — SHA của commit mà tag trỏ tới
- `crate_file` — đường dẫn file `.crate`
- `checksum_file` — đường dẫn file `.sha256`

## Job 2: `publish-crate`

Publish crate lên crates.io. Chạy khi:
- Push tag: luôn (trừ khi bị skip do `prepare` fail).
- `workflow_dispatch`: khi `inputs.publish_crate == true`.

### Environment

Yêu cầu `environment: release` với `id-token: write` (cho Trusted Publishing
OIDC).

### Bước

1. **Checkout tag** — giống `prepare`.

2. **Install Rust toolchain** — `dtolnay/rust-toolchain` (không cần components).

3. **Check if version already published** — `cargo info cadence-ime@<version>`.
   Nếu đã publish, skip các bước publish, tiếp tục job tiếp theo.

4. **Check if crate exists on crates.io** — `curl` API `crates.io/api/v1/crates/cadence-ime`.

5. **Publish**:
   - **Lần đầu** (crate chưa tồn tại): dùng `secrets.CRATES_IO_TOKEN` qua
     `CARGO_REGISTRY_TOKEN`. Báo lỗi nếu secret chưa cấu hình.
   - **Các lần sau** (crate đã tồn tại): dùng Trusted Publishing qua
     `rust-lang/crates-io-auth-action` (OIDC, không cần long-lived token).

6. **Verify registry** — poll `cargo info` tối đa 12 lần (120s) để xác nhận
   version đã visible trên crates.io.

### Idempotent

Nếu version đã publish, workflow skip publish và tiếp tục tạo GitHub Release.
Điều này cho phép rerun an toàn.

## Job 3: `github-release`

Tạo hoặc cập nhật GitHub Release. Chạy khi `prepare` thành công và
`publish-crate` thành công hoặc bị skip.

### Permissions

`contents: write` — cần để tạo/edit release và upload assets.

### Bước

1. **Checkout tag** — giống `prepare`.

2. **Download artifact** — `actions/download-artifact` lấy file `.crate` +
   `.sha256` từ job `prepare`.

3. **Extract release notes** — script Python trích section `## [<version>]`
   từ `CHANGELOG.md` vào `release-notes.md`.

4. **Create or update GitHub Release**:
   - Nếu release đã tồn tại: `gh release upload --clobber` + `gh release edit`.
   - Nếu chưa: `gh release create --verify-tag --latest`.
   - Tựa: `Cadence <version>`.
   - Assets: file `.crate` + file `.sha256`.

## Backfill

Backfill = chạy lại release workflow cho tag đã tồn tại. Trường hợp sử dụng:

- Workflow ban đầu thất bại.
- Cần cập nhật release notes hoặc assets.
- Cần tạo GitHub Release cho tag cũ chưa có release.

```bash
gh workflow run release.yml \
  --ref main \
  -f tag=v2026.1.0 \
  -f publish_crate=false
```

Quy tắc backfill:
- `--ref main` — workflow chạy từ branch `main` (mã workflow mới nhất).
- `-f tag=v2026.1.0` — tag phải đã tồn tại. Workflow checkout tag đó, không
  tạo tag mới.
- `-f publish_crate=false` — bỏ qua crates.io publish nếu version đã publish.
  Đặt `true` nếu cần publish (workflow idempotent, sẽ skip nếu đã publish).

## Bảo mật

### Pinned Actions

Tất cả GitHub Actions được pin về full commit SHA đã xác minh:

| Action | SHA | Version |
|---|---|---|
| `actions/checkout` | `11bd71901bbe5b1630ceea73d27597364c9af683` | v4.2.2 |
| `actions/upload-artifact` | `ea165f8d65b6e75b540449e92b4886f43607fa02` | v4.6.2 |
| `actions/download-artifact` | `d3f86a106a0bac45b974a628896c90dbdf5c8093` | v4.3.0 |
| `dtolnay/rust-toolchain` | `e97e2d8cc328f1b50210efc529dca0028893a2d9` | v1 |
| `rust-lang/crates-io-auth-action` | `c6f97d42243bad5fab37ca0427f495c86d5b1a18` | v1.0.5 |

Không dùng `@v*` tag — chỉ full SHA.

### Không dùng trigger nguy hiểm

Workflow không dùng `pull_request_target` hay `workflow_run` — hai trigger này
chạy với secret của base branch và có thể bị lợi dụng bởi PR từ fork.

### Token

- GitHub Release dùng `github.token` (tự động, scope `contents: write`).
- crates.io lần đầu: `secrets.CRATES_IO_TOKEN` (cấu hình trong repo Settings →
  Secrets and variables → Actions).
- crates.io các lần sau: OIDC Trusted Publishing (không cần secret).

### Checkout

Workflow luôn checkout exact tag commit, không bao giờ checkout `main` để
phát hành. Điều này đảm bảo package tái lập được.

## Cấu hình cần thiết trước lần đầu publish

1. **`CRATES_IO_TOKEN` secret**: Tạo token tại https://crates.io/settings/tokens
   với scope `publish-update`. Thêm vào repo Settings → Secrets and variables
   → Actions → New repository secret, tên `CRATES_IO_TOKEN`.

2. **`release` environment** (tùy chọn): Tạo environment `release` trong repo
   Settings → Environments. Có thể thêm protection rules (required reviewers,
   deployment branches). Job `publish-crate` yêu cầu environment này.

3. **Trusted Publishing** (cho các lần sau): Cấu hình tại
   https://crates.io/settings/publishing → thêm repository
   `LumeWorks/Cadence` với workflow `release.yml` và environment `release`.
   Chỉ cần sau lần publish đầu tiên.

## Sự cố thường gặp

### "tag does not match v<year>.<change>.<patch>"

Tag không khớp regex calendar/change/patch. Kiểm tra tag format.

### "lightweight tag, not an annotated tag"

Tạo tag lại với `git tag -a` (không `git tag` đơn thuần).

### "package version does not match tag version"

Sửa `version` trong `Cargo.toml` cho khớp tag, commit, và tạo tag lại (chỉ
nếu tag chưa push — nếu đã push, không di chuyển tag, mà tạo tag mới với version
đúng).

### "VCS SHA does not match tag commit"

`cargo package` được chạy từ working tree không khớp tag commit. Đảm bảo
workflow checkout đúng tag và working tree sạch.

### "CRATES_IO_TOKEN not configured"

Lần đầu publish cần secret. Cấu hình `CRATES_IO_TOKEN` trong repo settings, rồi
rerun workflow.

### "Version already published"

Version đã trên crates.io. Workflow skip publish (idempotent). Nếu cần tạo
GitHub Release, dùng backfill với `publish_crate=false`.
