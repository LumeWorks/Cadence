# CHANGELOG

## [Unreleased] — Phase 1

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
