# RFC 0024 - Phiên bản Cadence

Trạng thái: Chấp thuận - 2026.1.0 (đã triển khai).

## Vấn đề

Cadence cần hệ phiên bản rõ ràng. Hệ thống phải tương thích Cargo (ba
thành phần số) nhưng semantic là calendar/change/patch, không phải
major/minor/patch truyền thống.

## Bất biến

* Cú pháp ba thành phần tương thích Cargo: `<năm>.<thay đổi>.<vá>`.
* Tag nội bộ `v0.1.0` giữ nguyên, không xóa, không di chuyển.
* Bản phát hành công khai đầu tiên là `2026.1.0`.
* Không tuyên bố chưa được chứng minh.

## Quyết định

### Thành phần thứ nhất — năm

Năm phát hành. Khi sang năm mới, số thay đổi bắt đầu lại từ `1`.

### Thành phần thứ hai — phiên bản thay đổi

Tăng khi có thay đổi có ý nghĩa đối với người dùng hoặc người tích hợp:
* Thêm kiểu gõ.
* Thêm tính năng.
* Thay đổi hành vi.
* Thay đổi public API.
* Thay đổi policy lựa chọn.
* Thay đổi contract tích hợp.

### Thành phần thứ ba — bản vá

Tăng khi chỉ có:
* Sửa bug.
* Security fix tương thích.
* Performance fix không đổi contract.
* Documentation fix.
* Packaging fix.

## Ví dụ

```
2026.1.0  Telex + VNI, bản public đầu tiên
2026.1.1  patch bug
2026.2.0  thay đổi lớn tiếp theo trong 2026
2027.1.0  bản đầu tiên năm 2027
```

## Phản ví dụ

* `0.2.0` — bị loại: không theo calendar semantic.
* `1.0.0` — bị loại: không theo calendar semantic.

## Phương án bị loại

* **SemVer truyền thống**: bị loại — Cadence phát hành theo năm, không theo
  breaking change frequency.
* **Date-based (2026.08.06)**: bị loại — quá chi tiết, không phân biệt
  change/patch.

## Tác động public API

Không thêm API mới. `Cargo.toml` version = `2026.1.0`.

## Tác động hiệu năng / serde / no_std

Không thay đổi.

## Migration

* Git dependency trên `v0.1.0` → đổi sang `2026.1.0`.
* `0.1.0` chưa publish crates.io, nên không có registry conflict.

## Điều kiện xem xét lại

* Khi cần pre-release (alpha/beta): dùng `2026.1.0-alpha.1`.
* Khi cần metadata: dùng `2026.1.0+build.123`.
