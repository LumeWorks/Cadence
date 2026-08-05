// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Phiên soạn thảo — stateful, giữ lịch sử thao tác làm nguồn sự thật.

use alloc::string::String;
use alloc::vec::Vec;

use crate::ban_chup::BanChupSoan;
use crate::cau_hinh::{CauHinh, DangUnicode};
use crate::chu_viet::ChuCaiViet;
use crate::ket_qua::KetQuaXuLy;
use crate::render;
use crate::thao_tac::ThaoTacNhap;

/// Phiên soạn thảo của một đoạn composition.
///
/// Lịch sử thao tác là nguồn sự thật; snapshot luôn được dựng lại từ
/// lịch sử sau mỗi thay đổi. Con trỏ nội bộ nằm giữa các thao tác.
pub struct PhienGo {
    /// Bản sao giới hạn thao tác cần thiết cho phiên.
    gioi_han_thao_tac: usize,
    /// Dạng Unicode output.
    dang_unicode: DangUnicode,
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
            dang_unicode: cau_hinh.dang_unicode(),
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

    /// Xóa thao tác ngay trước con trỏ (backspace). Trả `KhongDoi` nếu
    /// con trỏ đang ở đầu lịch sử.
    pub fn xoa_lui(&mut self) -> KetQuaXuLy {
        if self.con_tro == 0 {
            return KetQuaXuLy::KhongDoi;
        }
        self.lich_su.remove(self.con_tro - 1);
        self.con_tro -= 1;
        self.xay_lai_ban_chup();
        KetQuaXuLy::CapNhat
    }

    /// Xóa thao tác ngay sau con trỏ (delete). Trả `KhongDoi` nếu con trỏ
    /// đang ở cuối lịch sử.
    pub fn xoa_phia_truoc(&mut self) -> KetQuaXuLy {
        if self.con_tro >= self.lich_su.len() {
            return KetQuaXuLy::KhongDoi;
        }
        self.lich_su.remove(self.con_tro);
        self.xay_lai_ban_chup();
        KetQuaXuLy::CapNhat
    }

    /// Di chuyển con trỏ về đầu lịch sử. Trả `KhongDoi` nếu đang ở đầu.
    pub fn ve_dau(&mut self) -> KetQuaXuLy {
        if self.con_tro == 0 {
            return KetQuaXuLy::KhongDoi;
        }
        self.con_tro = 0;
        self.xay_lai_ban_chup();
        KetQuaXuLy::CapNhat
    }

    /// Di chuyển con trỏ về cuối lịch sử. Trả `KhongDoi` nếu đang ở cuối.
    pub fn ve_cuoi(&mut self) -> KetQuaXuLy {
        let cuoi = self.lich_su.len();
        if self.con_tro == cuoi {
            return KetQuaXuLy::KhongDoi;
        }
        self.con_tro = cuoi;
        self.xay_lai_ban_chup();
        KetQuaXuLy::CapNhat
    }

    /// Khôi phục nguyên bản: đảm bảo snapshot hiển thị đúng nội dung gốc
    /// người dùng nhập, không bị biến đổi.
    ///
    /// Phase 1 luôn render nguyên bản nên không có gì cần khôi phục; method
    /// idempotent và trả `KhongDoi`. Phase 2 sẽ dùng nó để hủy biến đổi Telex.
    pub fn khoi_phuc_nguyen_ban(&mut self) -> KetQuaXuLy {
        KetQuaXuLy::KhongDoi
    }

    /// Commit đoạn đang soạn: trả nội dung hiện tại, rồi đặt lại phiên.
    ///
    /// Commit phiên rỗng trả `KhongDoi` và không thay đổi state.
    pub fn chap_nhan(&mut self) -> KetQuaXuLy {
        if self.dang_trong() {
            return KetQuaXuLy::KhongDoi;
        }
        let noi_dung = String::from(self.ban_chup_hien_tai.noi_dung());
        self.reset();
        KetQuaXuLy::ChapNhan { noi_dung }
    }

    /// Đặt lại phiên: xóa toàn bộ lịch sử, con trỏ và snapshot.
    pub fn dat_lai(&mut self) {
        self.reset();
    }

    /// Xóa toàn bộ state nội bộ. Dùng chung cho `chap_nhan` và `dat_lai`.
    fn reset(&mut self) {
        self.lich_su.clear();
        self.con_tro = 0;
        self.bo_dem.clear();
        self.ban_chup_hien_tai = BanChupSoan::rong();
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
    /// Phase 2 bước đầu: mỗi ký tự raw được phân tích thành `ChuCaiViet`
    /// rồi render lại qua module render. Kết quả hiển thị vẫn bằng raw
    /// (chưa có biến đổi Telex) nhưng pipeline render đã được kết nối.
    fn xay_lai_ban_chup(&mut self) {
        self.bo_dem.clear();
        for thao_tac in &self.lich_su {
            let chu = match render::phan_tich_ky_tu(thao_tac.ky_tu) {
                Some(c) => c,
                None => ChuCaiViet::thuong(thao_tac.ky_tu),
            };
            self.bo_dem
                .push_str(&render::render_chu(&chu, self.dang_unicode));
        }
        // Nội dung gốc: raw byte-for-byte.
        let noi_dung_goc: String = self.lich_su.iter().map(|t| t.ky_tu).collect();
        // Vị trí byte con trỏ: render tiền tố raw trước con trỏ.
        // TODO(phase-2): thay bằng ánh xạ provenance khi Telex gộp thao tác.
        let mut tien_to = String::new();
        for thao_tac in &self.lich_su[..self.con_tro] {
            let chu = match render::phan_tich_ky_tu(thao_tac.ky_tu) {
                Some(c) => c,
                None => ChuCaiViet::thuong(thao_tac.ky_tu),
            };
            tien_to.push_str(&render::render_chu(&chu, self.dang_unicode));
        }
        let loai = if noi_dung_goc.is_empty() {
            crate::loai_noi_dung::LoaiNoiDung::Trong
        } else {
            crate::loai_noi_dung::LoaiNoiDung::NguyenBan
        };
        self.ban_chup_hien_tai =
            BanChupSoan::dung(self.bo_dem.clone(), noi_dung_goc, tien_to.len(), loai);
    }
}
