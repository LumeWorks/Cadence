# RFC 0015 — Nhận diện ngữ cảnh kỹ thuật và bằng chứng lựa chọn

Trạng thái: Chấp thuận — Phase 3 (đã triển khai).

## Vấn đề

Phase 2 chỉ phân tích âm tiết tại cuối lịch sử. Phase 3 chia lịch sử thành
đoạn (RFC 0014), nhưng một đoạn `Chu` đơn lẻ có thể là tiếng Việt **hoặc**
identifier/URL/email/path/code. Cần nhận diện cấu trúc kỹ thuật xuyên đoạn để
buộc raw, ngăn Telex biến `bar` (sau `::`) hay `buf` (trước `=`) thành âm tiết
Việt hợp lệ.

## Quyết định

Module `ngu_canh.rs` nhận diện cấu trúc xuyên đoạn và trả `Vec<KetQuaNhanDien>`
— mỗi đoạn có `bat_buoc_raw: bool` và `bang_chung: BangChungLuaChon`.

### `BangChungLuaChon`

Enum thay cho điểm số floating-point (xem RFC 0013). Mỗi variant là một lý do
loại trừ nhau:

```text
AmTietTiengVietHoanChinh   BienDoiHinhChuRoRang   PhimDauHopLe
PhanCachIdentifier         CauTrucUrl             CauTrucEmail
CauTrucDuongDan            CauTrucCommand         ChuoiSoKyThuat
KyTuLapChat                Emoticon               NguyenBanDoNguoiGoiYeuCau
```

### Hai pass

**Pass 1 — span structure** (tiêu thụ nhiều đoạn liên tiếp):

| Recognizer       | Tín hiệu                                        | Bằng chứng          |
|------------------|-------------------------------------------------|---------------------|
| `nhan_url`       | Chu + KyThuat `://` + (non-ws)+                 | `CauTrucUrl`        |
| `nhan_email`     | (Chu/So/dot/plus)+ + `@` + (Chu/So/dot)+        | `CauTrucEmail`      |
| `nhan_duong_dan` | `/`, `~/`, `./`, `../`, `X:\`                   | `CauTrucDuongDan`   |
| `nhan_code`      | backtick mở + backtick đóng cùng số lượng       | `CauTrucCommand`    |
| `nhan_emoticon`  | `=)`+, `:)`, `:D`, `???`, `!!!`, `...` lặp      | `Emoticon`          |

URL dùng tín hiệu mạnh `://` để tránh false-positive trên `hoaf.com`.

**Pass 2 — per-segment adjacency** (cho đoạn Chu còn tự do):

| Tín hiệu                    | Ví dụ            | Bằng chứng          |
|-----------------------------|------------------|---------------------|
| KyThuat `::` trước          | `bar` sau `foo::`| `CauTrucDuongDan`   |
| KyThuat chứa `=` trước/sau  | `buf` trước `=`  | `CauTrucCommand`    |
| KyThuat `::` sau            | `foo` trước `::` | `CauTrucDuongDan`   |

`tim_ky_thuat_truoc`/`tim_ky_thuat_sau` bỏ qua KhoangTrang khi tìm KyThuat
kề cận.

### Luật raw cục bộ (module `lua_chon.rs`)

Ngoài ngữ cảnh xuyên đoạn, mỗi đoạn Chu chạy Telex rồi kiểm tra:

```text
Rule 0: 2+ dấu thanh trong một đoạn → raw (không phải âm tiết Việt).
Rule 1: shape transform + onset không hợp lệ → raw (foo→foo, f không là onset).
Rule 2: shape transform + onset hợp lệ → Telex (ddm→đm).
Rule 3: onset raw không hợp lệ + không escape hình chữ → raw (class→class).
Rule 4: escape hình chữ/dấu thanh → Telex (ý định người dùng).
Rule 5: chỉ tone + âm tiết không hợp lệ → raw (async→async).
```

### Luật nucleus-glide (module `am_tiet.rs`)

Hai nguyên âm đầy (a/ă/â/e/ê/ô/ơ) không có glide {i,u,ư,y,o} → `KhongHopLe`.
Ngăn `ae` (CASE) hay `oe` thành âm tiết giả.

## Phương án bị loại

* **Regex**: bị loại — vi phạm chính sách hot path, không `no_std` dễ dàng.
* **Từ điển keyword**: bị loại — phình binary, vi phạm "không dùng từ điển".
* **Điểm số f32/f64**: bị loại — không deterministic trực quan, khó audit.
* **Nhận diện sau Telex (cuối pipeline)**: bị loại — `bar` sau `::` đã thành
  `bả` trước khi kiểm tra; phải nhận **trước** khi chạy Telex.

## Tác động hiệu năng

Nhận diện là quét tuyến tính hai pass trên `Vec<Doan>` (đã nhỏ hơn history).
Không allocation ngoài `Vec<String>` raws (một lần mỗi rebuild).

## Tác động `no_std`

Không thêm dependency. `ngu_canh` là `pub(crate)`, không lộ public API.

## Tác động public API

Không thay đổi. `KetQuaNhanDien` và `BangChungLuaChon` là `pub(crate)`.
