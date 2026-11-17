use rune::{el, row, text, El, Tx};
use rune::color::Color;
use rune::tok::type_;
use crate::State;
use super::{icons, theme::*};

fn window_button(label: &'static str, close: bool, action: impl Fn(&mut State, &mut Tx) + 'static) -> El {
    let _ = close;
    el().size(32.0,32.0).radius(6.0).fill(Color::TRANSPARENT).hover(HOVER).center().no_window_drag()
        .child(text(label).font(rune::tok::TextStyle::new(16.0, 600.0)).color(TEXT).center()).on_press_tx(action)
}

pub fn view(s: &State) -> El {
    let title = if s.filename.is_empty() { "Veyra" } else { &s.filename };
    let mut left = row().w(112.0).h_full().center_y().gap(2.0);
    left = left.child(icons::button_enabled("rotate-left", &s.icons.rotate_left, s.has_image, |s| if s.has_image { s.rotation = (s.rotation - 90).rem_euclid(360) }))
        .child(icons::button_enabled("rotate-right", &s.icons.rotate_right, s.has_image, |s| if s.has_image { s.rotation = (s.rotation + 90).rem_euclid(360) }))
        .child(icons::button_tx_enabled("fullscreen", &s.icons.fullscreen, true, |_, tx| tx.toggle_full_screen()));
    let mut right = row().w(184.0).h_full().center_y().justify_end().gap(4.0);
    right = right.child(icons::button_enabled("info", &s.icons.info, s.has_image, |s| s.show_info = !s.show_info))
        .child(icons::button("menu", &s.icons.menu, |s| s.show_menu = !s.show_menu))
        .child(el().w(4.0).h_full());
    right = right.child(window_button("−", false, |_, tx| tx.minimize()))
        .child(window_button("□", false, |_, tx| tx.toggle_maximized()))
        .child(window_button("×", true, |_, tx| tx.close_window()));
    row().key("unified-header").w_full().h(46.0).padding(0.0).fill(SURFACE).border_sides([0.0, 0.0, 1.0, 0.0], BORDER).window_drag()
        .child(el().w(12.0).h_full())
        .child(left)
        .child(el().grow(1.0).h_full().center().child(text(title).font(type_::BODY).color(TEXT).truncate()))
        .child(right)
        .child(el().w(12.0).h_full())
}
