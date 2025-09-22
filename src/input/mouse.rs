#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Mouse {
    // Click
    Up(MouseButton),
    Down(MouseButton),
    // Movement
    Move,
    Drag(MouseButton),
    // Scroll
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
}
