// 256-Bit color ids -> https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797#256-colors
// RGB requires a true color terminal

// Foreground
#[derive(Copy, Clone)]
pub enum Color {
    None,
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    ID(usize),
    RGB(usize, usize, usize),
}

impl Color {
    pub fn to_bg(&self) -> ColorBG {
        match self {
            Color::None => ColorBG::None,
            Color::Reset => ColorBG::Reset,
            Color::Black => ColorBG::Black,
            Color::Red => ColorBG::Red,
            Color::Green => ColorBG::Green,
            Color::Yellow => ColorBG::Yellow,
            Color::Blue => ColorBG::Blue,
            Color::Magenta => ColorBG::Magenta,
            Color::Cyan => ColorBG::Cyan,
            Color::White => ColorBG::White,
            Color::ID(id) => ColorBG::ID(*id),
            Color::RGB(r, g, b) => ColorBG::RGB(*r, *g, *b),
        }
    }
}

impl super::PrintableStyle for Color {
    fn print(&self) -> String {
        match self {
            Color::None => String::new(),
            Color::Reset => String::from("\x1b[39m"),
            Color::Black => String::from("\x1b[30m"),
            Color::Red => String::from("\x1b[31m"),
            Color::Green => String::from("\x1b[32m"),
            Color::Yellow => String::from("\x1b[33m"),
            Color::Blue => String::from("\x1b[34m"),
            Color::Magenta => String::from("\x1b[35m"),
            Color::Cyan => String::from("\x1b[36m"),
            Color::White => String::from("\x1b[37m"),
            Color::ID(id) => format!("\x1b[38;5;{}m", id),
            Color::RGB(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
        }
    }
}

// Background
#[derive(Copy, Clone)]
pub enum ColorBG {
    None,
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    ID(usize),
    RGB(usize, usize, usize),
}

impl super::PrintableStyle for ColorBG {
    fn print(&self) -> String {
        match self {
            ColorBG::None => String::new(),
            ColorBG::Reset => String::from("\x1b[49m"),
            ColorBG::Black => String::from("\x1b[40m"),
            ColorBG::Red => String::from("\x1b[41m"),
            ColorBG::Green => String::from("\x1b[42m"),
            ColorBG::Yellow => String::from("\x1b[43m"),
            ColorBG::Blue => String::from("\x1b[44m"),
            ColorBG::Magenta => String::from("\x1b[45m"),
            ColorBG::Cyan => String::from("\x1b[46m"),
            ColorBG::White => String::from("\x1b[47m"),
            ColorBG::ID(id) => format!("\x1b[48;5;{}m", id),
            ColorBG::RGB(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
        }
    }
}

impl ColorBG {
    pub fn to_fg(&self) -> Color {
        match self {
            ColorBG::None => Color::None,
            ColorBG::Reset => Color::Reset,
            ColorBG::Black => Color::Black,
            ColorBG::Red => Color::Red,
            ColorBG::Green => Color::Green,
            ColorBG::Yellow => Color::Yellow,
            ColorBG::Blue => Color::Blue,
            ColorBG::Magenta => Color::Magenta,
            ColorBG::Cyan => Color::Cyan,
            ColorBG::White => Color::White,
            ColorBG::ID(id) => Color::ID(*id),
            ColorBG::RGB(r, g, b) => Color::RGB(*r, *g, *b),
        }
    }
}
