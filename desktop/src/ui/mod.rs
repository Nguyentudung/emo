pub mod theme; pub mod icons; pub mod titlebar; pub mod empty_state; pub mod viewport; pub mod floating_pills; pub mod menu_popover; pub mod info_sidebar;
use rune::{col, el, text, Cx, El, Tx, Cursor, ResizeDirection};
use rune::color::Color;
use crate::State;
use theme::*;
pub fn window_title(s: &State) -> String { if s.filename.is_empty() { "Veyra".to_owned() } else { format!("{} — Veyra", s.filename) } }
pub fn action_button(label:&'static str,f:impl Fn(&mut State)+'static)->El{el().size(160.0,38.0).radius(999.0).fill(ACCENT).hover(HOVER).child(text(label).color(rune::color::Color::WHITE).center()).on_press_tx(move|s,tx|{f(s);tx.set_title(window_title(s));})}
/// Rune applies this transaction to the active mapped winit Window via
/// `Window::set_title`; polling also covers navigation and non-transactional
/// state changes such as the Previous/Next controls.
pub fn sync_title(s:&mut State,_:f64,tx:&mut Tx)->bool{tx.set_title(window_title(s));false}
fn resize_handle(key: &'static str, direction: ResizeDirection, width: f32, height: f32, left: Option<f32>, top: Option<f32>, right: Option<f32>, bottom: Option<f32>, cursor: Cursor) -> El { el().key(key).fill(Color::TRANSPARENT).size(width,height).pin(left,top,right,bottom).cursor(cursor).no_window_drag().on_press_tx(move |_: &mut State,tx| tx.drag_resize_window(direction)) }
fn resize_handles(root: El) -> El { root
    // Straight edges leave the rounded 16px corner zones to the corner hit targets below.
    .child(resize_handle("resize-right",ResizeDirection::East,10.0,0.0,None,Some(20.0),Some(0.0),Some(20.0),Cursor::ResizeColumn))
    .child(resize_handle("resize-left",ResizeDirection::West,10.0,0.0,Some(0.0),Some(20.0),None,Some(20.0),Cursor::ResizeColumn))
    .child(resize_handle("resize-bottom",ResizeDirection::South,0.0,10.0,Some(20.0),None,Some(20.0),Some(0.0),Cursor::ResizeRow))
    .child(resize_handle("resize-top",ResizeDirection::North,0.0,10.0,Some(20.0),Some(0.0),Some(20.0),None,Cursor::ResizeRow))
    // Corners are last in the visual tree, so they win overlapping hit-tests.
    .child(resize_handle("resize-se",ResizeDirection::SouthEast,18.0,18.0,None,None,Some(0.0),Some(0.0),Cursor::ResizeColumn))
    .child(resize_handle("resize-sw",ResizeDirection::SouthWest,18.0,18.0,Some(0.0),None,None,Some(0.0),Cursor::ResizeColumn))
    .child(resize_handle("resize-ne",ResizeDirection::NorthEast,18.0,18.0,None,Some(0.0),Some(0.0),None,Cursor::ResizeColumn))
    .child(resize_handle("resize-nw",ResizeDirection::NorthWest,18.0,18.0,Some(0.0),Some(0.0),None,None,Cursor::ResizeColumn)) }
pub fn view(s:&State,_:&mut Cx<State>)->El{let mut root=col().key("app-root").w_full().h_full().fill(CANVAS).border(1.0, Color{r:1.0,g:1.0,b:1.0,a:0.08}).child(titlebar::view(s)); if !s.has_image{root=root.child(empty_state::view(s));if s.show_menu{root=root.child(menu_popover::view(s));}return resize_handles(root);} let mut content=rune::row().w_full().grow(1.0).min_w(0.0).min_h(0.0).child(viewport::view(s));if s.show_info{content=content.child(info_sidebar::view(s));} root=root.child(content);if s.show_menu{root=root.child(menu_popover::view(s));}resize_handles(root)}
