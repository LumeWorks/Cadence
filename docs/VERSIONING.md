# Phiên bản Cadence

Cadence sử dụng hệ phiên bản calendar-based:

```
<năm>.<số phiên bản thay đổi>.<số phiên bản vá>
```

Cú pháp ba thành phần tương thích Cargo, nhưng semantic là
calendar/change/patch, không phải major/minor/patch truyền thống.

## Thành phần thứ nhất — năm

Năm phát hành. Khi sang năm mới, số phiên bản thay đổi bắt đầu lại từ `1`.

```
2026.1.0
2027.1.0
```

## Thành phần thứ hai — phiên bản thay đổi

Tăng khi có thay đổi có ý nghĩa đối với người dùng hoặc người tích hợp:

* Thêm kiểu gõ.
* Thêm tính năng.
* Thay đổi hành vi.
* Thay đổi public API.
* Thay đổi policy lựa chọn.
* Thay đổi contract tích hợp.

```
2026.1.0  Telex + VNI, bản public đầu tiên
2026.2.0  thay đổi lớn tiếp theo trong 2026
```

## Thành phần thứ ba — bản vá

Tăng khi chỉ có:

* Sửa bug.
* Security fix tương thích.
* Performance fix không đổi contract.
* Documentation fix.
* Packaging fix.

```
2026.1.1
2026.1.2
```

## Mốc nội bộ

Tag nội bộ `v0.1.0` là mốc khóa lõi trước phát hành công khai, không phải
bản crates.io đầu tiên. Tag được giữ nguyên, không xóa, không di chuyển.

## Bản phát hành công khai đầu tiên

```
2026.1.0
```

Tag: `v2026.1.0`

## Tại sao không dùng SemVer truyền thống

Cadence phát hành theo năm lịch. SemVer truyền thống (major/minor/patch)
không phản ánh đúng nhịp phát hành calendar-based. Hệ calendar/change/patch
cho phép người dùng biết ngay năm và mức độ thay đổi.
