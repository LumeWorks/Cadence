# CHANGELOG

## [2026.1.1] - 2026-08-08

Patch sửa linh hoạt thứ tự tone/hình chữ cho Telex — dấu chèn ở khắp nơi
trong từ vẫn ra đúng chữ (parity VNI, RFC 0021).

### Sửa

- **Thứ tự linh hoạt Telex** (`src/kieu_go/telex.rs`): phím hình chữ
  (`w`/`a`/`e`/`o`/`d`) reach back tới base trần trong đoạn, không nhất thiết
  ngay sau base. `oiw`→ơi, `voiws`→với, `moiws`→mới, `uoiw`→ươi,
  `khongo`→không, `uongw`→ương. Bỏ forward-adjacency path cũ; shape giờ đi
  một đường backward search + mutate tại chỗ (giống VNI `ap_dung_modifier`).
- **Chặn reshape tiếng Anh** (`src/kieu_go/lua_chon.rs` Rule 2): shape ở xa
  (reach back qua ký tự khác) + âm tiết không hợp lệ → raw. Giữ `cadence`,
  `release`, `HTTPServer`, `deadbeefcafe` raw; shape liền base (`ddm`→đm) và
  tone-only (`text`→tẽt, `use`→úe) không bị chặn.
- **`co_hinh_xa` flag** (`src/kieu_go/don_vi.rs`): engine báo shape reach back
  qua ký tự khác; `lua_chon` dùng để chặn reshape rác. VNI luôn `false`.

### Thêm

- `tests/telex_thu_tu_linh_hoat.rs`: 49 test thứ tự linh hoạt + regression
  (tiếng Việt, escape, side-effect parity VNI, tiếng Anh không reshape, case).
- 10 unit test trong `telex.rs` (backward search, ươ hai hướng, escape
  backward, case preservation, không restroke chữ đã có dấu).
- RFC 0006: mục "Thứ tự linh hoạt" + "Chặn reshape tiếng Anh" + bất biến mới.
- `INVARIANTS.md` §11: bất biến shape reach back + gate, liên kết test.

### Không phá code

- 776 test pass (từ 717), 0 warning clippy, fmt clean.
- Toàn bộ corpus tiếng Anh raw giữ nguyên (4 test corpus xanh).
- `noi_dung_goc()` byte-for-byte raw không đổi; không thêm API mới.
- `asws` chốt thành `ắ` (xác định, không còn ambiguous như forward cũ).

## [2026.1.0] - 2026-08-06

Bản phát hành công khai đầu tiên. Telex + VNI, phân đoạn ngữ cảnh, code/chat
preservation, Unicode NFC/NFD, raw history/replay, trace, serde, no_std.

### Thêm

- **VNI engine** (`src/kieu_go/vni.rs`): digit `1..=5` dấu thanh, `6/7/8/9`
  hình chữ (mũ/móc/trăng/đ), ươ đặc biệt, thứ tự đảo, thay dấu, escape.
- **`KieuGo` enum** (`Telex`, `Vni`) + `CauHinh::kieu_go()` /
  `dat_kieu_go()`. Mặc định `Telex`.
- **Kiến trúc nhiều kiểu gõ** (RFC 0020): enum dispatch, không dynamic
  dispatch, lớp dùng chung (`don_vi`, `bo_dat_dau`, `am_tiet`, `render`,
  `lua_chon`, `phan_doan`, `ngu_canh`, `anh_xa`, `trace`).
- **Bộ đặt dấu chung** (`src/kieu_go/bo_dat_dau.rs`): `tim_nguyen_am_chinh`
  dùng chung Telex + VNI. Cải thiện `uy`/`ay` (bán âm `y`) và `qu`+`y` (nucleus
  → `quý`, `thủy`, `khuỷu`).
- **Nhận diện chuỗi số kỹ thuật VNI** (RFC 0022): 2+ digit VNI + âm tiết
  KhongHopLe → raw (`sha256`, `h264`, `user123`). Nhận diện bằng cấu trúc,
  không từ điển.
- **Trace VNI** (`TraceKetQua::Vni`): trace phân biệt Telex/VNI. Bằng chứng
  mới: `DauThanhVni`, `DauChuVni`, `ChuoiSoKyThuat`, `ChuoiPhienBan`.
- **RFC 0020–0024**: kiến trúc nhiều kiểu gõ, quy tắc VNI, VNI và chuỗi kỹ
  thuật, escape/hoàn tác VNI, hệ phiên bản Cadence.
- **`docs/VERSIONING.md`**: hệ phiên bản calendar/change/patch.
- **`docs/MIGRATION_0.1_TO_2026.1.md`**: migration guide.
- **`docs/research/vni-sources.md`**: nguồn nghiên cứu VNI.
- **VNI tests**: dấu thanh, hình chữ, từ/âm tiết, xung đột số, mixed
  content, escape, property (7), soak (4).
- 717 tests (40 test files) across all feature combinations.

### Không phá code

- `CauHinh::mac_dinh()` vẫn chọn Telex.
- Toàn bộ 655 test Telex cũ xanh, không sửa expected.
- `dat_kieu_telex` vẫn dùng (chỉ khi `kieu_go == Telex`).

## [0.1.0] - 2026-08-06 — Mốc khóa lõi trước phát hành công khai

Phát hành nội bộ. Lõi gõ tiếng Việt Telex đầy đủ, phân đoạn ngữ cảnh,
ổn định API, 655 tests across all feature combinations.

### Phase 4 - Ổn định và kiểm tra

Giai đoạn này ổn định và kiểm tra cho phát hành `0.1.0`: audit API, tài liệu
bất biến, mô hình bảo mật, chính sách MSRV, cargo-deny config, rule matrix
tests, editing/Unicode matrix tests, property/serde tests, soak tests, và
sửa một bug cursor.

### Thêm

- **Tài liệu Phase 4**: baseline audit (`PHASE3_BASELINE.md`), bảng bất biến
  liên kết test (`INVARIANTS.md`), public API inventory (`api/public-api-0.1.0.md`),
  chính sách ổn định API (`API_STABILITY.md`), mô hình bảo mật (`SECURITY_MODEL.md`),
  chính sách MSRV (`MSRV.md`), quyền riêng tư trace (`TRACE_PRIVACY.md`),
  chính sách dependency (`DEPENDENCIES.md`).
- **cargo-deny config** (`deny.toml`): license allowlist, ban advisories,
  warn multiple-versions.
- **Contract tests** (`tests/contract.rs`): compile-time Send/Sync/Clone/Static
  cho mọi public type + runtime thread smoke tests.
- **Corpus Phase 4** (`tests/corpus/`): 14 module, 120 test covering tiếng
  Việt, hình chữ, dấu thanh, escape, âm tiết, code, command, URL/email/path,
  teencode, emoticon, Unicode, context mix, editing, adversarial.
- **Rule matrix unit tests** (`src/am_tiet.rs`, `chu_viet.rs`, `render.rs`,
  `telex.rs`): 36 test tự kiểm tra bảng nội bộ (onset/coda shadow, tone key
  mapping, NFC round-trip, shape mapping).
- **Editing matrix tests** (`tests/corpus/editing.rs`): 7 matrix tests (mỗi
  thao tác × mỗi vị trí) + 4 backspace+retype + 2 delete-forward trên Telex.
- **NFC/NFD equivalence matrix** (`tests/corpus/unicode.rs`): 35 tổ hợp
  shape × tone canonical equivalent, grapheme count matrix, NFD cursor
  movement vào grapheme phân rã.
- **Property tests** (`tests/property.rs`): 8 property mới (navigation không
  đổi nội dung, cursor round-trip, boundary KhongDoi, loai_noi_dung ổn định,
  chap_nhan trả đúng, hai phiên cùng loai, xoa boundary KhongDoi).
- **Serde tests** (`tests/serde.rs`): 4 type derive tests + 5 round-trip
  tests (serde_json serialize→deserialize→equals).
- **Regression tests** (`tests/regression.rs`): 3 regression cho bug
  `di_phai_raw` ở cuối lịch sử.
- **Soak tests** (`tests/soak.rs`): 10 test chịu tải dài (1000 ký tự,
  navigation liên tục, chen/xóa lặp, mọi tổ hợp cấu hình, emoji+combining,
  giới hạn thấp, xóa đến rỗng).
- **Dev-dependency**: `serde_json` cho round-trip serde tests.
- 655 tests (34 test files) across all feature combinations.

### Sửa

- **Bug `di_phai_raw` ở cuối lịch sử**: khi raw cuối là tone key (không
  navigable), `di_phai` trả `CapNhat` sai thay vì `KhongDoi`. Nguyên nhân:
  `snap_raw` snap về navigable gần nhất, trả snapped value ≠ r, khiến
  caller thấy `moi != r`. Fix: trả `r` gốc (không snap) khi ở hoặc vượt
  navigable cuối.

### Bất biến Phase 4

- `di_phai` ở cuối luôn `KhongDoi`; `di_trai` ở đầu luôn `KhongDoi`.
- `xoa_lui` ở đầu luôn `KhongDoi`; `xoa_phia_truoc` ở cuối luôn `KhongDoi`.
- Navigation không thay đổi nội dung (raw và rendered).
- Mọi tổ hợp cấu hình (2×2×2×3=24) engine ổn định.
- NFC/NFD canonical equivalent cho mọi shape × tone.
- Serde round-trip cho mọi public data type.
- 1000 ký tự Telex liên tục: không panic, cursor hợp lệ.

### Hạn chế đã biết (0.1.0)

- **Ký tự Việt HOA dựng sẵn** (vd `Ế`, `Đ`) không được `phan_tich_ky_tu` nhận diện
  khi gõ trực tiếp (to_ascii_lowercase không đổi non-ASCII). Ký tự đó được giữ
  raw (an toàn), không parse. Workaround: dùng Telex để tạo chữ hoa (vd `Dd` → `Đ`).
- **Chỉ hỗ trợ Telex**. Chưa có VNI, VIQR, tự điển, autocomplete (theo phạm vi 0.1).
- **Chưa có differential testing harness** (so sánh black-box với engine tham chiếu).
  Correctness dựa vào 655 test, property, soak 2 triệu thao tác và rule matrix.
  Kế hoạch 0.2.
- **Không có FFI/adapter nền tảng** (Fcitx5, IBus, TSF, Wayland). Core là Rust
  library; CadenceRuntime là repository riêng.
- **`cargo-deny`/`cargo-audit`** là CI tool, không phải runtime dependency. Gate
  đã chạy xanh trong audit phát hành (xem `docs/RELEASE_CANDIDATE_REPORT.md`).

## [Unreleased] - Phase 3

Giai đoạn này triển khai triết lý "Gõ mọi thứ bạn cần": phân đoạn lịch sử,
nhận diện ngữ cảnh kỹ thuật, lựa chọn raw/Telex theo đoạn, chính sách lựa
chọn, và trace quyết định có cấu trúc.

### Thêm

- **Phân đoạn** (`phan_doan.rs`): chia lịch sử raw thành `Vec<Doan>` theo
  `LoaiDoan` (Chu/So/KhoangTrang/DauCau/KyThuat/Emoji/NguyenBan). Telex chạy
  độc lập từng đoạn Chu, không xuyên ranh giới.
- **Teencode lặp** (`phan_doan::la_teencode_lap`): run 3+ chữ cái doubled-base
  (`a`/`e`/`o`/`d`) có chữ khác trước → bảo toàn raw (`brooooo`→`brooooo`).
- **Nhận diện ngữ cảnh** (`ngu_canh.rs`): URL (`://`), email (`@`), đường dẫn
  (`/`, `~/`, `./`, `X:\`), code span/fence (backtick), namespace `::`,
  phép gán `=`, emoticon (`=)`, `:D`, `???`, ...). Hai pass: span structure
  + per-segment adjacency. `KetQuaNhanDien` + `BangChungLuaChon` (13 variants).
- **Luật raw cục bộ** (`lua_chon.rs`): Rule 0 (2+ dấu thanh→raw), Rule 1
  (shape + onset sai→raw), giữ Rule 2–5 Phase 2.
- **Nucleus-glide** (`am_tiet.rs`): 2 nguyên âm đầy không glide → `KhongHopLe`
  (`ae`→raw, ngăn CASE).
- **Chính sách lựa chọn** (`ChinhSachLuaChon`): `TuNhien` (mặc định),
  `UuTienTiengViet` (thông Rule 5), `UuTienNguyenBan`. Getter/setter trên
  `CauHinh`, wired vào `lua_chon` và `anh_xa::xay_lai`.
- **Trace API** (`trace.rs`, feature `trace`): `TraceStep` + `TraceKetQua`,
  `PhienGo::trace()` trả snapshot quyết định per-đoạn. `BangChungLuaChon`
  public khi trace bật.
- **RFC 0013–0019**: triết lý, phân đoạn, nhận diện, teencode, chính sách,
  trace, corpus.
- **Benchmark Phase 3**: code trộn, URL, namespace, teencode lạp.
- **Example `go_moi_thu`**: demo trộn code, URL, tiếng Việt, teencode,
  emoticon, chính sách.
- 435 tests (29 test files): corpus Phase 3 (43), property Phase 3 (8),
    chinh_sach_lua_chon (8), trace (6), phan_doan (10), ngu_canh (22), plus
    Phase 1–2 giữ nguyên.

### Bất biến Phase 3

- Cấu trúc kỹ thuật chắc chắn (URL, email, code fence, `::`, `=`) luôn raw
  trong mọi chính sách.
- `them_nguyen_ban` luôn chặn Telex bất kể chính sách.
- `noi_dung_goc()` giữ byte-for-byte raw.
- Trace deterministic: cùng cấu hình + history → cùng trace.
- Trace zero-overhead khi tắt feature.

### DoD acceptance cases (46/46 pass)

`async`, `class`, `struct`, `String`, `user_id`, `userName`, `HTTPServer`,
`SCREAMING_SNAKE_CASE`, `foo::bar`, `cargo build --release`, `fn main() {}`,
`let mut buf = String::new();`, `https://example.com`, `http://localhost:3000`,
`name@example.com`, `~/Documents/Cadence`, `./install.sh`, `C:\Users\Name`,
`127.0.0.1:8080`, `v1.2.3`, `c9868e1`, UUID, `=))`, `=))))))))))))`, `:v`,
`???`, `!!!!!!!`, `brooooo`, `vcl`, `ko`, `dc`, `ddm`, `cargo build lỗi rồi =))`,
`let ten_nguoi_dung = "Minh";`, `đm bug gì lắm thế`, `user_id của m là gì?`,
`brooooo m đang làm gì đấy???`, `tieengs`, `nguowif`, `dduwowngf`, `AA`, `DD`.

## [Phase 2]

Giai đoạn này triển khai Telex engine đầy đủ: hình chữ, dấu thanh, escape,
phân tích âm tiết, lựa chọn raw/Telex, và Unicode NFC/NFD output.

### Thêm

- **Telex engine** (`telex.rs`): biến đổi hình chữ (aa/aw/ee/oo/ow/uw/dd),
  dấu thanh (s/f/r/x/j/z), escape (lặp phím modifier), `uo`+`w`→`ươ` (tam
  nguyên âm), nguyên âm chính cho bo đặt dấu, bán âm cuối `i`/`u`/`o`.
- **Mô hình chữ viết** (`chu_viet.rs`): `ChuGoc`, `DauChu`, `DauThanh`,
  `KieuHoa`, `ChuCaiViet`.
- **Unicode render** (`render.rs`): bảng lookup NFC cho 134 tổ hợp nguyên âm ×
  dấu chữ × dấu thanh, NFD output, `phan_tich_ky_tu` reverse lookup.
- **Phân tích âm tiết** (`am_tiet.rs`): bảng âm đầu/âm cuối, parser
  `phan_tich_am_tiet`, `bat_dau_onset_hop_le`, `raw_co_onset_hop_le`.
- **Lựa chọn raw/Telex** (`lua_chon.rs`): shape→Telex, escape→Telex,
  tone+invalid→raw, invalid onset→raw, `them_nguyen_ban` chặn parse.
- **Ánh xạ pipeline** (`anh_xa.rs`): `xay_lai` rebuild từ raw, `raw_to_byte`
  mapping, grapheme navigation, snap cursor cho snapshot, `AmTietTiengViet`.
- `CauHinh` mở rộng: `KieuTelex` (CanBang/DayDu với `w`→`ư`, `[`→`ư`, `]`→`ơ`),
  `QuyTacDatDau` (HienDai/TruyenThong cho on-glide `oa`/`oe`), `DangUnicode`
  (Nfc/Nfd).
- `LoaiNoiDung` mở rộng: `BienDoiTelex`, `AmTietTiengViet`.
- `BanChupSoan::dung` nhận thêm `noi_dung_goc`, `chi_so_byte`, `loai_noi_dung`.
- Dependency `unicode-normalization` 0.1.25 (no_std compatible).
- Benchmark Phase 2: shape transform, tone mark, escape, âm tiết dài, `người`.
- RFC 0006–0012, tài liệu nguồn nghiên cứu.
- 299 tests (24 test files): DoD, shape, tone, escape, hoa, DayDu, NFD,
  round-trip, nguyen_ban, mix, con_tro, lua_chon, am_tiet, quy_tac_dat_dau.

### Bất biến Phase 2

- `noi_dung_goc()` trả byte-for-byte raw (không biến đổi).
- Shape transform luôn giữ Telex; không fallback raw.
- Escape luôn giữ Telex; ý định người dùng.
- `them_nguyen_ban` bypass Telex; ký tự literal, chặn Telex rules xuyên.
- Con trỏ raw là source of truth; snap chỉ cho snapshot byte offset.
- Nguyên âm chính nhận dấu thanh (không phải bán âm cuối `i`/`u`/`o`).
- On-glide `oa`/`oe`: HienDai trên `o`, TruyenThong trên `a`/`e`.
- NFC/NFD canonical equivalence: `NFD(NFC(x)) == NFD(x)`.

### DoD acceptance cases (19/19 pass)

`tieengs→tiếng`, `Vieetj→Việt`, `dduwowngf→đường`, `ddaay→đây`,
`nguowif→người`, `aa→â`, `aw→ă`, `ee→ê`, `oo→ô`, `ow→ơ`, `uw→ư`, `dd→đ`,
`ass→as`, `aaa→aa`, `aww→aw`, `ddd→dd`, `async→async`, `class→class`,
`ddm→đm`.

## [Phase 1]

Giai đoạn này dựng nền móng bất biến của Cadence. **Telex chưa được triển khai.**
Mọi ký tự được render nguyên bản.

### Thêm

- Library crate `cadence` duy nhất tại root.
- Giấy phép MPL-2.0 và NOTICE.
- Feature `std` (mặc định), `no_std + alloc`, `trace`, `serde` (optional).
- Lint `unsafe_code = forbid`, `missing_docs = warn`, clippy correctness/suspicious
  deny, `unwrap_used`/`wildcard_imports`/`enum_glob_use` deny.
- CI cho Linux/Windows/macOS, stable và MSRV 1.85.
- `CauHinh` với `gioi_han_thao_tac` (mặc định 128, `1..=4096`) và `LoiCauHinh`
  domain (Display + `std::error::Error` khi bật `std`).
- `BoGo` factory bất biến tạo `PhienGo` độc lập.
- `PhienGo` giữ lịch sử thao tác làm nguồn sự thật; replay nguyên bản sau mỗi
  thay đổi.
- Thao tác: `them_ky_tu`, `them_nguyen_ban`, `xoa_lui`, `xoa_phia_truoc`,
  `di_trai`, `di_phai`, `ve_dau`, `ve_cuoi`, `chap_nhan`, `khoi_phuc_nguyen_ban`,
  `dat_lai`.
- `KetQuaXuLy` (`KhongDoi`, `CapNhat`, `ChapNhan { noi_dung }`).
- `BanChupSoan` snapshot trung lập nền tảng (`noi_dung`, `noi_dung_goc`,
  `con_tro`, `loai_noi_dung`).
- `ViTriVanBan` theo byte/UTF-16/grapheme (dùng `unicode-segmentation`).
- `LoaiNoiDung` (`Trong`, `NguyenBan`).
- Giới hạn thao tác: vượt giới hạn trả `KhongDoi`, giữ nguyên state.
- Tài liệu kiến trúc và RFC 0001–0005.
- Test tích hợp theo nhóm (cấu hình, phiên cơ bản, con trỏ, Unicode, giới hạn).
- Property test cho mười bất biến nền tảng (proptest) kèm regression case.
- Benchmark nền (criterion): thêm ASCII/Unicode, chèn giữa, xóa lùi, replay 16 và
  128 thao tác.

### Bất biến Phase 1

- `noi_dung == noi_dung_goc` (chưa có Telex).
- `chap_nhan` phiên rỗng trả `KhongDoi`; phiên có nội dung trả `ChapNhan` rồi reset.
- Hai phiên từ cùng `BoGo` hoàn toàn độc lập.
- Vị trí byte luôn là ranh giới UTF-8; vị trí grapheme luôn là ranh giới cluster.
- Số thao tác không vượt `gioi_han_thao_tac`.

### Cấm trong Phase 1

Không triển khai Telex, VNI, dấu thanh, âm tiết, từ điển, heuristic code/URL,
trace Telex, FFI, adapter nền tảng, serialization snapshot nếu chưa có use case,
dynamic plugin, callback, event dispatcher, regex, cache incremental phức tạp,
`unsafe`, background task, logging raw input.
