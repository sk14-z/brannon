pub mod binds;

use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Key {
    Null,
    SOH,        // Start of Heading
    STX,        // Start of Text
    ETX,        // End of Text
    EOT,        // End of Transmission
    ENQ,        // Enquiry
    ACK,        // Acknowledgement
    BEL,        // Bell
    BS,         // Backspace
    HT,         // Horizontal Tab
    LF,         // Line Feed
    VT,         // Vertical Tab
    FF,         // Form Feed
    CR,         // Carriage Return
    SO,         // Shift Out
    SI,         // Shift In
    DLE,        // Data Link Escape
    DC1,        // Device Control 1
    DC2,        // Device Control 2
    DC3,        // Device Control 3
    DC4,        // Device Control 4
    NAK,        // Negative Acknowledgement
    SYN,        // Synchronous Idle
    ETB,        // End of Transmission Block
    CAN,        // Cancel
    EM,         // End of Medium
    SUB,        // Substitute
    ESC,        // Escape
    FS,         // File Separator
    GS,         // Group Separator
    RS,         // Record Separator
    US,         // Unit Separator
    Space,      // Space
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
    Lt,         // '<'
    Eq,         // '='
    Gt,         // '>'
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
    VBar,       // '|'
    RBrace,     // '}'
    Tilde,      // '~'
    Del,        // '\x7F'
}

impl Key {
    pub fn new(c: char) -> Option<Self> {
        match c {
            '\0' => None,
            '\x01' => Some(Key::SOH),
            '\x02' => Some(Key::STX),
            '\x03' => Some(Key::ETX),
            '\x04' => Some(Key::EOT),
            '\x05' => Some(Key::ENQ),
            '\x06' => Some(Key::ACK),
            '\x07' => Some(Key::BEL),
            '\x08' => Some(Key::BS),
            '\t' => Some(Key::HT),
            '\n' => Some(Key::LF),
            '\x0B' => Some(Key::VT),
            '\x0C' => Some(Key::FF),
            '\r' => Some(Key::CR),
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
            '<' => Some(Key::Lt),
            '=' => Some(Key::Eq),
            '>' => Some(Key::Gt),
            '?' => Some(Key::QMark),
            '@' => Some(Key::t),
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
            '|' => Some(Key::VBar),
            '}' => Some(Key::RBrace),
            '~' => Some(Key::Tilde),
            '\x7F' => Some(Key::Del),
            _ => None,
        }
    }

    pub fn to_char(&self) -> Option<char> {
        match self {
            Key::Null => None,
            Key::SOH => Some('\x01'),
            Key::STX => Some('\x02'),
            Key::ETX => Some('\x03'),
            Key::EOT => Some('\x04'),
            Key::ENQ => Some('\x05'),
            Key::ACK => Some('\x06'),
            Key::BEL => Some('\x07'),
            Key::BS => Some('\x08'),
            Key::HT => Some('\t'),
            Key::LF => Some('\n'),
            Key::VT => Some('\x0B'),
            Key::FF => Some('\x0C'),
            Key::CR => Some('\r'),
            Key::SO => None,
            Key::SI => None,
            Key::DLE => None,
            Key::DC1 => None,
            Key::DC2 => None,
            Key::DC3 => None,
            Key::DC4 => None,
            Key::NAK => None,
            Key::SYN => None,
            Key::ETB => None,
            Key::CAN => None,
            Key::EM => None,
            Key::SUB => None,
            Key::ESC => Some('\x1B'),
            Key::FS => None,
            Key::GS => None,
            Key::RS => None,
            Key::US => None,
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
            Key::Lt => Some('<'),
            Key::Eq => Some('='),
            Key::Gt => Some('>'),
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
            Key::VBar => Some('|'),
            Key::RBrace => Some('}'),
            Key::Tilde => Some('~'),
            Key::Del => Some('\x7F'),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Key::LF {
            write!(f, "Enter")
        } else if let Some(c) = self.to_char() {
            write!(f, "{}", c)
        } else {
            write!(f, "")
        }
    }
}
