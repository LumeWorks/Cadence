# CHANGELOG

## [Unreleased] — Phase 2

Giai đoạn này triển khai Telex engine đầy đủ: hình chữ, dấu thanh, escape,
phân tích âm tiết, lựa chọn raw/Telex, và Unicode NFC/NFD output.

### Thêm

- **Telex engine** (`telex.rs`): biến đổi hình chữ (aa/aw/ee/oo/ow/uw/dd),
  dấu thanh (s/f/r/x/j/z), escape (lặp phím modifier), `uo`+`w`→`ươ` (tam
  nguyên âm), nguyên âm chính cho bo đặt dấu.
- **Mô hình chữ viết** (`chu_viet.rs`): `ChuGoc`, `DauChu`, `DauThanh`,
  `KieuHoa`, `ChuCaiViet`.
- **Unicode render** (`render.rs`): bảng lookup NFC cho 134 tổ hợp nguyên âm ×
  dấu chữ × dấu thanh, NFD output, `phan_tich_ky_tu` reverse lookup.
- **Phân tích âm tiết** (`am_tiet.rs`): bảng âm đầu/âm cuối, parser
  `phan_tich_am_tiet`, `bat_dau_onset_hop_le`, `raw_co_onset_hop_le`.
- **Lựa chọn raw/Telex** (`lua_chon.rs`): shape→Telex, escape→Telex,
  tone+invalid→raw, invalid onset→raw.
- **Ánh xạ pipeline** (`anh_xa.rs`): `xay_lai` rebuild từ raw, `raw_to_byte`
  mapping, grapheme navigation, snap cursor cho snapshot.
- `CauHinh` mở rộng: `KieuTelex` (CanBang/DayDu), `QuyTacDatDau`
  (HienDai/TruyenThong), `DangUnicode` (Nfc/Nfd).
- `LoaiNoiDung` mở rộng: `BienDoiTelex`, `AmTietTiengViet`.
- `BanChupSoan::dung` nhận thêm `noi_dung_goc`, `chi_so_byte`, `loai_noi_dung`.
- Dependency `unicode-normalization` 0.1.25 (no_std compatible).
- Benchmark Phase 2: shape transform, tone mark, escape, âm tiết dài, `người`.
- RFC 0006–0012.
- Test acceptance 19 DoD cases (`tests/telex_dod.rs`).

### Bất biến Phase 2

- `noi_dung_goc()` trả byte-for-byte raw (không biến đổi).
- Shape transform luôn giữ Telex; không fallback raw.
- Escape luôn giữ Telex; ý định người dùng.
- `them_nguyen_ban` bypass Telex; ký tự literal, chặn Telex rules.
- Con trỏ raw là source of truth; snap chỉ cho snapshot byte offset.
- Nguyên âm chính nhận dấu thanh (không phải bán âm cuối `i`/`u`).

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
