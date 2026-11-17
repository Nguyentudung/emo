use rune::{col, el, row, El, Tx};
use crate::State;
use super::{icons, theme::*};
pub fn view(s: &State) -> El { col().w_full().h(TOOLBAR_HEIGHT).padding(6.0).fill(SURFACE).child(row().w_full().h_full().center_y().gap(3.0).child(icons::button("rotate-left",&s.icons.rotate_left,|s|s.rotation=(s.rotation-90).rem_euclid(360))).child(icons::button("rotate-right",&s.icons.rotate_right,|s|s.rotation=(s.rotation+90).rem_euclid(360))).child(icons::button_tx("fullscreen",&s.icons.fullscreen,|_,tx: &mut Tx|tx.toggle_full_screen())).child(icons::button("info",&s.icons.info,|s|s.show_info=!s.show_info)).child(el().grow(1.0)).child(icons::button("menu",&s.icons.menu,|s|s.show_menu=!s.show_menu))) }
