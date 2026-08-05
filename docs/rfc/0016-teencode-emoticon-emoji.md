# RFC 0016 — Teencode, lặp ký tự, và emoticon

Trạng thái: Chấp thuận — Phase 3 (đã triển khai).

## Vấn đề

Teencode và emoticon dùng ký tự lặp và dấu câu theo cách không phải âm tiết
Việt. Nếu Telex áp dụng mù quáng, `brooooo` → `brồoooo`, `=))))` → bị biến đổi.
Cadence phải bảo toàn hơn là sửa.

## Quyết định

### Teencode lặp (`phan_doan::la_teencode_lap`)

Run 3+ chữ cái hình chữ doubled-base (`a`/`e`/`o`/`d`) giống nhau liên tiếp,
bắt đầu sau một ký tự khác trong đoạn → bảo toàn raw:

```text
"brooo"  → "brooo"   (raw — lặp có chữ khác trước)
"ooo"    → "oo"      (escape Phase 2 — nguyên đoạn, không phải teencode lặp)
"aa"     → "â"       (Telex — shape transform bình thường)
```

Tiếng Việt không có nguyên âm/phụ âm doubled-base lặp 3+, nên rule chỉ chạm
teencode, không phá âm tiết hợp lệ.

### Emoticon (`ngu_canh::nhan_emoticon`)

Mẫu `=)`+, `:)`, `:D`, `:P`, `:v`, `:3`, `;)`, `^^`, `???`, `!!!`, `...` lặp
→ `bat_buoc_raw`. Các chữ trong `:v`/`:D`/`:P` bị khóa raw.

### Emoji (`phan_doan::LoaiDoan::Emoji`)

Non-ASCII không phải chữ Việt (emoji, combining mark, dấu câu Unicode) được
phân loại `Emoji` → render nguyên bản, không qua Telex.

## Thứ tự ưu tiên

```text
1. them_nguyen_ban       → raw (ranh giới đoạn)
2. nhận diện ngữ cảnh     → raw (RFC 0015)
3. teencode lặp           → raw (trước lua_chon)
4. lua_chon cục bộ         → Telex hoặc raw (RFC 0015)
```

## Phương án bị loại

* **Giới hạn cứng ký tự lặp**: bị loại — `gioi_han_thao_tac` đã giới hạn tổng
  thao tác; không cần rule riêng.
* **Sửa teencode thành chuẩn**: bị loại — vi phạm "không phán xét chính tả".
* **Từ điển teencode**: bị loại — phình binary, không cần thiết.

## Tác động public API

Không thay đổi. Logic nội bộ `pub(crate)`.
