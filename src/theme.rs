use crate::style::color::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub fg: Color,
    pub fg_alt: Color,
    pub fg_focus: Color,
    pub bg: ColorBG,
    pub bg_alt: ColorBG,
    pub bg_focus: ColorBG,
    pub accent: Color,
    pub accent_alt: Color,
    pub border: Color,
    pub border_alt: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
}

impl Theme {
    pub fn new() -> Theme {
        Self {
            fg: Color::ID(255),
            fg_alt: Color::ID(245),
            fg_focus: Color::ID(232),
            bg: ColorBG::ID(232),
            bg_alt: ColorBG::ID(233),
            bg_focus: ColorBG::ID(255),
            accent: Color::ID(255),
            accent_alt: Color::ID(245),
            border: Color::ID(250),
            border_alt: Color::ID(245),
            red: Color::Red,
            green: Color::Green,
            yellow: Color::Yellow,
            blue: Color::Blue,
            magenta: Color::Magenta,
            cyan: Color::Cyan,
            white: Color::White,
        }
    }

    // I'll do this eventually
    pub fn from_file(/* file name */) /* -> Theme */ {}
}
