# RFC 0008 — Phân tích âm tiết tiếng Việt

Trạng thái: Chấp thuận — Phase 2.

## Vấn đề

Khi chỉ có dấu thanh (không có shape transform), Cadence cần kiểm tra xem Telex
output có phải âm tiết tiếng Việt hợp lệ hay không. Nếu không hợp lệ (vd: `asf`
→ `ás` không hợp lệ vì `s` là phụ âm không thuộc coda), fallback về raw.

## Quyết định

Module `am_tiet.rs` triển khai parser âm tiết với:

### Bảng âm đầu (onset)

```
ngh, ng, nh, gh, gi, kh, ph, th, tr, qu, ch, b, c, d, đ, g, h,
k, l, m, n, p, q, r, s, t, v, x
```

Sắp xếp theo độ dài giảm để match prefix dài trước.

### Bảng âm cuối (coda)

```
ch, ng, nh, c, m, n, p, t
```

### Parser `phan_tich_am_tiet(s) -> MucHopLe`

1. Tách âm đầu (longest prefix match).
2. Sau âm đầu, ký tự tiếp phải là nguyên âm.
3. Tách âm cuối (longest suffix match).
4. Vần (giữa onset và coda) phải chỉ chứa nguyên âm.

Trả về `MucHopLe::CoTheTiepTuc` (hợp lệ hoặc có thể tiếp tục) hoặc
`MucHopLe::KhongHopLe` (không thể thành âm tiết).

### Helper cho selection

* `bat_dau_onset_hop_le(s)`: kiểm tra xem chuỗi bắt đầu bằng onset hợp lệ (không
  yêu cầu vowel theo sau). Dùng cho escape `dd` (bắt đầu bằng `d` hợp lệ).
* `raw_co_onset_hop_le(raw)`: kiểm tra raw có onset hợp lệ và theo sau là nguyên
  âm. Dùng cho tone selection: `cl` trong `class` → onset `c` hợp lệ nhưng theo
  sau là `l` (phụ âm) → không hợp lệ → raw.

## Lý do

* Parser tĩnh không cần từ điển, phù hợp `no_std + alloc`.
* Phase 2 chỉ kiểm tra cấu trúc (onset + vowel + coda), không kiểm tra bảng vần
  đầy đủ. `CoTheTiepTuc` cho mọi vần chỉ chứa nguyên âm.
* Kiểm tra chi tiết hơn (bảng vần) dành cho Phase 3.

## Phương án bị loại

* **Từ điển vần đầy đủ**: bị loại cho Phase 2 — phức tạp, tăng kích thước.
* **Regex**: bị loại — policy không dùng regex.
* **Không parse, luôn giữ Telex**: bị loại — `asf` → `ás` sai chính tả, cần
  fallback raw.

## Bất biến

* Onset rỗng hợp lệ (âm tiết bắt đầu bằng nguyên âm).
* Coda rỗng hợp lệ (vần mở).
* `bat_dau_onset_hop_le("")` trả `true` (chuỗi rỗng có thể tiếp tục).

## Tác động tới Phase sau

* Phase 3 có thể thêm `MucHopLe::HoanChinh` khi có bảng vần đầy đủ.
* Phase 3 có thể phân tích thanh điệu (tone register) cho quy tắc đặt dấu
  truyền thống.
