# RFC 0007 — Unicode NFC/NFD output

Trạng thái: Chấp thuận — Phase 2 (đã triển khai).

## Vấn đề

Cadence cần xuất chữ tiếng Việt có dấu dưới dạng Unicode. Có hai dạng chuẩn:
NFC (precomposed) và NFD (decomposed). Host cần chọn dạng phù hợp.

## Quyết định

`CauHinh` có `dang_unicode: DangUnicode` với hai giá trị:

* `DangUnicode::Nfc` (mặc định): xuất ký tự precomposed (vd: `ế` = U+1EBF).
* `DangUnicode::Nfd`: xuất ký tự decomposed (vd: `e` + `\u{0302}` + `\u{0301}`).

Module `render.rs` chứa:

* Bảng lookup NFC: ánh xạ `(ChuGoc, DauChu, DauThanh)` → codepoint
  precomposed. Bảng này phủ kín 134 tổ hợp nguyên âm × dấu chữ × dấu thanh của
  tiếng Việt.
* Hàm `render_chu()`: xuất `ChuCaiViet` thành `String` theo NFC hoặc NFD.
* Hàm `phan_tich_ky_tu()`: ánh xạ ngược từ ký tự precomposed thành
  `ChuCaiViet`, dùng khi người dùng paste hoặc `them_nguyen_ban`.

## Lý do

* NFC là dạng phổ biến nhất, tương thích hầu hết ứng dụng và font.
* NFD cần cho môi trường yêu cầu decomposed (macOS HFS+ normalization, một số
  search engine).
* Bảng lookup tĩnh nhanh hơn `unicode-normalization` crate cho tổ hợp tiếng
  Việt cụ thể; crate vẫn dùng cho NFD fallback và kiểm tra.

## Phương án bị loại

* **Chỉ NFC**: bị loại — một số môi trường cần NFD.
* **Tự normalize bằng crate cho mọi output**: bị loại — bảng tĩnh tiếng Việt
  nhanh hơn và kiểm soát được; crate chỉ dùng phụ trợ.
* **Chỉ NFD**: bị loại — hầu hết font/ứng dụng kỳ vọng NFC.

## Bất biến

* Cùng `ChuCaiViet` luôn render ra cùng chuỗi NFC (deterministic).
* NFC output luôn là một codepoint đơn cho mỗi chữ (nếu có trong bảng).
* NFD output luôn có base char + combining marks theo thứ tự chuẩn Unicode.
* `phan_tich_ky_tu` của NFC output phải trả về `ChuCaiViet` gốc (round-trip).

## Tác động tới Phase sau

* Phase 3 có thể thêm NFKC/NFKD nếu cần compatibility normalization.
* Phase 3 có thể thêm tùy chọn custom normalization cho font đặc biệt.
