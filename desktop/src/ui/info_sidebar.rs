use rune::{col, text, El};
use rune::tok::type_;
use crate::State;
use super::theme::*;
pub fn view(s: &State) -> El { col().key("info-drawer").w(280.0).h_full().padding(16.0).gap(10.0).fill(SURFACE).child(text("Image Information").font(type_::BODY).color(TEXT)).child(text("Folder").color(SECONDARY)).child(text(&s.folder).color(TEXT).w_full().truncate()).child(text("Image Size").color(SECONDARY)).child(text(&s.dimensions).color(TEXT)).child(text("Image Format").color(SECONDARY)).child(text(&s.format).color(TEXT)).child(text("File Size").color(SECONDARY)).child(text(&s.file_size).color(TEXT)) }
