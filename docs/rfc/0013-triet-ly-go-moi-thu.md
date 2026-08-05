# RFC 0013 — Triết lý "Gõ mọi thứ bạn cần"

Trạng thái: Chấp thuận — Phase 3.

## Vấn đề

Phase 2 xử lý toàn bộ lịch sử raw như **một đoạn Telex duy nhất**. Phím dấu
thanh và phím hình chữ được áp dụng xuyên qua ranh giới từ, nên nội dung kỹ
thuận trộn tiếng Việt bị hỏng:

```text
raw:     cargo build lỗi rồi
telex:   cảrgo build lỗỉ ồi   (sai — `r` trong cargo thành dấu hỏi)
kỳ vọng: cargo build lỗi rồi
```

Người dùng không thể bật/tắt bộ gõ liên tục khi chuyển giữa tiếng Việt, code,
URL, command và chat. Cadence phải tự quyết định **đoạn nào** được biến đổi
Telex, **đoạn nào** được giữ nguyên — mà không phán xét nội dung.

## Quyết định

Triết lý chính thức:

```text
Cadence — Gõ mọi thứ bạn cần.
```

Nguyên tắc xử lý:

```text
Hiểu rõ thì biến đổi.
Không chắc thì bảo toàn.
Đã biến đổi thì phải hoàn tác được.
Raw input không bao giờ bị mất.
```

Cadence chỉ trả lời một câu hỏi cho mỗi đoạn:

```text
Có đủ bằng chứng để áp dụng biến đổi Telex vào đoạn này không?
```

Không biến mọi `s`/`f`/`r`/`x`/`j`/`w` thành modifier một cách mù quáng. Quyết
định thuộc về **đoạn cục bộ**, không thuộc toàn phiên.

## Bất biến

* Raw input trong `noi_dung_goc()` giữ byte-for-byte mọi thứ người dùng nhập.
* Replay deterministic: cùng cấu hình + lịch sử + con trỏ cho cùng snapshot và
  cùng lý do lựa chọn.
* Không phán xét chính tả, teencode, tiếng lóng, code, command hay chat.
* Không dùng từ điển, AI, network, async, OS API, regex trong hot path.
* Mỗi `PhienGo` độc lập; không global mutable state.

## Quyết định kiến trúc

1. **Phân đoạn** (RFC 0014): lịch sử raw được chia thành các đoạn theo loại ký
   tự (chữ, số, khoảng trắng, dấu câu, kỹ thuật, emoji, nguyên bản).
2. **Ý định nội dung** (RFC 0014): mỗi đoạn có một ý định loại trừ nhau
   (`TiengViet`/`KyThuat`/`TuDo`/`NguyenBanBatBuoc`).
3. **Bằng chứng lựa chọn** (RFC 0015): quyết định dựa trên enum bằng chứng, không
   dùng điểm số floating-point.
4. **Nhận diện kỹ thuật** (RFC 0015–0016): identifier, URL, email, đường dẫn,
   command, code fence/span được nhận qua cấu trúc, không qua từ điển.
5. **Teencode/emoticon/emoji** (RFC 0017): bảo toàn hơn là sửa; ký tự lặp chỉ
   bị giới hạn bởi `gioi_han_thao_tac`.
6. **Chính sách lựa chọn** (RFC 0018): `TuNhien`/`UuTienTiengViet`/
   `UuTienNguyenBan` thay cho mười boolean.
7. **Trace** (RFC 0019): giải thích quyết định có cấu trúc, không phải logging.

## Thứ tự ưu tiên lựa chọn (khung)

```text
1. them_nguyen_ban / restore raw bắt buộc   → raw
2. code fence / code span                    → raw
3. URL / email / path chắc chắn              → raw
4. emoticon / emoji                           → raw
5. số / version / hash / UUID                 → raw
6. âm tiết tiếng Việt hoàn chỉnh             → Telex
7. biến đổi hình chữ rõ                       → Telex (khi không xung đột kỹ thuật)
8. dấu thanh mơ hồ                            → Telex (khi parser còn khả năng thành Việt)
9. không đủ bằng chứng                        → raw
```

Chi tiết và override được tài liệu hóa trong RFC 0018.

## Ví dụ đúng

```text
tieengs          → tiếng        (âm tiết Việt hoàn chỉnh)
ddm              → đm            (hình chữ rõ `dd`→`đ`)
cargo build      → cargo build   (đoạn kỹ thuật, raw)
https://x.com    → https://x.com (cấu trúc URL)
=))))            → =))))         (emoticon, raw)
brooooo          → brooooo       (ký tự lặp, raw)
```

## Phản ví dụ

```text
async   → áync        (sai — identifier phải giữ raw)
class   → clá         (sai — `cl` onset không hợp lệ, raw)
cargo   → cảrgo       (sai — tone xuyên ranh giới từ)
```

## Phương án bị loại

* **Một cờ `mode_code` cho toàn phiên**: bị loại — không cho phép code trộn
  tiếng Việt trong cùng phiên; vi phạm "chuyển context trong phiên".
* **N boolean `la_code`/`la_url`/...**: bị loại — phình API, khó bảo trì, trạng
  thái không loại trừ nhau.
* **Điểm số `f32`/`f64`**: bị loại — không deterministic trực quan, khó audit.
* **Từ điển keyword hàng nghìn phần tử**: bị loại — vi phạm "không dùng từ
  điển", nặng, không cần thiết khi nhận cấu trúc đủ.
* **Regex trong hot path**: bị loại — vi phạm chính sách hot path.

## Tác động hiệu năng

Phân đoạn là quét tuyến tính O(n). Mỗi đoạn chạy Telex trên slice cục bộ; tổng
chi phí không tệ hơn Phase 2 chạy Telex trên toàn history. Không có allocation
mỗi rule ngoài `Vec` đã có. Token bình thường vẫn mức microsecond (xem
benchmark Phase 3).

## Tác động `no_std`

Không thêm runtime dependency. Vẫn dùng `alloc` (đã có). Trace dùng `cfg`.

## Tác động public API

* Thêm `ChinhSachLuaChon` vào `CauHinh` (getter/setter, mặc định `TuNhien`).
* Thêm feature `trace` thực sự (trước đây là no-op).
* Không breaking change: mọi API Phase 1–2 giữ nguyên.

## Điều kiện xem xét lại

* Nếu nhận cấu trúc không đủ cho một lớp input thực tế lớn, thêm danh sách
  keyword nhỏ kèm benchmark chứng minh và RFC riêng.
* Nếu trace overhead vượt ngân sách, tách trace thành snapshot immutable riêng.
