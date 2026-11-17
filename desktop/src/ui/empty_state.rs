use rune::{col, el, text, El};
use rune::tok::type_;
use crate::{open_file, open_folder, State};
use super::{theme::*, action_button, window_title};
pub fn view(s: &State) -> El { col().w_full().grow(1.0).center().gap(12.0).child(el().size(64.0,64.0).pixels(&s.icons.empty)).child(text("View and Edit Images").font(type_::TITLE).color(TEXT)).child(text("Choose a folder or file to start viewing").font(type_::BODY).color(SECONDARY)).child(action_button("Open Folder...",open_folder)).child(text("or Open File...").color(SECONDARY).on_press_tx(|s,tx|{open_file(s);tx.set_title(window_title(s));})) }
