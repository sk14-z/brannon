use std::{char, fmt};

pub enum BoxChar {
    // Light line box drawing characters
    LightH = 0x2500,       // '─' Horizontal
    LightV = 0x2502,       // '│' Vertical
    LightTL = 0x250C,      // '┌' Top-left corner
    LightTR = 0x2510,      // '┐' Top-right corner
    LightBL = 0x2514,      // '└' Bottom-left corner
    LightBR = 0x2518,      // '┘' Bottom-right corner
    LightCross = 0x253C,   // '┼' Cross (intersection)
    LightTJUp = 0x252C,    // '┬' T-junction up
    LightTJDown = 0x2534,  // '┴' T-junction down
    LightTJLeft = 0x251C,  // '├' T-junction left
    LightTJRight = 0x2524, // '┤' T-junction right

    // Heavy line box drawing characters
    HeavyH = 0x2501,       // '━' Horizontal
    HeavyV = 0x2503,       // '┃' Vertical
    HeavyTL = 0x250F,      // '┏' Top-left corner
    HeavyTR = 0x2513,      // '┓' Top-right corner
    HeavyBL = 0x2517,      // '┗' Bottom-left corner
    HeavyBR = 0x251B,      // '┛' Bottom-right corner
    HeavyCross = 0x254B,   // '╋' Cross (intersection)
    HeavyTJUp = 0x2533,    // '┳' T-junction up
    HeavyTJDown = 0x253B,  // '┻' T-junction down
    HeavyTJLeft = 0x2523,  // '┣' T-junction left
    HeavyTJRight = 0x252B, // '┫' T-junction right

    // Double-line box drawing characters
    DoubleH = 0x2550,       // '═' Horizontal
    DoubleV = 0x2551,       // '║' Vertical
    DoubleTL = 0x2554,      // '╔' Top-left corner
    DoubleTR = 0x2557,      // '╗' Top-right corner
    DoubleBL = 0x255A,      // '╚' Bottom-left corner
    DoubleBR = 0x255D,      // '╝' Bottom-right corner
    DoubleCross = 0x256C,   // '╬' Cross (intersection)
    DoubleTJUp = 0x2566,    // '╦' T-junction up
    DoubleTJDown = 0x2569,  // '╩' T-junction down
    DoubleTJLeft = 0x2560,  // '╠' T-junction left
    DoubleTJRight = 0x2563, // '╣' T-junction right

    // Arcs
    ArcTL = 0x256D, // '╭' Top-left arc
    ArcTR = 0x256E, // '╮' Top-right arc
    ArcBL = 0x2570, // '╰' Bottom-left arc
    ArcBR = 0x256F, // '╯' Bottom-right arc

    // Diagonal lines
    DiagonalLR = 0x2571,    // '╱' Diagonal left-to-right
    DiagonalRL = 0x2572,    // '╲' Diagonal right-to-left
    CrossDiagonal = 0x2573, // '╳' Crossed diagonals
}

impl BoxChar {
    pub fn to_char(&self) -> char {
        match self {
            BoxChar::LightH => '─',
            BoxChar::LightV => '│',
            BoxChar::LightTL => '┌',
            BoxChar::LightTR => '┐',
            BoxChar::LightBL => '└',
            BoxChar::LightBR => '┘',
            BoxChar::LightCross => '┼',
            BoxChar::LightTJUp => '┬',
            BoxChar::LightTJDown => '┴',
            BoxChar::LightTJLeft => '├',
            BoxChar::LightTJRight => '┤',
            BoxChar::HeavyH => '━',
            BoxChar::HeavyV => '┃',
            BoxChar::HeavyTL => '┏',
            BoxChar::HeavyTR => '┓',
            BoxChar::HeavyBL => '┗',
            BoxChar::HeavyBR => '┛',
            BoxChar::HeavyCross => '╋',
            BoxChar::HeavyTJUp => '┳',
            BoxChar::HeavyTJDown => '┻',
            BoxChar::HeavyTJLeft => '┣',
            BoxChar::HeavyTJRight => '┫',
            BoxChar::DoubleH => '═',
            BoxChar::DoubleV => '║',
            BoxChar::DoubleTL => '╔',
            BoxChar::DoubleTR => '╗',
            BoxChar::DoubleBL => '╚',
            BoxChar::DoubleBR => '╝',
            BoxChar::DoubleCross => '╬',
            BoxChar::DoubleTJUp => '╦',
            BoxChar::DoubleTJDown => '╩',
            BoxChar::DoubleTJLeft => '╠',
            BoxChar::DoubleTJRight => '╣',
            BoxChar::ArcTL => '╭',
            BoxChar::ArcTR => '╮',
            BoxChar::ArcBL => '╰',
            BoxChar::ArcBR => '╯',
            BoxChar::DiagonalLR => '╱',
            BoxChar::DiagonalRL => '╲',
            BoxChar::CrossDiagonal => '╳',
        }
    }
}

impl fmt::Display for BoxChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
