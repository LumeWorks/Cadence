// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Phiên soạn thảo - stateful, giữ lịch sử thao tác làm nguồn sự thật.

use alloc::string::String;
use alloc::vec::Vec;

use crate::anh_xa;
use crate::ban_chup::BanChupSoan;
use crate::cau_hinh::{CauHinh, ChinhSachLuaChon, DangUnicode, KieuGo, KieuTelex, QuyTacDatDau};
use crate::ket_qua::KetQuaXuLy;
use crate::thao_tac::ThaoTacNhap;

/// Phiên soạn thảo của một đoạn composition.
///
/// Lịch sử thao tác là nguồn sự thật; snapshot luôn được dựng lại từ
/// lịch sử sau mỗi thay đổi. Con trỏ nội bộ là raw position (số thao tác
/// trước con trỏ), luôn được snap về ranh giới grapheme navigable.
///
/// # Threading
///
/// `PhienGo` là `Send`: có thể chuyển quyền sở hữu sang thread khác (một
/// phiên cho một input context, xử lý trên thread đó). `PhienGo` cũng là
/// `Sync` cho chia sẻ chỉ đọc (`&PhienGo`): các method đọc (`ban_chup`,
/// `trace`, `dang_trong`) an toàn khi chia sẻ; các method `&mut self` cần
/// quyền sở hữu độc quyền. Cam kết này được kiểm chứng trong
/// `tests/contract.rs`.
pub struct PhienGo {
    /// Bản sao giới hạn thao tác cần thiết cho phiên.
    gioi_han_thao_tac: usize,
    /// Dạng Unicode output.
    dang_unicode: DangUnicode,
    /// Kiểu gõ (Telex hoặc VNI).
    kieu_go: KieuGo,
    /// Kiểu Telex (cân bằng hay đầy đủ).
    kieu_telex: KieuTelex,
    /// Quy tắc đặt dấu thanh (hiện đại hay truyền thống).
    quy_tac_dat_dau: QuyTacDatDau,
    /// Chính sách lựa chọn raw/Telex theo ngữ cảnh (Phase 3).
    chinh_sach_lua_chon: ChinhSachLuaChon,
    /// Lịch sử thao tác (nguồn sự thật).
    lich_su: Vec<ThaoTacNhap>,
    /// Con trỏ nội bộ: số thao tác raw trước con trỏ (`0..=lich_su.len()`).
    con_tro: usize,
    /// Ánh xạ raw position → byte offset (tính lại mỗi replay).
    raw_to_byte: Vec<usize>,
    /// Các raw position là ranh giới grapheme navigable.
    navigable: Vec<usize>,
    /// Snapshot hiện tại, dựng lại sau mỗi thay đổi.
    ban_chup_hien_tai: BanChupSoan,
    /// Trace bước quyết định (chỉ khi feature `trace`).
    #[cfg(feature = "trace")]
    trace: alloc::vec::Vec<crate::trace::TraceStep>,
}

impl PhienGo {
    /// Tạo phiên rỗng từ cấu hình.
    pub(crate) fn moi(cau_hinh: CauHinh) -> Self {
        Self {
            gioi_han_thao_tac: cau_hinh.gioi_han_thao_tac(),
            dang_unicode: cau_hinh.dang_unicode(),
            kieu_go: cau_hinh.kieu_go(),
            kieu_telex: cau_hinh.kieu_telex(),
            quy_tac_dat_dau: cau_hinh.quy_tac_dat_dau(),
            chinh_sach_lua_chon: cau_hinh.chinh_sach_lua_chon(),
            lich_su: Vec::new(),
            con_tro: 0,
            raw_to_byte: alloc::vec![0],
            navigable: alloc::vec![0],
            ban_chup_hien_tai: BanChupSoan::rong(),
            #[cfg(feature = "trace")]
            trace: alloc::vec::Vec::new(),
        }
    }

    /// Trả snapshot hiện tại.
    #[must_use]
    pub fn ban_chup(&self) -> &BanChupSoan {
        &self.ban_chup_hien_tai
    }

    /// Trả trace bước quyết định cho phiên hiện tại.
    ///
    /// Chỉ available khi feature `trace` bật. Mỗi phần tử mô tả quyết định
    /// raw/Telex cho một đoạn raw, kèm bằng chứng và chuỗi vào/ra.
    #[cfg(feature = "trace")]
    #[must_use]
    pub fn trace(&self) -> &[crate::trace::TraceStep] {
        &self.trace
    }

    /// Trả `true` nếu phiên đang rỗng.
    #[must_use]
    pub fn dang_trong(&self) -> bool {
        self.ban_chup_hien_tai.dang_trong()
    }

    /// Thêm ký tự theo chế độ tự động (do Telex biến đổi).
    ///
    /// Khi phiên đã đạt giới hạn thao tác, trả `KhongDoi` và giữ nguyên state.
    pub fn them_ky_tu(&mut self, ky_tu: char) -> KetQuaXuLy {
        self.chen_thao_tac(ThaoTacNhap::tu_dong(ky_tu))
    }

    /// Thêm ký tự nguyên bản, giữ đúng ký tự người dùng bấm, không biến đổi
    /// Telex. Ký tự nguyên bản cũng chặn rule Telex nối xuyên qua nó.
    pub fn them_nguyen_ban(&mut self, ky_tu: char) -> KetQuaXuLy {
        self.chen_thao_tac(ThaoTacNhap::nguyen_ban(ky_tu))
    }

    /// Di chuyển con trỏ sang trái một grapheme hiển thị. Trả `KhongDoi`
    /// nếu đang ở đầu.
    pub fn di_trai(&mut self) -> KetQuaXuLy {
        let moi = anh_xa::di_trai_raw(self.con_tro, &self.navigable);
        if moi == self.con_tro {
            return KetQuaXuLy::KhongDoi;
        }
        self.con_tro = moi;
        self.cap_nhat_con_tro();
        KetQuaXuLy::CapNhat
    }

    /// Di chuyển con trỏ sang phải một grapheme hiển thị. Trả `KhongDoi`
    /// nếu đang ở cuối.
    pub fn di_phai(&mut self) -> KetQuaXuLy {
        let moi = anh_xa::di_phai_raw(self.con_tro, &self.navigable);
        if moi == self.con_tro {
            return KetQuaXuLy::KhongDoi;
        }
        self.con_tro = moi;
        self.cap_nhat_con_tro();
        KetQuaXuLy::CapNhat
    }

    /// Xóa thao tác raw ngay trước con trỏ (backspace hoàn tác một thao tác
    /// nhập). Trả `KhongDoi` nếu con trỏ đang ở đầu lịch sử.
    pub fn xoa_lui(&mut self) -> KetQuaXuLy {
        if self.con_tro == 0 {
            return KetQuaXuLy::KhongDoi;
        }
        self.lich_su.remove(self.con_tro - 1);
        self.con_tro -= 1;
        self.xay_lai_ban_chup();
        KetQuaXuLy::CapNhat
    }

    /// Xóa thao tác raw ngay sau con trỏ (delete). Trả `KhongDoi` nếu con
    /// trỏ đang ở cuối lịch sử.
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
        self.cap_nhat_con_tro();
        KetQuaXuLy::CapNhat
    }

    /// Di chuyển con trỏ về cuối lịch sử. Trả `KhongDoi` nếu đang ở cuối.
    pub fn ve_cuoi(&mut self) -> KetQuaXuLy {
        let cuoi = self.lich_su.len();
        if self.con_tro == cuoi {
            return KetQuaXuLy::KhongDoi;
        }
        self.con_tro = cuoi;
        self.cap_nhat_con_tro();
        KetQuaXuLy::CapNhat
    }

    /// Khôi phục nguyên bản: hiển thị đúng raw input, không biến đổi Telex.
    ///
    /// Đây là **no-op idempotent**: pipeline của Cadence luôn dựng lại snapshot
    /// từ lịch sử raw làm nguồn sự thật, nên raw không bao giờ bị mất và luôn
    /// có sẵn qua [`BanChupSoan::noi_dung_goc`](crate::BanChupSoan::noi_dung_goc).
    /// Method này tồn tại cho API completeness và luôn trả [`KetQuaXuLy::KhongDoi`].
    ///
    /// Lý do không toggle raw/Telex trong cùng phiên: một phiên chỉ có một
    /// output; nếu host muốn xem raw, đọc `noi_dung_goc()`. Việc thêm toggle
    /// sẽ tạo trạng thái hiển thị thứ hai, phá bất biến "lịch sử là nguồn sự
    /// thật duy nhất" (RFC 0002). Nếu sau này cần chế độ xem raw riêng, sẽ
    /// thêm snapshot view riêng chứ không toggle state phiên.
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
        self.raw_to_byte = alloc::vec![0];
        self.navigable = alloc::vec![0];
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

    /// Dựng lại toàn bộ snapshot từ lịch sử thao tác qua pipeline Telex.
    fn xay_lai_ban_chup(&mut self) {
        let ket_qua = anh_xa::xay_lai(
            &self.lich_su,
            self.dang_unicode,
            self.kieu_go,
            self.kieu_telex,
            self.quy_tac_dat_dau,
            self.chinh_sach_lua_chon,
        );
        // Giữ con_tro là raw position thực (không snap) để xoa_lui hoàn tác
        // đúng thao tác raw gần nhất. Chỉ snap khi tính byte cho snapshot.
        let con_tro_snap = anh_xa::snap_raw(self.con_tro, &ket_qua.navigable);
        let byte = anh_xa::byte_tai(con_tro_snap, &ket_qua.raw_to_byte);
        let noi_dung_goc: String = self.lich_su.iter().map(|t| t.ky_tu).collect();
        self.raw_to_byte = ket_qua.raw_to_byte;
        self.navigable = ket_qua.navigable;
        #[cfg(feature = "trace")]
        {
            self.trace = ket_qua.trace;
        }
        self.ban_chup_hien_tai =
            BanChupSoan::dung(ket_qua.noi_dung, noi_dung_goc, byte, ket_qua.loai_noi_dung);
    }

    /// Chỉ cập nhật con trỏ (sau khi di chuyển) mà không replay toàn bộ.
    /// Tính lại byte offset từ raw_to_byte hiện có (có snap).
    fn cap_nhat_con_tro(&mut self) {
        let con_tro_snap = anh_xa::snap_raw(self.con_tro, &self.navigable);
        let byte = anh_xa::byte_tai(con_tro_snap, &self.raw_to_byte);
        let noi_dung = String::from(self.ban_chup_hien_tai.noi_dung());
        let noi_dung_goc = String::from(self.ban_chup_hien_tai.noi_dung_goc());
        let loai = self.ban_chup_hien_tai.loai_noi_dung();
        self.ban_chup_hien_tai = BanChupSoan::dung(noi_dung, noi_dung_goc, byte, loai);
    }
}
