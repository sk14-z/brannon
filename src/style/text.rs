#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TextStyle {
    Bold,
    Dim,
    Italic,
    Underline,
    Blinking,
    Inverse,
    Hidden,
    Strikethrough,
    NoBold,
    NoDim,
    NoItalic,
    NoUnderline,
    NoBlinking,
    NoInverse,
    NoHidden,
    NoStrikethrough,
}

impl super::PrintableStyle for TextStyle {
    fn print(&self) -> String {
        match self {
            TextStyle::Bold => String::from("\x1b[1m"),
            TextStyle::Dim => String::from("\x1b[2m"),
            TextStyle::Italic => String::from("\x1b[3m"),
            TextStyle::Underline => String::from("\x1b[4m"),
            TextStyle::Blinking => String::from("\x1b[5m"),
            TextStyle::Inverse => String::from("\x1b[7m"),
            TextStyle::Hidden => String::from("\x1b[8m"),
            TextStyle::Strikethrough => String::from("\x1b[9m"),
            TextStyle::NoBold | TextStyle::NoDim => String::from("\x1b[22m"),
            TextStyle::NoItalic => String::from("\x1b[23m"),
            TextStyle::NoUnderline => String::from("\x1b[24m"),
            TextStyle::NoBlinking => String::from("\x1b[25m"),
            TextStyle::NoInverse => String::from("\x1b[27m"),
            TextStyle::NoHidden => String::from("\x1b[28m"),
            TextStyle::NoStrikethrough => String::from("\x1b[29m"),
        }
    }
}
