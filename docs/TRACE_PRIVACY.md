# Trace và quyền riêng tư

Trace của Cadence là feature **opt-in** (`feature = "trace"`). Tài liệu này nêu
cam kết quyền riêng tư và hướng dẫn host ứng dụng.

## Cam kết của core

Khi `trace` **tắt**:
- Toàn bộ module `trace` bị `cfg` ẩn, không compile.
- Không allocation trace, không format `String` trace.
- Output không đổi (trace không ảnh hưởng pipeline).
- Không overhead.

Khi `trace` **bật**:
- Trace chỉ chứa **token hiện tại** của mỗi đoạn: chuỗi raw và chuỗi output.
- Trace **không** chứa:
  - Pointer, địa chỉ bộ nhớ.
  - Timing, timestamp.
  - Machine-specific data (thread id, process id).
  - Config đầy đủ (chỉ snapshot quyết định per-đoạn).
  - Lịch sử thao tác đầy đủ (chỉ chuỗi raw/ra của đoạn hiện tại).
- Trace là snapshot chỉ đọc, clone chuỗi, không giữ reference vào `PhienGo`.
- Trace deterministic: cùng config + history → cùng trace.

## Kiểm chứng

| Test | Bất biến |
|---|---|
| `tests/trace.rs::trace_deterministic` | trace deterministic |
| `tests/trace.rs::trace_telex_bien_doi` | trace chứa chuỗi raw/ra, không chứa metadata |
| `tests/property_phase3.rs::deterministic` | trace bật/tắt không đổi output (qua cùng pipeline) |
| `tests/contract.rs::phien_go_send_sync` | trace field không phá Send/Sync |

`TraceStep` chỉ có 6 field: `doan_bat_dau`, `doan_ket_thuc` (raw index),
`bang_chung` (enum bằng chứng), `ket_qua` (Telex/NguyenBan), `chuoi_raw`,
`chuoi_ra`. Không có field timing/pointer.

## Hướng dẫn host

Cadence là lõi xử lý nhập liệu. **Nội dung người dùng gõ là dữ liệu nhạy cảm.**
Host application tích hợp Cadence nên tuân thủ:

1. **Không log raw input mặc định.** Không gửi `noi_dung_goc()`, `chuoi_raw`,
   hoặc trace lên network, file log, telemetry mà không có ý định rõ.
2. **Không gửi trace lên network.** Trace là công cụ debug cục bộ. Nếu gửi,
   phải xin phép người dùng và redact.
3. **Phải xin phép người dùng** nếu lưu nội dung nhập (analytics, bug report).
4. **Nên redact khi báo bug.** Thay nội dung thật bằng chuỗi mẫu (ASCII, ký tự
   Unicode công khai). Dùng trace để giải thích **quyết định** (Telex/raw),
   không phải để trích dẫn nội dung.
5. **Trace tắt trong production build** trừ khi debug. Feature `trace` mặc định
   tắt; bật qua `Cargo.toml` của host.

## Không telemetry trong core

Cadence không có và sẽ không có:
- Logging framework (`log`, `tracing`).
- Telemetry, metrics, analytics.
- Network call.
- File I/O.
- Environment variable đọc trong core.

Mọi "logging" phải do host quyết định, không phải core.

## Tác động no_std

Feature `trace` yêu cầu `alloc` (đã có). Không thêm dependency. Trace dùng
`alloc::string::String` và `alloc::vec::Vec` (đã có).
