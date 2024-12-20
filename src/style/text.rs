#[derive(Copy, Clone)]
pub enum Text {
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

impl super::PrintableStyle for Text {
    fn print(&self) -> String {
        match self {
            Text::Bold => String::from("\x1b[1m"),
            Text::Dim => String::from("\x1b[2m"),
            Text::Italic => String::from("\x1b[3m"),
            Text::Underline => String::from("\x1b[4m"),
            Text::Blinking => String::from("\x1b[5m"),
            Text::Inverse => String::from("\x1b[7m"),
            Text::Hidden => String::from("\x1b[8m"),
            Text::Strikethrough => String::from("\x1b[9m"),
            Text::NoBold | Text::NoDim => String::from("\x1b[22m"),
            Text::NoItalic => String::from("\x1b[23m"),
            Text::NoUnderline => String::from("\x1b[24m"),
            Text::NoBlinking => String::from("\x1b[25m"),
            Text::NoInverse => String::from("\x1b[27m"),
            Text::NoHidden => String::from("\x1b[28m"),
            Text::NoStrikethrough => String::from("\x1b[29m"),
        }
    }
}
