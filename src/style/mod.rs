pub mod align;
pub mod color;
pub mod line;
pub mod orientation;
pub mod text;

use crate::printf;

pub trait PrintableStyle {
    fn print(&self) -> String;
}

pub fn set_style(s: impl PrintableStyle) {
    printf!("{}", s.print());
}

pub fn reset() {
    printf!("\x1b[0m");
}
