# RFC 0018 - Trace API có cấu trúc

Trạng thái: Chấp thuận - Phase 3 (chưa triển khai).

## Vấn đề

Khi Cadence chọn raw thay vì Telex (hoặc ngược lại), người dùng và tooling cần
biết **lý do**. Logging runtime phổ thông không phù hợp: `no_std`, overhead,
khó audit. Cần trace có cấu trúc, opt-in qua feature, không phá API.

## Quyết định

Feature `trace` (đã có trong `Cargo.toml`, hiện là no-op) kích hoạt module
`trace.rs` với snapshot quyết định:

### `TraceStep`

```text
struct TraceStep {
    doan_bat_dau: usize,      // raw index đầu
    doan_ket_thuc: usize,     // raw index cuối
    loai_doan: LoaiDoan,      // loại đoạn
    bang_chung: BangChungLuaChon,  // lý do
    ket_qua: KetQuaLuaChon,   // Telex hay NguyenBan
    chuoi_raw: String,        // input đoạn
    chuoi_ra: String,         // output đoạn
}
```

### `PhienGo::trace() -> Vec<TraceStep>`

Trả snapshot quyết định cho phiên hiện tại. Chỉ available khi feature `trace`
bật:

```rust
#[cfg(feature = "trace")]
impl PhienGo {
    pub fn trace(&self) -> Vec<TraceStep> { ... }
}
```

### Nguyên tắc

* **Không overhead khi tắt**: `cfg(feature = "trace")` ẩn toàn bộ code trace.
* **Snapshot immutable**: trace không giữ reference vào `PhienGo`; clone chuỗi.
* **Deterministic**: cùng cấu hình + history → cùng trace.
* **Không I/O**: trace không ghi file/log; caller quyết định cách dùng.

## Tác động `no_std`

Feature `trace` yêu cầu `alloc` (đã có). Không thêm dependency.

## Tác động public API

* `TraceStep` và `PhienGo::trace()` chỉ tồn tại khi `feature = "trace"`.
* `BangChungLuaChon` cần `pub` (hiện `pub(crate)`) để trace user-facing.
* Không breaking change khi tắt trace.

## Phương án bị loại

* **`log` crate**: bị loại - dependency ngoài, không `no_std` mặc định.
* **Callback mỗi thao tác**: bị loại - phức tạp, overhead ngay cả khi không dùng.
* **Global trace buffer**: bị loại - vi phạm "mỗi PhienGo độc lập".
