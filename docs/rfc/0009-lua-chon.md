# RFC 0009 — Lựa chọn raw vs Telex

Trạng thái: Chấp thuận — Phase 2 (đã triển khai).

## Vấn đề

Sau khi Telex engine biến đổi raw, Cadence cần quyết định: giữ kết quả Telex hay
fallback về raw? Sai quyết định dẫn đến output sai chính tả.

## Quyết định

Module `lua_chon.rs` triển khai `lua_chon() -> KetQuaLuaChon` với quy tắc:

### Quy tắc 1: Shape transform → Telex
Nếu kết quả Telex có shape transform (â, ă, ê, ô, ơ, ư, đ), luôn giữ Telex.

### Quy tắc 2: Escape → Telex
Nếu có escape (lặp phím modifier), luôn giữ Telex. Escape là ý định người dùng.

### Quy tắc 3: Tone + âm tiết không hợp lệ → raw
Nếu chỉ có tone transform (không shape, không escape), parse Telex output:
* `MucHopLe::KhongHopLe` → fallback raw.
* `MucHopLe::CoTheTiepTuc` → giữ Telex.

### Quy tắc 4: Invalid onset → raw
Nếu raw có onset không hợp lệ (vd: `cl` trong `class`, `fl` trong `flag`),
toàn bộ token fallback về raw. Kiểm tra bằng `raw_co_onset_hop_le`.

### Quy tắc 5: `them_nguyen_ban` → raw
Bypass hoàn toàn Telex; raw chars là literal và chặn mọi quy tắc Telex tiếp theo.

## Thứ tự áp dụng

```
1. them_nguyen_ban? → raw (chặn hết)
2. escape? → telex
3. shape transform? → telex
4. invalid onset in raw? → raw
5. tone only + KhongHopLe? → raw
6. else → telex
```

## Lý do

* Shape transform luôn hợp lệ vì `â`, `ơ` v.v. là chữ tiếng Việt có thật.
* Escape thể hiện ý định người dùng muốn hiển thị literal modifier.
* Tone only có thể tạo output sai chính tả (`asf` → `ás`); cần parse để xác minh.
* Invalid onset (`cl`) không thể thành âm tiết tiếng Việt; raw an toàn hơn.

## Phương án bị loại

* **Luôn giữ Telex**: bị loại — output sai chính tả (`ás`, `ớl`).
* **Luôn fallback raw khi tone only**: bị loại — `ás` hợp lệ nếu gõ `asf` trong
  từ mượn; và `tiếng` có tone only cần giữ.
* **Từ điển để kiểm tra**: bị loại — policy không dùng từ điển.

## Bất biến

* Nếu có shape transform, không bao giờ fallback raw.
* Nếu escape, không bao giờ fallback raw.
* `them_nguyen_ban` luôn chặn Telex, bất kể nội dung.

## Tác động tới Phase sau

* Phase 3 có thể thêm quy tắc dựa trên bảng vần đầy đủ.
* Phase 3 có thể thêm tùy chọn cấu hình cho strict/loose selection.
