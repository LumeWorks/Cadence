# Migration 0.1.0 → 2026.1.0

Tag nội bộ `v0.1.0` chưa publish crates.io nhưng có thể có người dùng Git
dependency. Tài liệu này hướng dẫn migration sang `2026.1.0`.

## Không breaking change

`CauHinh::mac_dinh()` vẫn chọn Telex. Code hiện tại không cần đổi.

```rust
// 0.1.0
let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("hop le");
```

```rust
// 2026.1.0 — code không đổi
let bo_go = BoGo::new(CauHinh::mac_dinh()).expect("hop le");
```

## Thêm VNI

```rust
use cadence::{BoGo, CauHinh, KieuGo};

let mut c = CauHinh::mac_dinh();
c.dat_kieu_go(KieuGo::Vni);
let bo_go = BoGo::new(c).expect("hop le");
let mut phien = bo_go.tao_phien();

// VNI: "a61" → ấ
for ch in "a61".chars() {
    phien.them_ky_tu(ch);
}
assert_eq!(phien.ban_chup().noi_dung(), "ấ");
```

## Cargo.toml

```toml
# 0.1.0 (Git dependency)
cadence = { package = "cadence-ime", git = "..." }

# 2026.1.0 (crates.io)
cadence = { package = "cadence-ime", version = "2026.1.0" }
```

## API mới

| API | Mô tả |
|-----|-------|
| `KieuGo` enum | `Telex`, `Vni` |
| `CauHinh::kieu_go()` | Trả kiểu gõ hiện tại |
| `CauHinh::dat_kieu_go()` | Đặt kiểu gõ |

## API giữ nguyên

| API | Trạng thái |
|-----|------------|
| `CauHinh::mac_dinh()` | Vẫn Telex |
| `CauHinh::dat_kieu_telex()` | Vẫn dùng (chỉ khi `kieu_go == Telex`) |
| `CauHinh::dat_quy_tac_dat_dau()` | Dùng chung Telex + VNI |
| `CauHinh::dat_dang_unicode()` | Dùng chung |
| `CauHinh::dat_chinh_sach_lua_chon()` | Dùng chung |
| `PhienGo`, `BoGo`, `BanChupSoan` | Không đổi |

## Trace

`TraceKetQua` thêm biến thể `Vni` (bên cạnh `Telex` và `NguyenBan`).

## Serde

`KieuGo` derive serde. Mặc định deserialize là `Telex`.
