# Nguồn nghiên cứu Phase 2

Tài liệu tổng hợp các nguồn tham khảo cho Phase 2 Telex engine, Unicode
normalization, và âm tiết tiếng Việt.

## Telex

* **VietKey / unikey** - phương pháp gõ Telex phổ biến nhất. Quy ước:
  `aa`→â, `aw`→ă, `ee`→ê, `oo`→ô, `ow`→ơ, `uw`→ư, `dd`→đ, `s`→sắc,
  `f`→huyền, `r`→hỏi, `x`→ngã, `j`→nặng, `z`→xóa dấu.
* **GoTiếng Việt (gov.vn)** - bộ gõ chính thức, hành vi Telex tương tự.
* **OpenKey** - open source, tham khảo cách xử lý escape (lặp phím).
* **Escape rule**: lặp đúng phím modifier đang hoạt động → hiện literal
  (vd: `ass`→`as`, `aww`→`aw`, `ddd`→`dd`).

## Âm tiết tiếng Việt

* **Thompson, Laurence C. (1965).** *A Vietnamese Reference Grammar* - mô hình
  âm tiết: onset + rhyme (vowel + coda). Bảng âm đầu, âm cuối.
* **Nguyễn Đình Hòa (1997).** *Vietnamese - Tiếng Việt không son phấn* -
  quy tắc đặt dấu thanh trên nguyên âm chính.
* **Emeneau, M.B. (1951).** *Studies in Vietnamese (Annamese) Grammar* -
  phân tích vần tiếng Việt.
* **Đoàn Thiện Thuật (2003).** *Tiếng Việt - Những vấn đề ngữ âm, chữ viết*
  - bảng âm đầu 22+ đơn vị, âm cuối 8 đơn vị.

### Bảng âm đầu (onset)

```
ngh, ng, nh, gh, gi, kh, ph, th, tr, qu, ch, b, c, d, đ, g, h,
k, l, m, n, p, q, r, s, t, v, x
```

### Bảng âm cuối (coda)

```
ch, ng, nh, c, m, n, p, t
```

### Quy tắc đặt dấu thanh

1. **Bán âm cuối**: `i`/`u` cuối câu không mang dấu thanh → dấu trên
   nguyên âm trước (vd: `tiếng`, `chúi`).
2. **On-glide `o`+`a`/`e`**: quy tắc hiện đại đặt trên `o` (`hóa`, `dóe`),
   truyền thống đặt trên `a`/`e` (`hoá`, `doé`).
3. **Tam nguyên âm `ươ`**: dấu trên `ơ` (`người`, `mười`).

## Unicode normalization

* **Unicode Standard Annex #15.** *Unicode Normalization Forms* - NFC, NFD,
  NFKC, NFKD.
* **Unicode Code Charts: Latin Extended Additional (U+1E00–U+1EFF)** -
  các ký tự tiếng Việt precomposed (â, ă, ê, ô, ơ, ư, đ + dấu thanh).
* **unicode-normalization crate (rust)** - dùng cho NFD fallback, kiểm tra
  canonical equivalence. MSRV 1.85 compatible.

### Bảng tổ hợp tiếng Việt

* Nguyên âm nền: `a`, `e`, `i`, `o`, `u`, `y`.
* Dấu chữ: Breve (ă), Circumflex (â/ê/ô), Horn (ơ/ư), Stroke (đ).
* Dấu thanh: Sắc (U+0301), Huyền (U+0300), Hỏi (U+0309), Ngã (U+0303),
  Nặng (U+0323).
* Tổng tổ hợp: 17 nguyên âm nền × 5 dấu thanh × (1–3 dấu chữ) = ~134
  codepoint precomposed trong U+1E00–U+1EFF.

## no_std + alloc

* **Rust Embedded Working Group** - best practices cho `no_std + alloc`.
* `unicode-normalization` 0.1.25: no_std compatible, dùng `alloc`.
* `unicode-segmentation` 1.x: no_std compatible, đã dùng từ Phase 1.

## Tham khảo code

* **Rust standard library** - `char::is_alphabetic`, `str::is_char_boundary`.
* **unicode-segmentation** - `UnicodeSegmentation::graphemes()` cho cursor
  boundary.
