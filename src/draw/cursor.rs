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

#[derive(PartialEq, Eq)]
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
    if s == CursorShape::None {
        printf!("\x1b[?25l");
    } else {
        printf!("\x1b[?25h");
        printf!("\x1b[{} q", s as usize);
    }
}
