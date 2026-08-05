# RFC 0014 — Phân đoạn và ý định nội dung

Trạng thái: Chấp thuận — Phase 3 (đã triển khai).

## Vấn đề

Phase 2 xử lý toàn bộ lịch sử raw như một đoạn Telex duy nhất. Phím dấu thanh
và phím hình chữ được áp dụng xuyên qua ranh giới từ, nên nội dung kỹ thuật trộn
tiếng Việt bị hỏng (xem RFC 0013). Cần chia lịch sử thành các đoạn cục bộ và
quyết định xử lý từng đoạn độc lập.

## Quyết định

Module `phan_doan.rs` chia lịch sử raw thành `Vec<Doan>`, mỗi đoạn là một run
liên tục các thao tác cùng [`LoaiDoan`].

### `LoaiDoan`

```text
Chu          — ASCII letters, chữ Việt dựng sẵn, [ ] (DayDu). Ứng viên Telex.
So           — chữ số ASCII. Render nguyên bản.
KhoangTrang  — khoảng trắng ASCII. Render nguyên bản.
DauCau       — . , ! ? ; ' " ( ) - _  Render nguyên bản.
KyThuat      — : / \ @ # $ % ^ & * + = < > { } | ` ~  Ranh giới mạnh.
Emoji        — non-ASCII không phải chữ Việt (emoji, combining mark). Nguyên bản.
NguyenBan    — ký tự do them_nguyen_ban. Luôn ranh giới đoạn.
```

### Quy tắc phân đoạn

1. Mỗi thao tác được phân loại theo ký tự + `CachNhap` + `KieuTelex`.
2. Các thao tác cạnh nhau cùng `LoaiDoan` được gộp thành một `Doan`.
3. `them_nguyen_ban` luôn là `NguyenBan` bất kể ký tự — tạo ranh giới mạnh.
4. `DayDu`: `[` và `]` được xếp `Chu` vì chúng sinh `ư`/`ơ` qua Telex.

### Ý định nội dung

Mỗi đoạn có một ý định loại trừ nhau, suy luận từ `LoaiDoan` và nhận diện ngữ
cảnh (RFC 0015):

```text
TiengViet         — đoạn Chu có thể là âm tiết Việt.
KyThuat           — URL, email, path, code, identifier, command.
TuDo              — teencode, emoticon, ký tự lặp, chưa đủ bằng chứng.
NguyenBanBatBuoc  — them_nguyen_ban hoặc restore raw.
```

Chỉ `TiengViet` được biến đổi Telex; mọi ý định khác render nguyên bản.

## Teencode lặp

`la_teencode_lap()` phát hiện run 3+ chữ cái hình chữ doubled-base (`a`/`e`/
`o`/`d`) giống nhau liên tiếp, bắt đầu sau một ký tự khác trong đoạn:

```text
"ooo"  (nguyên đoạn) → escape Telex → "oo"  (giữ behavior Phase 2)
"brooo"(lặp có chữ khác trước) → bảo toàn raw → "brooo"
```

Tiếng Việt không có nguyên âm/phụ âm doubled-base lặp 3+, nên rule chỉ chạm
teencode/nước ngoài, không phá âm tiết Việt hợp lệ.

## Tác động hiệu năng

Phân đoạn là quét tuyến tính O(n). Mỗi đoạn chạy Telex trên slice cục bộ; tổng
chi phí không tệ hơn Phase 2 chạy Telex trên toàn history.

## Tác động `no_std`

Không thêm dependency. Dùng `alloc::vec::Vec` (đã có).

## Tác động public API

Không thay đổi public API. `phan_doan` là `pub(crate)`.
