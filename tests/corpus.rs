// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Lê Hùng Quang Minh

//! Corpus Phase 4 - kiểm chứng hệ thống theo nhóm nội dung.
//!
//! Mỗi module liên kết với rule/parser branch cụ thể (xem `docs/INVARIANTS.md`
//! và RFC 0019). Corpus này mở rộng `corpus_phase3.rs` (46 DoD) với enumeration
//! có hệ thống: mọi âm đầu, âm cuối, nguyên âm, dấu thanh, quy tắc đặt dấu,
//! NFC/NFD input, code nhiều ngôn ngữ, kỹ thuật, chat, adversarial.
//!
//! Dùng `#[path]` để Rust resolve submodule trong `tests/corpus/` (integration
//! test crate root là `tests/corpus.rs`).

#[path = "corpus/am_tiet.rs"]
mod am_tiet;
#[path = "corpus/adversarial.rs"]
mod adversarial;
#[path = "corpus/code.rs"]
mod code;
#[path = "corpus/command.rs"]
mod command;
#[path = "corpus/context_mix.rs"]
mod context_mix;
#[path = "corpus/dau_thanh.rs"]
mod dau_thanh;
#[path = "corpus/editing.rs"]
mod editing;
#[path = "corpus/emoticon.rs"]
mod emoticon;
#[path = "corpus/escape.rs"]
mod escape;
#[path = "corpus/hinh_chu.rs"]
mod hinh_chu;
#[path = "corpus/teencode.rs"]
mod teencode;
#[path = "corpus/tieng_viet.rs"]
mod tieng_viet;
#[path = "corpus/unicode.rs"]
mod unicode;
#[path = "corpus/url_email_path.rs"]
mod url_email_path;
