# AGENTS.md

Tài liệu này quy định cách mọi AI agent (Claude Code, ZCode, Cursor, Copilot,
windsurf, ...) làm việc trong repository Cadence. Đọc file này **trước khi**
bắt đầu bất kỳ tác vụ nào. Tuân thủ nghiêm ngặt.

Cadence là lõi gõ tiếng Việt viết bằng Rust (Telex + VNI), no_std + alloc,
MPL-2.0. Xem `README.md`, `CONTRIBUTING.md`, `docs/` cho bối cảnh đầy đủ.

---

## Nguyên tắc tối cao

1. **Ngôn ngữ**: Identifier domain **tiếng Việt không dấu** (vd `PhienGo`,
   `them_ky_tu`). Comment, doc, commit message, issue, PR **tiếng Việt có dấu**.
   Không trộn tiếng Anh trừ thuật ngữ kỹ thuật (RFC, API, CI, NFC/NFD).
2. **An toàn trước**: `unsafe_code` bị `forbid`. Không `unwrap()` trong production
   code. Chỉ `expect()` trong inline `#[cfg(test)]` khi giải thích được invariant.
3. **no_std + alloc**: Không gọi filesystem, env var, stdout, network, thread,
   hoặc API chỉ có trong `std`. Error type phải hoạt động khi tắt `std`.
4. **Raw là nguồn sự thật**: `noi_dung_goc()` trả byte-for-byte raw. Không normalize
   raw, không đổi hoa/thường, không mất combining mark. Xem `docs/INVARIANTS.md`.

---

## Quy trình làm việc (Issues → PR → Project)

Branch `main` được bảo vệ. **Không bao giờ commit thẳng main.**

### Luồng bắt buộc

1. **Tạo issue** mô tả vấn đề/tính năng (dùng issue template 🐞 Báo lỗi /
   ✨ Đề xuất tính năng). Gán label + đưa vào Project board (Kanban).
2. **Tạo branch** từ `main`: `fix/<ten>`, `feature/<ten>`, `release/<version>`,
   `setup/<ten>`, `docs/<ten>`. Tên tiếng Việt không dấu, gạch chéo.
3. **Viết code + test** trên branch. Mỗi commit một bước tiến có thể giải thích.
4. **Chạy toàn bộ gate** (xem mục "Gate kiểm tra" bên dưới) trước khi mở PR.
5. **Mở PR** liên kết issue (`Closes #<so>`), điền checklist PR template.
6. **CI chạy 9 checks** (rustfmt, clippy, test 6 matrix, rustdoc) — phải xanh.
7. **Review**: cần 1 approval. Admin có thể bypass khi gấp (nhưng nên review).
8. **Merge**: dùng **merge commit** (giữ lịch sử branch + PR). Branch tự xóa.
9. **Kéo issue** qua cột Project board: Tồn đọng → Cần làm → Đang làm →
   Đang review → Xong.

### Tên branch hợp lệ

```
fix/linh-hoat-thu-tu
feature/bo-loc-ung-dung
release/2026.1.2
setup/workflow
docs/rfc-0025
```

---

## Gate kiểm tra (chạy trước mỗi PR)

Tất cả phải xanh trước khi mở PR. CI sẽ chạy lại các gate này.

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
cargo test --no-default-features --features serde,trace
cargo check --release --no-default-features
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
rustup run 1.85 cargo check --all-features        # MSRV check
```

Ngoài ra (thủ công, không phải gate cứng):

```bash
grep -RIn --exclude-dir=target --exclude-dir=.git "unsafe" src           # chỉ forbid
grep -RIn --exclude-dir=target --exclude-dir=.git -E "unwrap\(\)" src     # test only
grep -RIn --exclude-dir=target --exclude-dir=.git -E "panic!|todo!|unimplemented!|unreachable!" src
```

Production `src` không có `unsafe` usage, `unwrap()`, `panic!`, `expect(`,
`unreachable!`, I/O/network/thread/lock, mutable static. (`panic!`/`expect(`
chỉ trong inline `#[cfg(test)]` — xem `docs/SECURITY_MODEL.md`.)

---

## Phong cách code

* Identifier domain **tiếng Việt không dấu**: `PhienGo`, `BoGo`, `them_ky_tu`,
  `dat_kieu_go`, `CauHinh`, `KetQuaXuLy`.
* Comment + doc **tiếng Việt có dấu**: `// Áp dụng dấu thanh lên nguyên âm chính.`
* Public API tiếng Việt không dấu. Field private; hành vi nằm trong `impl`.
* Mỗi hàm một trách nhiệm; tách điều kiện phức tạp thành method có tên rõ.
* Ưu tiên `match` đầy đủ cho enum, `Option` và nhánh quan trọng. Không wildcard
  import (`enum_glob_use`, `wildcard_imports` bị deny).
* Không getter/setter máy móc; chỉ expose method có ý nghĩa.
* Để `rustfmt` quyết định format. Chạy `cargo fmt` trước commit.
* TODO được phép nhưng phải ghi rõ thiếu gì, vì sao chưa làm, điều kiện xóa.
  Không để TODO che lỗi an toàn hoặc hành vi chưa test.

### Quy ước license header

Mỗi source file bắt đầu bằng:

```rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh
```

---

## Kiến trúc (tóm tắt)

```
src/
  lib.rs              — module declarations + public re-exports
  bo_go.rs            — BoGo: factory bất biến tạo PhienGo
  phien_go.rs         — PhienGo: session stateful (lịch sử raw + rebuild)
  anh_xa.rs           — xay_lai: pipeline rebuild từ raw history
  phan_doan.rs        — phân đoạn lịch sử theo LoaiDoan
  ngu_canh.rs         — nhận diện ngữ cảnh (URL, code, technical strings)
  cau_hinh.rs          — CauHinh: config (KieuGo, KieuTelex, QuyTacDatDau, ...)
  kieu_go/
    telex.rs          — engine Telex (hình chữ, dấu thanh, escape)
    vni.rs            — engine VNI (digit modifier 1-9)
    bo_dat_dau.rs     — tim_nguyen_am_chinh: nguyên âm chính (dùng chung)
    am_tiet.rs        — phan_tich_am_tiet: parser âm tiết (onset/coda)
    chu_viet.rs       — ChuCaiViet: domain model (chữ gốc + dấu + hoa)
    render.rs         — Unicode NFC/NFD render
    don_vi.rs         — DonViRender: grapheme output + provenance
    lua_chon.rs       — lua_chon: quyết định raw vs biến đổi
```

### Nguyên tắc thiết kế

* **Không dynamic dispatch**: engine Telex/VNI là free function, chọn qua
  `match KieuGo` trong `anh_xa.rs`. Không `Box<dyn>`.
* **Layer trung lập kiểu gõ**: `don_vi`, `bo_dat_dau`, `am_tiet`, `render`,
  `lua_chon`, `phan_doan`, `ngu_canh` dùng chung cho mọi kiểu gõ.
* **Provenance**: mỗi `DonViRender` mang `raw_bat_dau`/`raw_ket_thuc` +
  `thao_tac_anh_huong` (non-contiguous). Cursor navigation dựa trên raw position.
* **Replay, không incremental**: mỗi thay đổi rebuild toàn bộ snapshot từ
  raw history (`xay_lai_ban_chup`). Deterministic.
* **Tầng render không biết shape**: chỉ `render.rs` biết `ế` = code point nào.
  `chu_viet.rs` chỉ giữ cấu trúc domain.

---

## Quy tắc đóng góp

* **Mỗi bug phải có regression test.** Sửa lỗi → thêm test chứng minh lỗi hết.
* **Không thêm dependency tùy tiện.** Mọi dependency phải có mục đích rõ, được
  ghi trong `docs/DEPENDENCIES.md`. Runtime dep hiện có: `unicode-segmentation`,
  `unicode-normalization` (cả hai no_std compatible). Dev: `proptest`,
  `criterion`, `serde_json`.
* **Không triển khai ngoài phạm vi core**: FFI, GUI, IPC, network, thread,
  async runtime, nhận diện ứng dụng — đó là vai trò của CadenceRuntime (repo riêng).
* **Test mọi feature combination**: `--all-features`, `--no-default-features`,
  `--no-default-features --features serde,trace`.

---

## Phát hành

Phát hành tự động qua `.github/workflows/release.yml` (xem `docs/RELEASE.md`).

* **Version scheme**: calendar/change/patch `<năm>.<thay đổi>.<vá>` (vd `2026.1.1`).
  Xem `docs/VERSIONING.md`, RFC 0024.
* **Patch** (tăng thành phần 3): sửa bug, không phá code, không thêm API.
* **Change** (tăng thành phần 2): tính năng mới, thay đổi hành vi, đổi API.
* **Quy trình**:
  1. Bump `version` trong `Cargo.toml` + `Cargo.lock`.
  2. Thêm entry `## [<version>] - <date>` trong `CHANGELOG.md`.
  3. Merge qua PR như thường.
  4. Tạo annotated tag: `git tag -a v<version> -m "Cadence <version>"`.
  5. Push tag: `git push origin v<version>` — kích hoạt release workflow.
  6. Workflow chạy gate, đóng gói, publish crates.io, tạo GitHub Release.

* **Không force-push, không xóa, không di chuyển tag.** Tag `v0.1.0` (mốc nội
  bộ) không bao giờ thay đổi.
* **Release notes** trích tự động từ `CHANGELOG.md` (section giữa `## [<version>]`
  và `##` tiếp theo). Viết đầy đủ.

---

## Bất biến quan trọng

Xem `docs/INVARIANTS.md` cho danh sách đầy đủ + test khóa. Tóm tắt:

1. **Raw byte-for-byte**: `noi_dung_goc()` giữ raw nguyên, không normalize.
2. **Determinism**: cùng config + history → cùng snapshot.
3. **Undo**: thêm → xóa lùi → snapshot cũ. Chèn giữa → xóa đúng vị trí → cũ.
4. **Unicode**: NFC idempotent dưới NFC, NFD idempotent dưới NFD. Raw không
   normalize. Cursor là UTF-8 boundary, grapheme boundary.
5. **Phân đoạn**: đoạn không overlap, phủ toàn history, không xuyên ranh giới
   `them_nguyen_ban`. Tone/shape không xuyên ranh giới cấm.
6. **Giới hạn**: history không vượt `gioi_han_thao_tac`. Vượt → `KhongDoi`, state
   giữ. Không recursion theo input length.
7. **Isolation**: hai `PhienGo` không chia sẻ mutable state.
8. **Commit/reset**: commit phiên rỗng → `KhongDoi`; có nội dung → `ChapNhan`
   rồi reset sạch.
9. **Selection**: 2+ dấu thanh → raw. shape + onset sai → raw. shape ở xa +
   âm tiết không hợp lệ → raw. Cấu trúc kỹ thuật (URL, `::`, `=`) luôn raw.
10. **Trace**: zero-overhead khi tắt feature, không đổi output, không chứa
    pointer/timing/machine data.

---

## Tài liệu tham khảo

| Tài liệu | Nội dung |
|---|---|
| `README.md` | Giới thiệu, trạng thái, sử dụng |
| `CONTRIBUTING.md` | Quy ước đóng góp, lệnh kiểm tra, quy trình git |
| `docs/VERSIONING.md` | Hệ phiên bản calendar/change/patch |
| `docs/RELEASE.md` | Quy trình phát hành + gate |
| `docs/INVARIANTS.md` | Bất biến + liên kết test |
| `docs/SECURITY_MODEL.md` | Mô hình bảo mật, unsafe policy |
| `docs/API_STABILITY.md` | Chính sách ổn định API |
| `docs/MSRV.md` | Chính sách MSRV (Rust 1.85) |
| `docs/DEPENDENCIES.md` | Chính sách dependency |
| `docs/rfc/` | RFC 0001–0024 (quy tắc, kiến trúc, quyết định) |
| `docs/INTEGRATION.md` | Hướng dẫn tích hợp |

---

## Tóm tắt cho agent

> **Trước khi code**: đọc `CONTRIBUTING.md` + `docs/INVARIANTS.md` + RFC liên
> quan. **Trước khi PR**: chạy gate. **Commit**: tiếng Việt không dấu. **Code**:
> tiếng Việt không dấu (identifier) + có dấu (comment). **Không commit main**:
> branch → PR → review → merge commit. **Mỗi bug có regression test.**
