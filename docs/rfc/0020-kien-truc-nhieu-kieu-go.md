# RFC 0020 - Kiến trúc nhiều kiểu gõ

Trạng thái: Chấp thuận - 2026.1.0 (đã triển khai).

## Vấn đề

Cadence 0.1.0 chỉ hỗ trợ Telex. Để hỗ trợ VNI và các kiểu gõ sau này,
cần kiến trúc cho phép nhiều kiểu gõ mà không copy pipeline, không dynamic
dispatch, và không phá hành vi Telex hiện có.

## Bất biến

* Mỗi phiên dùng đúng một kiểu gõ (`KieuGo`).
* Hai kiểu gõ không hoạt động đồng thời trong cùng phiên.
* Mặc định là `Telex` để không thay đổi hành vi người dùng hiện tại.
* Raw history là nguồn sự thật duy nhất, bất kể kiểu gõ.
* `noi_dung_goc()` trả byte-for-byte raw cho mọi kiểu gõ.
* `them_nguyen_ban` chặn modifier của mọi kiểu gõ.
* Cursor/provenance dùng chung cho mọi kiểu gõ.

## Quyết định

### Enum dispatch, không dynamic dispatch

```rust
pub(crate) enum BoNhanKieuGo {
    Telex(BoNhanTelex),
    Vni(BoNhanVni),
}
```

Thực tế triển khai: `anh_xa::render_chu` match trên `KieuGo` và gọi
`telex::xu_ly_doan_chu` hoặc `vni::xu_ly_doan_chu`. Không `Box<dyn>`, không
dynamic allocation, không trait object.

### Lớp dùng chung

Telex và VNI chỉ khác cách diễn giải raw action thành ý định chữ Việt.
Các lớp dùng chung:

* `don_vi` — `DonViRender`, `NoiDungDonVi`, `KetQuaDoanChu`.
* `bo_dat_dau` — `tim_nguyen_am_chinh`, `tim_nguyen_am_cuoi`, `vi_tri_chen`.
* `am_tiet` — parser âm tiết.
* `render` — Unicode NFC/NFD.
* `lua_chon` — selection raw/biến đổi.
* `phan_doan` — phân đoạn theo loại ký tự.
* `ngu_canh` — nhận diện ngữ cảnh kỹ thuật.
* `anh_xa` — cursor/provenance, `xay_lai`.
* `trace` — trace quyết định.

Không copy pipeline Telex thành pipeline VNI riêng.

### Public API

```rust
pub enum KieuGo {
    Telex,
    Vni,
}

impl CauHinh {
    pub fn kieu_go(&self) -> KieuGo;
    pub fn dat_kieu_go(&mut self, kieu_go: KieuGo);
}
```

Không dùng boolean `dung_telex: bool` / `dung_vni: bool`.

## Rule table

| Kiểu gõ | Dấu thanh | Hình chữ | Escape | Phân đoạn digit |
|---------|-----------|----------|--------|------------------|
| Telex   | s/f/r/x/j/z | w/a/e/o/d | lặp phím | digit = So (raw) |
| VNI     | 1/2/3/4/5 | 6/7/8/9 | lặp digit | digit 1-9 = Chu (modifier) |

## Ví dụ

```rust
let mut c = CauHinh::mac_dinh(); // Telex
c.dat_kieu_go(KieuGo::Vni);
let bo_go = BoGo::new(c).expect("hop le");
```

## Phản ví dụ

* `dung_telex: bool` — bị loại: không rõ ràng, cho phép trạng thái vô hiệu.
* `Box<dyn BoNhanKieuGo>` — bị loại: dynamic dispatch, allocation.
* Copy pipeline Telex — bị loại: trùng lặp, khó bảo trì.
* Trait public — bị loại: lộ internal abstraction ra API.

## Phương án bị loại

* **Trait object dispatch**: bị loại vì dynamic allocation và dispatch.
* **Copy pipeline**: bị loại vì trùng lặp code.
* **Boolean flag**: bị loại vì không type-safe.
* **Feature flag `vni`**: bị loại vì VNI là lõi, không optional.

## Tác động public API

* Thêm `KieuGo` enum (public, serde).
* Thêm `CauHinh::kieu_go()`, `CauHinh::dat_kieu_go()`.
* `CauHinh::dat_kieu_telex` vẫn giữ (chỉ dùng khi `kieu_go == Telex`).
* Không breaking change: `mac_dinh()` vẫn chọn Telex.

## Tác động hiệu năng

* Enum match thay vì method call: zero-cost.
* Không allocation thêm trong hot path.
* VNI modifier loop cùng độ phức tạp Telex.

## Tác động serde

* `KieuGo` derive serde. Mặc định `Telex`.
* Deserialize không tạo cấu hình invalid.

## Tác động no_std

* Không thay đổi. VNI dùng `alloc` như Telex.

## Migration

* `CauHinh::mac_dinh()` vẫn chọn Telex → không cần đổi code.
* Muốn VNI: `c.dat_kieu_go(KieuGo::Vni)`.

## Điều kiện xem xét lại

* Khi thêm kiểu gõ thứ 3 (VIQR, tự điển): xem xét internal trait.
* Khi cần chuyển kiểu gõ giữa phiên (chưa có use case).
