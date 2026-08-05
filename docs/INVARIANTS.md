# Bất biến Cadence

Tài liệu này liệt kê các bất biến (invariants) chính thức của Cadence và liên
kết tới test chứng minh từng bất biến. Mỗi bất biến phải có ít nhất một test
ghi rõ; nếu test bị xóa, bất biến mờ đi và phải được bổ sung lại.

Quy ước đánh giá: một bất biến được "chứng minh" khi có test fail nếu bất biến
bị vi phạm. Tài liệu này không thay thế test; nó lập bản đồ bất biến ↔ test.

## 1. Raw input (byte-for-byte)

Mọi thao tác người dùng còn tồn tại phải phục hồi được byte-for-byte qua
`BanChupSoan::noi_dung_goc()`.

Không normalize raw, không đổi hoa/thường raw, không mất combining mark,
variation selector, zero-width joiner, modifier Telex, ký tự nguyên bản, ký
tự lặp.

| Test | Bất biến |
|---|---|
| `tests/property_phase3.rs::noi_dung_goc_nguyen_ven` | `noi_dung_goc == raw` với chuỗi có nghĩa |
| `tests/property.rs::them_nguyen_ban_khong_bien_doi` | `them_nguyen_ban` → `noi_dung == noi_dung_goc` |
| `tests/telex_hinh_chu.rs::aa_giu_nguyen_ban` | shape transform giữ `noi_dung_goc` nguyên |
| `tests/telex_dau_thanh.rs::asf_thay_dau_thanh_huyen` | `noi_dung_goc == "asf"` khi output `à` |
| `tests/unicode.rs::emoji_co_zero_width_joiner` | ZWJ không bị tách |
| `tests/unicode.rs::emoji_co_variation_selector` | VS16 không mất |
| `tests/unicode.rs::chuoi_co_combining_mark` | combining mark không mất |
| `tests/phan_doan.rs::nguyen_ban_tach_rieng` | `them_nguyen_ban` tạo ranh giới, raw giữ |

## 2. Determinism

Cùng `CauHinh` + lịch sử thao tác + con trỏ cho cùng `BanChupSoan` + trace.

Trace bật hay tắt không thay đổi output.

| Test | Bất biến |
|---|---|
| `tests/property_phase3.rs::deterministic` | cùng input → cùng output |
| `tests/property.rs::hai_phien_doc_lap` | replay cùng actions → identical snapshot |
| `tests/trace.rs::trace_deterministic` | cùng input → cùng trace |
| `tests/property_phase3.rs::moi_chinh_sach_url_raw` | policy không đổi output cho cấu trúc kỹ thuật |

## 3. Undo / chỉnh sửa giữa đoạn

Snapshot A → thêm một thao tác → xóa lùi → snapshot A.
Khi chèn giữa, xóa đúng thao tác vừa chèn phục hồi snapshot cũ.

| Test | Bất biến |
|---|---|
| `tests/property.rs::them_roi_xoa_lui_tra_ve_cu` | thêm rồi xóa lùi (cuối) → snapshot cũ |
| `tests/telex_con_tro.rs::backspace_roi_nhap_lai_tao_shape` | xóa rồi nhập lại tạo lại shape |
| `tests/phien_con_tro.rs::chuoi_thao_tac_chinh_sua_phuc_tap` | chuỗi chỉnh sửa phức tạp đúng |

## 4. Unicode

- Output NFC idempotent dưới NFC.
- Output NFD idempotent dưới NFD.
- Raw không bị normalize.
- Cursor byte luôn là UTF-8 boundary.
- Cursor UTF-16 không vượt số code unit.
- Cursor grapheme không nằm giữa grapheme hiển thị.
- Navigation không kẹt ở modifier vô hình.
- Emoji ZWJ không bị tách tại public cursor.

| Test | Bất biến |
|---|---|
| `tests/property.rs::nfc_nfd_canonical_equivalent` | `NFD(NFC(x)) == NFD(x)` |
| `tests/telex_config.rs::tat_ca_config_canonical_equivalent` | mọi config canonical equivalent |
| `tests/telex_nfd.rs::nfc_vs_nfd_khac_byte_nhung_tuong_duong` | khác byte, tương đương canonical |
| `tests/property.rs::byte_index_char_boundary_sau_di_chuyen` | byte index là char boundary sau di |
| `tests/unicode.rs::con_tro_khong_nam_giua_utf8_code_point` | byte không giữa code point |
| `tests/unicode.rs::con_tro_khong_nam_giua_grapheme_cluster` | grapheme không giữa cluster |
| `tests/regression.rs::reg_con_tro_giua_cluster_snap_ve_ranh_gioi` | snap về ranh giới cluster |
| `tests/unicode.rs::emoji_co_zero_width_joiner` | ZWJ một grapheme, cursor không tách |
| `tests/telex_nfd.rs::nfd_byte_index_la_char_boundary` | NFD byte index char boundary |

## 5. Phân đoạn

- Các đoạn không overlap.
- Các đoạn phủ toàn bộ lịch sử có output.
- Tổng phạm vi đoạn không vượt lịch sử.
- Không tạo đoạn rỗng vô nghĩa.
- Replay phân đoạn deterministic.
- Ký tự kỹ thuật không bị nuốt.
- Telex không nối xuyên ranh giới bị cấm.

| Test | Bất biến |
|---|---|
| `tests/phan_doan.rs::khoang_trang_tach_doan` | khoảng trắng tách, tone không xuyên |
| `tests/phan_doan.rs::dau_cau_tach_doan` | `_` tách đoạn |
| `tests/phan_doan.rs::nguyen_ban_tach_rieng` | `them_nguyen_ban` ranh giới |
| `tests/phan_doan.rs::emoji_rieng` | emoji tách riêng |
| `tests/telex_mix.rs::telex_nguyen_ban_telex_hai_doan` | hai đoạn Telex độc lập |
| `tests/telex_mix.rs::nguyen_ban_chan_tone_xuyen` | raw chặn tone xuyên |

## 6. Giới hạn

- Lịch sử không vượt `gioi_han_thao_tac`.
- Input vượt giới hạn không sửa state cũ.
- Không allocation theo kích thước bên ngoài token.
- Không recursion phụ thuộc độ dài input.
- Không vòng lặp vô hạn.
- Không số học tràn trong index/cursor.

| Test | Bất biến |
|---|---|
| `tests/gioi_han.rs::them_qua_gioi_han_khong_doi_state` | vượt giới hạn → `KhongDoi`, state giữ |
| `tests/gioi_han.rs::sau_khi_xoa_co_the_them_lai` | xóa rồi thêm lại được |
| `tests/gioi_han.rs::nhieu_phien_co_gioi_han_doc_lap` | phiên độc lập dưới giới hạn |
| `tests/gioi_han.rs::gioi_han_ap_dung_cho_nguyen_ban` | giới hạn áp dụng cả `them_nguyen_ban` |
| `tests/regression.rs::reg_gioi_han_xoa_roi_them_lai` | giới hạn không khoá vĩnh viễn |
| `tests/property.rs::bat_bien_sau_hanh_dong` | số thao tác raw ≤ giới hạn |

## 7. Isolation

Hai `PhienGo` không chia sẻ mutable state. `BoGo` tạo nhiều phiên độc lập.
Không global mutable cache.

| Test | Bất biến |
|---|---|
| `tests/phien_co_ban.rs::hai_phien_doc_lap` | hai phiên không rò state |
| `tests/gioi_han.rs::nhieu_phien_co_gioi_han_doc_lap` | phiên dưới cùng BoGo độc lập |
| `tests/property.rs::hai_phien_doc_lap` | replay độc lập trên hai phiên |
| `tests/phien_co_ban.rs::token_sau_commit_khong_chua_state_cu` | commit rồi thêm mới không rò |

## 8. Commit / reset

- `chap_nhan` phiên rỗng → `KhongDoi`.
- `chap_nhan` phiên có nội dung → `ChapNhan { noi_dung }` rồi reset.
- Sau commit, phiên rỗng hoàn toàn.

| Test | Bất biến |
|---|---|
| `tests/phien_co_ban.rs::commit_phien_rong_khong_sinh_noi_dung` | rỗng → `KhongDoi` |
| `tests/phien_co_ban.rs::commit_phien_co_noi_dung_tra_dung_chuoi` | có nội dung → `ChapNhan` |
| `tests/phien_co_ban.rs::sau_commit_phien_rong_hoan_toan` | sau commit rỗng |
| `tests/property.rs::commit_roi_reset_sach` | commit/reset không rò state |
| `tests/regression.rs::reg_commit_sau_reset_tra_khong_doi` | reset rồi commit → `KhongDoi` |

## 9. Trace

- Trace zero-overhead khi tắt feature (`cfg` ẩn).
- Trace không thay đổi output.
- Trace không chứa pointer, địa chỉ, timing, machine-specific data.
- Trace chỉ chứa token hiện tại (chuỗi raw/ra của đoạn).

| Test | Bất biến |
|---|---|
| `tests/trace.rs::trace_telex_bien_doi` | trace phản ánh quyết định Telex |
| `tests/trace.rs::trace_cargo_build_raw` | trace phản ánh raw |
| `tests/trace.rs::trace_deterministic` | trace deterministic |
| `tests/trace.rs::trace_rong` | phiên rỗng → trace rỗng |

## 10. Config / serialization

- `CauHinh` chỉ mang giá trị hợp lệ (validation trong `dat_gioi_han_thao_tac`).
- `dat_gioi_han_thao_tac` lỗi giữ giá trị cũ.
- `CauHinh` không derive `Deserialize` (tránh bypass validation).
- `KetQuaXuLy`, `LoaiNoiDung` derive serde (không phải snapshot, không ràng buộc).

| Test | Bất biến |
|---|---|
| `tests/cau_hinh.rs::dat_gioi_han_bang_khong_bi_tu_choi` | 0 bị từ chối |
| `tests/cau_hinh.rs::dat_gioi_han_vuot_toi_da_bi_tu_choi` | 4097 bị từ chối |
| `tests/cau_hinh.rs::cau_hinh_loi_khong_thay_doi_gia_tri_cu` | lỗi giữ giá trị cũ |
| `tests/serde.rs::ket_qua_xu_ly_co_serde` | `KetQuaXuLy` Serialize/Deserialize |
| `tests/serde.rs::loai_noi_dung_co_serde` | `LoaiNoiDung` Serialize/Deserialize |

## 11. Selection / policy

- Cấu trúc kỹ thuật chắc chắn (URL, email, code fence, `::`, `=`) luôn raw
  trong mọi chính sách.
- `them_nguyen_ban` luôn chặn Telex.
- 2+ dấu thanh → raw.
- shape + onset không hợp lệ → raw.

| Test | Bất biến |
|---|---|
| `tests/chinh_sach_lua_chon.rs::tat_ca_chinh_sach_cau_truc_raw` | cấu trúc raw mọi policy |
| `tests/chinh_sach_lua_chon.rs::tat_ca_chinh_sach_2_dau_thanh_raw` | 2+ dấu thanh → raw |
| `tests/chinh_sach_lua_chon.rs::tat_ca_chinh_sach_shape_onset_sai_raw` | shape + onset sai → raw |
| `tests/chinh_sach_lua_chon.rs::uu_tien_tieng_viet_tone_telex` | UuTienTiengViet thông Rule 5 |
| `tests/telex_lua_chon.rs::class_ve_raw` | `class` raw |

## Bất biến sẽ bổ sung Phase 4

Các bất biến dưới đây chưa có test và sẽ được thêm trong Phase 4 (soak,
differential, property nâng cao, rule matrix). Mục này sẽ được điền đầy sau
khi test tương ứng commit.

- Soak: không panic, không invariant failure sau hàng triệu thao tác.
- Navigation đạt đầu/cuối trong hữu hạn bước (property).
- Segment count không vượt action count hợp lý (property).
- Mọi `char` không panic (property mở rộng).
- Serialization round-trip không tạo config bất hợp lệ.
- Cursor UTF-16 không vượt số code unit (property tách riêng).
