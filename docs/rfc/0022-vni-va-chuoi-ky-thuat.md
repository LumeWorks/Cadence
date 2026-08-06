# RFC 0022 - VNI và chuỗi kỹ thuật

Trạng thái: Chấp thuận - 2026.1.0 (đã triển khai).

## Vấn đề

VNI dùng digit `1..=9` làm modifier, nhưng digit cũng xuất hiện trong chuỗi
kỹ thuật (`sha256`, `h264`, `v1.2.3`, `127.0.0.1`). Cadence phải bảo toàn
chuỗi kỹ thuật raw, không biến đổi digit thành modifier VNI vô nghĩa.

## Bất biến

* Chuỗi kỹ thuật (hash, version, IP, architecture, identifier có số) luôn raw.
* Tiếng Việt có số modifier (`tieng1` → `tiếng`) vẫn biến đổi khi hợp lệ.
* Phân biệt bằng cấu trúc, không từ điển.
* Sau đoạn kỹ thuật, VNI hoạt động lại trong đoạn tiếng Việt tiếp theo.
* Raw không bao giờ mất.

## Quyết định

### Phân đoạn VNI

Trong chế độ VNI, digit `1..=9` được phân loại là `LoaiDoan::Chu` (ứng viên
modifier), không phải `So`. Digit `0` vẫn là `So` (không phải modifier VNI).

### Nhận diện chuỗi số kỹ thuật

Rule A: đoạn `Chu` có 2+ digit VNI AND phần chữ (bỏ digit) là âm tiết
`KhongHopLe` → raw. (vd `sha256`: bỏ digit → `sha`, `sha` không hợp lệ → raw.)

Rule B: đoạn `Chu` có 2+ digit VNI AND kề `So` hoặc `DauCau`(`.``-`) → raw.
(vd `v1` trong `v1.2.3`: kề `.` → raw.)

Không dùng từ điển `sha256`/`h264`/`ipv6`. Nhận diện bằng cấu trúc.

### Ngữ cảnh Phase 3

VNI modifier chỉ kích hoạt khi:
* Đoạn được phân loại là `Chu` (chữ tự nhiên).
* Có nguyên âm/chữ phù hợp để modifier tác động.
* Không bị buộc raw bởi nhận diện ngữ cảnh.
* Người gọi không dùng `them_nguyen_ban`.

Raw thắng khi:
* Chuỗi giống version/hash/identifier có số.
* Chuỗi có dấu phân cách kỹ thuật.
* Số không thể tác động hợp lệ lên chữ Việt.

## Rule table

| Input | Output | Lý do |
|-------|--------|-------|
| `sha256` | sha256 | 2+ digit, `sha` KhongHopLe |
| `h264` | h264 | 2+ digit, `h2` không có nguyên âm |
| `v1.2.3` | v1.2.3 | kề `.` → raw |
| `127.0.0.1` | 127.0.0.1 | So + DauCau |
| `x86_64` | x86_64 | 2+ digit, `x` không onset hợp lệ |
| `tieng1` | tiếng | 1 digit, `tieng` CoTheTiepTuc |
| `toi6` | tôi | 1 digit, `toi` CoTheTiepTuc |
| `user123` | user123 | 2+ digit, `user` KhongHopLe |

## Ví dụ

```
toi6_dang_fix_h264 → tôi_dang_fix_h264
sha256 bi loi64 → sha256 bi lỗ
user123 cua toi6 → user123 cua tôi
```

## Phản ví dụ

* Danh sách `sha256`/`h264`/`ipv6` trong code — bị loại, dùng cấu trúc.
* Cờ `dang_go_code` — bị loại, phá độc lập đoạn.

## Phương án bị loại

* **Từ điển kỹ thuật**: bị loại — policy không dùng từ điển lớn.
* **Cờ session**: bị loại — phá "Gõ mọi thứ bạn cần".
* **Luôn raw khi có digit**: bị loại — `tieng1` phải biến đổi.

## Tác động public API

Không thêm API mới.

## Tác động hiệu năng

Nhận diện cấu trúc O(n) trên độ dài đoạn, không allocation thêm.

## Tác động serde / no_std

Không thay đổi.

## Điều kiện xem xét lại

* Khi gặp false-positive/negative trong thực tế.
* Khi cần pattern kỹ thuật đặc thù không giải quyết được bằng cấu trúc.
