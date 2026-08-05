# Kiến trúc Cadence

Cadence là một Rust library crate duy nhất đóng vai trò lõi gõ tiếng Việt.
Tài liệu này mô tả kiến trúc Phase 1 và các quyết định nền móng.

## Mục tiêu Phase 1

Phase 1 chỉ dựng **nền móng bất biến**:

* Nhận và giữ nguyên mọi ký tự người dùng nhập.
* Duy trì lịch sử thao tác không phá hủy.
* Hỗ trợ con trỏ trong đoạn đang soạn.
* Hỗ trợ thêm, xóa lùi, xóa phía trước, di chuyển, commit, reset.
* Luôn giữ chính xác nội dung gốc.
* Snapshot trung lập nền tảng (byte, UTF-16, grapheme).
* Hoạt động đúng với Unicode bất kỳ.
* Compile được với `std` và `no_std + alloc`.

Telex **chưa** được triển khai. Mọi ký tự được render nguyên bản.

## Sơ đồ lớp

```text
BoGo (factory bất biến)
  └─ CauHinh (cấu hình, field private)
       ↓ tao_phien
  PhienGo (phiên soạn thảo, stateful)
       ├─ lịch sử thao tác (nguồn sự thật)
       ├─ con trỏ nội bộ (nằm giữa các thao tác)
       ├─ snapshot hiện tại (dựng lại từ lịch sử)
       └─ buffer render tái sử dụng
            ↓ ban_chup
       BanChupSoan (snapshot trung lập nền tảng)
            ├─ noi_dung / noi_dung_goc
            ├─ con_tro: ViTriVanBan (byte/utf16/grapheme)
            └─ loai_noi_dung: LoaiNoiDung
```

## Nguồn sự thật

Lịch sử thao tác là nguồn sự thật duy nhất. Snapshot **luôn** được dựng lại
từ lịch sử sau mỗi thay đổi. Không giữ chuỗi render làm nguồn chính, không
chỉnh trực tiếp `String` rồi đồng bộ ngược.

## Replay

Pipeline replay Phase 1:

```text
lịch sử thao tác → render nguyên bản → tính vị trí con trỏ → cập nhật snapshot
```

Replay toàn đoạn sau mỗi thay đổi là thiết kế chính thức Phase 1. Không có
cache incremental phức tạp.

## Không gian không thuộc core

Cadence không chứa FFI, GUI, IPC, network, thread, async runtime, hay logic
nhận diện ứng dụng. Đó là vai trò của LCand (Linux) và WCand (Windows).

## RFC

Các quyết định kiến trúc quan trọng được ghi tại:

* [`rfc/0001-kien-truc-loi.md`](rfc/0001-kien-truc-loi.md)
* [`rfc/0002-lich-su-thao-tac.md`](rfc/0002-lich-su-thao-tac.md)
* [`rfc/0003-public-api-v01.md`](rfc/0003-public-api-v01.md)
* [`rfc/0004-unicode-va-con-tro.md`](rfc/0004-unicode-va-con-tro.md)
* [`rfc/0005-gioi-han-phien.md`](rfc/0005-gioi-han-phien.md)

## Dependency

| Dependency | Loại | Mục đích |
|---|---|---|
| `unicode-segmentation` | runtime | Tính grapheme cluster chính xác. |
| `serde` | optional | Derive serialization cho public data type (chỉ khi bật feature `serde`). |
| `proptest` | dev | Property test bất biến nền tảng. |
| `criterion` | dev | Benchmark nền. |

Chi tiết chính sách dependency xem `CONTRIBUTING.md`.
