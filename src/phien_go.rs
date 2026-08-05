// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Phiên soạn thảo — stateful, giữ lịch sử thao tác làm nguồn sự thật.

use alloc::string::String;
use alloc::vec::Vec;

use crate::ban_chup::BanChupSoan;
use crate::cau_hinh::CauHinh;
use crate::ket_qua::KetQuaXuLy;
use crate::thao_tac::ThaoTacNhap;

/// Phiên soạn thảo của một đoạn composition.
///
/// Lịch sử thao tác là nguồn sự thật; snapshot luôn được dựng lại từ
/// lịch sử sau mỗi thay đổi. Con trỏ nội bộ nằm giữa các thao tác.
pub struct PhienGo {
    /// Bản sao giới hạn thao tác cần thiết cho phiên.
    gioi_han_thao_tac: usize,
    /// Lịch sử thao tác (nguồn sự thật).
    lich_su: Vec<ThaoTacNhap>,
    /// Con trỏ nội bộ: số thao tác nằm trước con trỏ (`0..=lich_su.len()`).
    con_tro: usize,
    /// Snapshot hiện tại, dựng lại sau mỗi thay đổi.
    ban_chup_hien_tai: BanChupSoan,
    /// Buffer render tái sử dụng để giảm allocation.
    bo_dem: String,
}

impl PhienGo {
    /// Tạo phiên rỗng từ cấu hình.
    pub(crate) fn moi(cau_hinh: CauHinh) -> Self {
        Self {
            gioi_han_thao_tac: cau_hinh.gioi_han_thao_tac(),
            lich_su: Vec::new(),
            con_tro: 0,
            ban_chup_hien_tai: BanChupSoan::rong(),
            bo_dem: String::new(),
        }
    }

    /// Trả snapshot hiện tại.
    #[must_use]
    pub fn ban_chup(&self) -> &BanChupSoan {
        &self.ban_chup_hien_tai
    }

    /// Trả `true` nếu phiên đang rỗng.
    #[must_use]
    pub fn dang_trong(&self) -> bool {
        self.ban_chup_hien_tai.dang_trong()
    }

    /// Thêm ký tự theo chế độ tự động (sẽ do Telex biến đổi trong Phase 2).
    ///
    /// Phase 1: render nguyên bản, kết quả hiển thị giống `them_nguyen_ban`.
    /// Khi phiên đã đạt giới hạn thao tác, trả `KhongDoi` và giữ nguyên state.
    pub fn them_ky_tu(&mut self, ky_tu: char) -> KetQuaXuLy {
        self.chen_thao_tac(ThaoTacNhap::tu_dong(ky_tu))
    }

    /// Thêm ký tự nguyên bản, giữ đúng ký tự người dùng bấm, không biến đổi.
    ///
    /// Phase 1: kết quả hiển thị giống `them_ky_tu`, nhưng lịch sử ghi cờ
    /// `NguyenBan` để Phase 2 biết không áp dụng Telex cho ký tự này.
    pub fn them_nguyen_ban(&mut self, ky_tu: char) -> KetQuaXuLy {
        self.chen_thao_tac(ThaoTacNhap::nguyen_ban(ky_tu))
    }

    /// Di chuyển con trỏ sang trái một thao tác. Trả `KhongDoi` nếu đang ở đầu.
    pub fn di_trai(&mut self) -> KetQuaXuLy {
        if self.con_tro == 0 {
            return KetQuaXuLy::KhongDoi;
        }
        self.con_tro -= 1;
        self.xay_lai_ban_chup();
        KetQuaXuLy::CapNhat
    }

    /// Di chuyển con trỏ sang phải một thao tác. Trả `KhongDoi` nếu đang ở cuối.
    pub fn di_phai(&mut self) -> KetQuaXuLy {
        if self.con_tro >= self.lich_su.len() {
            return KetQuaXuLy::KhongDoi;
        }
        self.con_tro += 1;
        self.xay_lai_ban_chup();
        KetQuaXuLy::CapNhat
    }

    /// Chèn một thao tác tại con trỏ, cập nhật con trỏ và dựng lại snapshot.
    fn chen_thao_tac(&mut self, thao_tac: ThaoTacNhap) -> KetQuaXuLy {
        if self.lich_su.len() >= self.gioi_han_thao_tac {
            return KetQuaXuLy::KhongDoi;
        }
        self.lich_su.insert(self.con_tro, thao_tac);
        self.con_tro += 1;
        self.xay_lai_ban_chup();
        KetQuaXuLy::CapNhat
    }

    /// Render nguyên bản toàn lịch sử vào buffer và dựng lại snapshot.
    ///
    /// Phase 1 không có Telex nên mọi thao tác (tự động hay nguyên bản)
    /// đều được render nguyên ký tự.
    fn xay_lai_ban_chup(&mut self) {
        self.bo_dem.clear();
        for thao_tac in &self.lich_su {
            self.bo_dem.push(thao_tac.ky_tu);
        }
        let chi_so_byte = self.tinh_vi_tri_con_tro_byte();
        self.ban_chup_hien_tai = BanChupSoan::dung(self.bo_dem.clone(), chi_so_byte);
    }

    /// Tính vị trí byte của con trỏ: tổng `len_utf8` của các ký tự nằm
    /// trước con trỏ trong lịch sử.
    fn tinh_vi_tri_con_tro_byte(&self) -> usize {
        self.lich_su[..self.con_tro]
            .iter()
            .map(|thao_tac| thao_tac.ky_tu.len_utf8())
            .sum()
    }
}
