use crate::printf;
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Protocol {
    Default,
    Kitty,
}

impl Protocol {
    pub fn activate(&self) {
        match self {
            Protocol::Default => printf!("\x1b[=0;1u"),
            Protocol::Kitty => printf!("\x1b[={};1u", 0b1111),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum KeyState {
    Press = 1,
    Repeat = 2,
    Release = 3,
}

impl<T: Into<usize>> From<T> for KeyState {
    fn from(n: T) -> Self {
        match n.into() {
            1 => KeyState::Press,
            2 => KeyState::Repeat,
            3 => KeyState::Release,
            _ => KeyState::Press,
        }
    }
}

impl Display for KeyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyState::Press => write!(f, ""),
            KeyState::Repeat => write!(f, " (repeat)"),
            KeyState::Release => write!(f, " (release)"),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Key {
    // F(n) is sent as \x1b[n~ where n = [11..=15, 17..=21, 23..=24]
    // F1-F4 can also be sent as \x1bOP, \x1bOQ, \x1bOR, \x1bOS
    Function(u8),
    // Sent as escape sequences
    Up,       // \x1b[A or \x1bOA
    Down,     // \x1b[B or \x1bOB
    Right,    // \x1b[C or \x1bOC
    Left,     // \x1b[D or \x1bOD
    Home,     // \x1b[1~ or \x1b[H or \x1bOH
    Insert,   // \x1b[2~
    Delete,   // \x1b[3~
    End,      // \x1b[4~ or \x1b[F or \x1bOF
    PageUp,   // \x1b[5~
    PageDown, // \x1b[6~

    // Control characters
    Escape,    // '\x1b'
    Backspace, // '\x08' or '\x7f'
    Tab,       // '\t'
    Enter,     // '\n' or '\r'

    // Visible characters
    Space,      // ' '
    Excl,       // '!'
    DblQuote,   // '"'
    Hash,       // '#'
    Dollar,     // '$'
    Percent,    // '%'
    Amp,        // '&'
    SglQuote,   // '\''
    LParen,     // '('
    RParen,     // ')'
    Star,       // '*'
    Plus,       // '+'
    Comma,      // ','
    Minus,      // '-'
    Period,     // '.'
    Slash,      // '/'
    D0,         // '0'
    D1,         // '1'
    D2,         // '2'
    D3,         // '3'
    D4,         // '4'
    D5,         // '5'
    D6,         // '6'
    D7,         // '7'
    D8,         // '8'
    D9,         // '9'
    Colon,      // ':'
    Semicolon,  // ';'
    LThan,      // '<'
    Equals,     // '='
    GThan,      // '>'
    QMark,      // '?'
    At,         // '@'
    A,          // 'A'
    B,          // 'B'
    C,          // 'C'
    D,          // 'D'
    E,          // 'E'
    F,          // 'F'
    G,          // 'G'
    H,          // 'H'
    I,          // 'I'
    J,          // 'J'
    K,          // 'K'
    L,          // 'L'
    M,          // 'M'
    N,          // 'N'
    O,          // 'O'
    P,          // 'P'
    Q,          // 'Q'
    R,          // 'R'
    S,          // 'S'
    T,          // 'T'
    U,          // 'U'
    V,          // 'V'
    W,          // 'W'
    X,          // 'X'
    Y,          // 'Y'
    Z,          // 'Z'
    LBracket,   // '['
    BSlash,     // '\\'
    RBracket,   // ']'
    Caret,      // '^'
    Underscore, // '_'
    Backtick,   // '`'
    a,          // 'a'
    b,          // 'b'
    c,          // 'c'
    d,          // 'd'
    e,          // 'e'
    f,          // 'f'
    g,          // 'g'
    h,          // 'h'
    i,          // 'i'
    j,          // 'j'
    k,          // 'k'
    l,          // 'l'
    m,          // 'm'
    n,          // 'n'
    o,          // 'o'
    p,          // 'p'
    q,          // 'q'
    r,          // 'r'
    s,          // 's'
    t,          // 't'
    u,          // 'u'
    v,          // 'v'
    w,          // 'w'
    x,          // 'x'
    y,          // 'y'
    z,          // 'z'
    LBrace,     // '{'
    Bar,        // '|'
    RBrace,     // '}'
    Tilde,      // '~'
}

impl Key {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '\x1B' => Some(Key::Escape),
            '\x7f' | '\x08' => Some(Key::Backspace),
            '\t' => Some(Key::Tab),
            '\n' | '\r' => Some(Key::Enter),
            ' ' => Some(Key::Space),
            '!' => Some(Key::Excl),
            '"' => Some(Key::DblQuote),
            '#' => Some(Key::Hash),
            '$' => Some(Key::Dollar),
            '%' => Some(Key::Percent),
            '&' => Some(Key::Amp),
            '\'' => Some(Key::SglQuote),
            '(' => Some(Key::LParen),
            ')' => Some(Key::RParen),
            '*' => Some(Key::Star),
            '+' => Some(Key::Plus),
            ',' => Some(Key::Comma),
            '-' => Some(Key::Minus),
            '.' => Some(Key::Period),
            '/' => Some(Key::Slash),
            '0' => Some(Key::D0),
            '1' => Some(Key::D1),
            '2' => Some(Key::D2),
            '3' => Some(Key::D3),
            '4' => Some(Key::D4),
            '5' => Some(Key::D5),
            '6' => Some(Key::D6),
            '7' => Some(Key::D7),
            '8' => Some(Key::D8),
            '9' => Some(Key::D9),
            ':' => Some(Key::Colon),
            ';' => Some(Key::Semicolon),
            '<' => Some(Key::LThan),
            '=' => Some(Key::Equals),
            '>' => Some(Key::GThan),
            '?' => Some(Key::QMark),
            '@' => Some(Key::At),
            'A' => Some(Key::A),
            'B' => Some(Key::B),
            'C' => Some(Key::C),
            'D' => Some(Key::D),
            'E' => Some(Key::E),
            'F' => Some(Key::F),
            'G' => Some(Key::G),
            'H' => Some(Key::H),
            'I' => Some(Key::I),
            'J' => Some(Key::J),
            'K' => Some(Key::K),
            'L' => Some(Key::L),
            'M' => Some(Key::M),
            'N' => Some(Key::N),
            'O' => Some(Key::O),
            'P' => Some(Key::P),
            'Q' => Some(Key::Q),
            'R' => Some(Key::R),
            'S' => Some(Key::S),
            'T' => Some(Key::T),
            'U' => Some(Key::U),
            'V' => Some(Key::V),
            'W' => Some(Key::W),
            'X' => Some(Key::X),
            'Y' => Some(Key::Y),
            'Z' => Some(Key::Z),
            '[' => Some(Key::LBracket),
            '\\' => Some(Key::BSlash),
            ']' => Some(Key::RBracket),
            '^' => Some(Key::Caret),
            '_' => Some(Key::Underscore),
            '`' => Some(Key::Backtick),
            'a' => Some(Key::a),
            'b' => Some(Key::b),
            'c' => Some(Key::c),
            'd' => Some(Key::d),
            'e' => Some(Key::e),
            'f' => Some(Key::f),
            'g' => Some(Key::g),
            'h' => Some(Key::h),
            'i' => Some(Key::i),
            'j' => Some(Key::j),
            'k' => Some(Key::k),
            'l' => Some(Key::l),
            'm' => Some(Key::m),
            'n' => Some(Key::n),
            'o' => Some(Key::o),
            'p' => Some(Key::p),
            'q' => Some(Key::q),
            'r' => Some(Key::r),
            's' => Some(Key::s),
            't' => Some(Key::t),
            'u' => Some(Key::u),
            'v' => Some(Key::v),
            'w' => Some(Key::w),
            'x' => Some(Key::x),
            'y' => Some(Key::y),
            'z' => Some(Key::z),
            '{' => Some(Key::LBrace),
            '|' => Some(Key::Bar),
            '}' => Some(Key::RBrace),
            '~' => Some(Key::Tilde),
            _ => None,
        }
    }

    pub fn to_char(&self) -> Option<char> {
        match self {
            Key::Escape => Some('\x1B'),
            Key::Backspace => Some('\x08'),
            Key::Tab => Some('\t'),
            Key::Enter => Some('\n'),
            Key::Space => Some(' '),
            Key::Excl => Some('!'),
            Key::DblQuote => Some('"'),
            Key::Hash => Some('#'),
            Key::Dollar => Some('$'),
            Key::Percent => Some('%'),
            Key::Amp => Some('&'),
            Key::SglQuote => Some('\''),
            Key::LParen => Some('('),
            Key::RParen => Some(')'),
            Key::Star => Some('*'),
            Key::Plus => Some('+'),
            Key::Comma => Some(','),
            Key::Minus => Some('-'),
            Key::Period => Some('.'),
            Key::Slash => Some('/'),
            Key::D0 => Some('0'),
            Key::D1 => Some('1'),
            Key::D2 => Some('2'),
            Key::D3 => Some('3'),
            Key::D4 => Some('4'),
            Key::D5 => Some('5'),
            Key::D6 => Some('6'),
            Key::D7 => Some('7'),
            Key::D8 => Some('8'),
            Key::D9 => Some('9'),
            Key::Colon => Some(':'),
            Key::Semicolon => Some(';'),
            Key::LThan => Some('<'),
            Key::Equals => Some('='),
            Key::GThan => Some('>'),
            Key::QMark => Some('?'),
            Key::At => Some('@'),
            Key::A => Some('A'),
            Key::B => Some('B'),
            Key::C => Some('C'),
            Key::D => Some('D'),
            Key::E => Some('E'),
            Key::F => Some('F'),
            Key::G => Some('G'),
            Key::H => Some('H'),
            Key::I => Some('I'),
            Key::J => Some('J'),
            Key::K => Some('K'),
            Key::L => Some('L'),
            Key::M => Some('M'),
            Key::N => Some('N'),
            Key::O => Some('O'),
            Key::P => Some('P'),
            Key::Q => Some('Q'),
            Key::R => Some('R'),
            Key::S => Some('S'),
            Key::T => Some('T'),
            Key::U => Some('U'),
            Key::V => Some('V'),
            Key::W => Some('W'),
            Key::X => Some('X'),
            Key::Y => Some('Y'),
            Key::Z => Some('Z'),
            Key::LBracket => Some('['),
            Key::BSlash => Some('\\'),
            Key::RBracket => Some(']'),
            Key::Caret => Some('^'),
            Key::Underscore => Some('_'),
            Key::Backtick => Some('`'),
            Key::a => Some('a'),
            Key::b => Some('b'),
            Key::c => Some('c'),
            Key::d => Some('d'),
            Key::e => Some('e'),
            Key::f => Some('f'),
            Key::g => Some('g'),
            Key::h => Some('h'),
            Key::i => Some('i'),
            Key::j => Some('j'),
            Key::k => Some('k'),
            Key::l => Some('l'),
            Key::m => Some('m'),
            Key::n => Some('n'),
            Key::o => Some('o'),
            Key::p => Some('p'),
            Key::q => Some('q'),
            Key::r => Some('r'),
            Key::s => Some('s'),
            Key::t => Some('t'),
            Key::u => Some('u'),
            Key::v => Some('v'),
            Key::w => Some('w'),
            Key::x => Some('x'),
            Key::y => Some('y'),
            Key::z => Some('z'),
            Key::LBrace => Some('{'),
            Key::Bar => Some('|'),
            Key::RBrace => Some('}'),
            Key::Tilde => Some('~'),
            _ => None,
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Key::Function(n) => write!(f, "F{}", n),
            Key::Left => write!(f, "←"),
            Key::Up => write!(f, "↑"),
            Key::Right => write!(f, "→"),
            Key::Down => write!(f, "↓"),
            Key::Home => write!(f, "Home"),
            Key::Insert => write!(f, "Ins"),
            Key::Delete => write!(f, "Del"),
            Key::End => write!(f, "End"),
            Key::PageUp => write!(f, "PgUp"),
            Key::PageDown => write!(f, "PgDn"),
            Key::Escape => write!(f, "Esc"),
            Key::Backspace => write!(f, "Backspace"),
            Key::Tab => write!(f, "Tab"),
            Key::Enter => write!(f, "Enter"),
            Key::Space => write!(f, "Space"),
            _ => {
                if let Some(c) = self.to_char() {
                    write!(f, "{}", c)
                } else {
                    write!(f, "")
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Key, KeyState};

    #[test]
    fn from_char() {
        assert_eq!(Key::from_char('a'), Some(Key::a));
        assert_eq!(Key::from_char('\\'), Some(Key::BSlash));
        assert_eq!(Key::from_char('\x1b'), Some(Key::Escape));
    }

    #[test]
    fn to_char() {
        assert_eq!(Key::a.to_char(), Some('a'));
        assert_eq!(Key::BSlash.to_char(), Some('\\'));
        assert_eq!(Key::Escape.to_char(), Some('\x1b'));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Key::a), "a");
        assert_eq!(format!("{}", Key::BSlash), "\\");
        assert_eq!(format!("{}", Key::Escape), "Esc");
        assert_eq!(format!("{}", Key::Left), "←");
    }

    #[test]
    fn keystate_from() {
        assert_eq!(KeyState::from(1usize), KeyState::Press);
        assert_eq!(KeyState::from(2usize), KeyState::Repeat);
        assert_eq!(KeyState::from(3usize), KeyState::Release);
        // Fallback / unknown
        assert_eq!(KeyState::from(0usize), KeyState::Press);
        assert_eq!(KeyState::from(42usize), KeyState::Press);
    }

    #[test]
    fn keystate_display() {
        assert_eq!(format!("{}", KeyState::Press), "");
        assert_eq!(format!("{}", KeyState::Repeat), " (repeat)");
        assert_eq!(format!("{}", KeyState::Release), " (release)");
    }

    #[test]
    fn function_key_display() {
        assert_eq!(format!("{}", Key::Function(5)), "F5");
    }

    #[test]
    fn arrow_key_display() {
        assert_eq!(format!("{}", Key::Up), "↑");
        assert_eq!(format!("{}", Key::Right), "→");
        assert_eq!(format!("{}", Key::Down), "↓");
    }

    #[test]
    fn round_trip_canonical_chars() {
        // For every ASCII char, if we can parse it into a Key and that key can
        // produce a char, then parsing that produced char must yield the same Key.
        for c in 0u8..=127 {
            let ch = c as char;
            if let Some(key) = Key::from_char(ch)
                && let Some(c2) = key.to_char()
            {
                // Special cases where multiple input chars map to one canonical output
                // Backspace: '\x7f' and '\x08' canonicalizes to '\x08'
                if ch == '\x7f' && c2 == '\x08' {
                    assert_eq!(Key::from_char(c2), Some(key));
                    continue;
                }

                // Enter: '\r' canonicalizes to '\n'
                if ch == '\r' && c2 == '\n' {
                    assert_eq!(Key::from_char(c2), Some(key));
                    continue;
                }

                assert_eq!(
                    Key::from_char(c2),
                    Some(key),
                    "char {:x} round trip failed",
                    c
                );
            }
        }
    }

    #[test]
    fn alternative_backspace_and_enter_inputs() {
        assert_eq!(Key::from_char('\x7f'), Some(Key::Backspace));
        assert_eq!(Key::from_char('\x08'), Some(Key::Backspace));
        assert_eq!(Key::from_char('\r'), Some(Key::Enter));
        assert_eq!(Key::from_char('\n'), Some(Key::Enter));
    }

    #[test]
    fn unhandled_char() {
        // Non-ASCII / unmapped char should return None
        assert_eq!(Key::from_char('é'), None);
    }
}
