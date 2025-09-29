use crate::{
    printf,
    style::color::{Color, ColorBG},
};

pub fn show() {
    printf!("\x1b[?25h");
}

pub fn hide() {
    printf!("\x1b[?25l");
}

pub fn home() {
    printf!("\x1b[H");
}

pub fn go(p: crate::unit::Point) {
    printf!("\x1b[{};{}H", p.y.calc(), p.x.calc());
}

pub fn up() {
    printf!("\x1b[1A");
}

pub fn down() {
    printf!("\x1b[1B");
}

pub fn right() {
    printf!("\x1b[1C");
}

pub fn left() {
    printf!("\x1b[1D");
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    None,
    BlinkBlock,
    SolidBlock,
    BlinkingUnderline,
    SolidUnderline,
    BlinkingBar,
    SolidBar,
}

pub fn set_shape(s: CursorShape) {
    printf!("\x1b[{} q", s as usize);
}
