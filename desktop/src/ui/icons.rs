use rune::{el, ImageCache, El, Pixels, Tx};
use super::theme::{CONTROL, HOVER};
use crate::State;

pub struct Icons { pub rotate_left: Pixels, pub rotate_right: Pixels, pub fullscreen: Pixels, pub info: Pixels, pub menu: Pixels, pub previous: Pixels, pub next: Pixels, pub zoom: Pixels, pub down: Pixels, pub empty: Pixels }
impl Default for Icons { fn default() -> Self { Self { rotate_left: Pixels::new(), rotate_right: Pixels::new(), fullscreen: Pixels::new(), info: Pixels::new(), menu: Pixels::new(), previous: Pixels::new(), next: Pixels::new(), zoom: Pixels::new(), down: Pixels::new(), empty: Pixels::new() } } }

pub fn load(cache: &ImageCache) -> Icons {
    let p = |bytes: &'static [u8]| cache.load(bytes).pixels().clone();
    Icons { rotate_left: p(&include_bytes!("../../ui/icons-adwaita/object-rotate-left-symbolic.svg")[..]), rotate_right: p(&include_bytes!("../../ui/icons-adwaita/object-rotate-right-symbolic.svg")[..]), fullscreen: p(&include_bytes!("../../ui/icons-adwaita/view-fullscreen-symbolic.svg")[..]), info: p(&include_bytes!("../../ui/icons-adwaita/info-symbolic.svg")[..]), menu: p(&include_bytes!("../../ui/icons-adwaita/open-menu-symbolic.svg")[..]), previous: p(&include_bytes!("../../ui/icons-hugeicons/arrow-left-01.svg")[..]), next: p(&include_bytes!("../../ui/icons-hugeicons/arrow-right-01.svg")[..]), zoom: p(&include_bytes!("../../ui/icons-hugeicons/zoom-in-area.svg")[..]), down: p(&include_bytes!("../../ui/icons-hugeicons/arrow-down-01.svg")[..]), empty: p(&include_bytes!("../../ui/icons-hugeicons/image-02.svg")[..]) }
}

pub fn button(key: &'static str, icon: &Pixels, f: impl Fn(&mut State) + 'static) -> El { button_enabled(key, icon, true, f) }
pub fn button_enabled(key: &'static str, icon: &Pixels, enabled: bool, f: impl Fn(&mut State) + 'static) -> El { let mut b=el().key(key).size(CONTROL, CONTROL).padding(0.0).radius(8.0).fill(rune::color::Color::TRANSPARENT).center().child(el().size(20.0,20.0).pixels(icon).opacity(if enabled { 1.0 } else { 0.38 })); if enabled { b=b.hover(HOVER).on_press(f); } b }
pub fn button_tx_enabled(key: &'static str, icon: &Pixels, enabled: bool, f: impl Fn(&mut State, &mut Tx) + 'static) -> El { let mut b=el().key(key).size(CONTROL, CONTROL).padding(0.0).radius(8.0).fill(rune::color::Color::TRANSPARENT).center().child(el().size(20.0,20.0).pixels(icon).opacity(if enabled { 1.0 } else { 0.38 })); if enabled { b=b.hover(HOVER).on_press_tx(f); } b }
