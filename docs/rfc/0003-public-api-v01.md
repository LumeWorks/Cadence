# RFC 0003 — Public API v0.1

Trạng thái: Chấp thuận — Phase 1.

## Vấn đề

Public surface phải đủ nhỏ để Phase 2 thêm Telex mà không phá API, nhưng đủ lớn để
host application tích hợp được.

## Quyết định

Public API tối thiểu Phase 1:

```rust
pub struct BoGo;
pub struct PhienGo;
pub struct CauHinh;
pub struct BanChupSoan;
pub struct ViTriVanBan;

pub enum KetQuaXuLy { KhongDoi, CapNhat, ChapNhan { noi_dung: String } }
pub enum LoaiNoiDung { Trong, NguyenBan }
pub enum LoiCauHinh { GioiHanThaoTacKhongHopLe { gioi_han, toi_thieu, toi_da } }
```

* `BoGo` — factory bất biến; `tao_phien` trả phiên độc lập.
* `PhienGo` — stateful, giữ lịch sử private.
* `BanChupSoan` — snapshot chỉ đọc.
* `ViTriVanBan` — vị trí con trỏ theo byte/UTF-16/grapheme.
* `KetQuaXuLy` — kết quả thao tác; **không** trả snapshot clone trong `CapNhat`;
  người gọi lấy snapshot qua `ban_chup()`.

Quy ước đặt tên: identifier domain tiếng Việt không dấu; comment tiếng Việt có dấu.

## Lý do

* Surface nhỏ giảm rủi ro breaking change khi Phase 2 thêm Telex.
* Snapshot chỉ đọc tránh host sửa state nội bộ.
* `KetQuaXuLy::ChapNhan` mang nội dung commit (chuỗi cuối), không mang diff —
  Cadence không yêu cầu host "xóa N ký tự".

## Phương án bị loại

* **Trả diff "hãy xóa N ký tự":** bị loại — gắn chặt host với state render, khó
  dùng trên nền tảng chỉ nhận chuỗi.
* **`ChuyenTiep` trong `KetQuaXuLy`:** bị loại cho Phase 1 vì Cadence chỉ nhận ký
  tự thuộc đoạn composition, chưa có khái niệm chuyển tiếp. Thêm sau nếu có hành
  vi thật và test rõ.
* **Expose `Vec<ThaoTacNhap>`:** bị loại — rò chi tiết nội bộ.
* **Getter/setter máy móc:** bị loại; chỉ expose method có ý nghĩa (ví dụ
  `ban_chup()`, `dang_trong()`).

## Bất biến

* `chap_nhan()` phiên rỗng trả `KhongDoi`; phiên có nội dung trả `ChapNhan` rồi
  reset, không rò state.
* Hai phiên từ cùng `BoGo` hoàn toàn độc lập.
* `noi_dung == noi_dung_goc` trong Phase 1.

## Tác động tới Phase sau

* Phase 2 có thể thêm variant `LoaiNoiDung::Telex` và trạng thái Telex bên trong
  `PhienGo` mà không phá chữ ký method hiện có.
* Nếu cần `ChuyenTiep`, thêm vào `KetQuaXuLy` cùng test semantic.
