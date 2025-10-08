use std::fmt::Display;

use super::{
    mask::Mask,
    modifier::{Modifier, ModifierList},
};

// Reported as ESC [ < btn ; col ; row M (m if release)
// The upper two bits of btn are translated one byte up
// Ex. scroll wheel down has a button id of 5 (0101), this is sent as 65 (0100 0001)
// Modifiers are also given different bit values than with keys

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MouseState {
    Click,
    Scroll,
    Release,
    Drag,
}

impl Display for MouseState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MouseState::Click | MouseState::Scroll => write!(f, ""),
            MouseState::Release => write!(f, " (release)"),
            MouseState::Drag => write!(f, " (drag)"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mouse {
    Left = 0b0000_0000,
    Middle = 0b0000_0001,
    Right = 0b0000_0010,
    WheelUp = 0b0100_0000,
    WheelDown = 0b0100_0001,
    WheelLeft = 0b0100_0010,
    WheelRight = 0b0100_0011,
}

impl From<usize> for Mouse {
    fn from(value: usize) -> Self {
        match value {
            0b0000_0000 => Mouse::Left,
            0b0000_0001 => Mouse::Middle,
            0b0000_0010 => Mouse::Right,
            0b0100_0000 => Mouse::WheelUp,
            0b0100_0001 => Mouse::WheelDown,
            0b0100_0010 => Mouse::WheelLeft,
            0b0100_0011 => Mouse::WheelRight,
            _ => Mouse::Left, // Fallback to left button on unknown value
        }
    }
}

const MOUSE_SHIFT: usize = 0b0000_0100;
const MOUSE_ALT: usize = 0b0000_1000;
const MOUSE_CTRL: usize = 0b0001_0000;
const MOUSE_DRAG: usize = 0b0010_0000;

impl Mask for Mouse {
    type Output = (Mouse, MouseState, ModifierList);

    fn mask(&self) -> usize {
        *self as usize
    }

    fn unmask(mask: usize) -> Self::Output {
        // Remove modifier bits
        let btn = (mask & !(MOUSE_SHIFT | MOUSE_ALT | MOUSE_CTRL | MOUSE_DRAG)).into();

        let state = {
            if mask & MOUSE_DRAG != 0 {
                MouseState::Drag
            } else {
                match btn {
                    Mouse::Left | Mouse::Middle | Mouse::Right => MouseState::Click,
                    _ => MouseState::Scroll,
                }
            }
        };

        let mut mods: ModifierList = [].into();

        if mask & MOUSE_SHIFT != 0 {
            mods += Modifier::Shift;
        }

        if mask & MOUSE_ALT != 0 {
            mods += Modifier::Alt;
        }

        if mask & MOUSE_CTRL != 0 {
            mods += Modifier::Ctrl;
        }

        (btn, state, mods)
    }
}

impl Display for Mouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mouse::Left => write!(f, "Left Click"),
            Mouse::Middle => write!(f, "Middle Click"),
            Mouse::Right => write!(f, "Right Click"),
            Mouse::WheelUp => write!(f, "Scroll Up"),
            Mouse::WheelDown => write!(f, "Scroll Down"),
            Mouse::WheelLeft => write!(f, "Scroll Left"),
            Mouse::WheelRight => write!(f, "Scroll Right"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_buttons_and_mods() {
        assert_eq!(
            Mouse::unmask(0),
            (Mouse::Left, MouseState::Click, [].into())
        );
        assert_eq!(
            Mouse::unmask(8),
            (Mouse::Left, MouseState::Click, Modifier::Alt.into())
        );
        assert_eq!(
            Mouse::unmask(25),
            (
                Mouse::Middle,
                MouseState::Click,
                [Modifier::Alt, Modifier::Ctrl].into()
            )
        );
        assert_eq!(
            Mouse::unmask(42),
            (Mouse::Right, MouseState::Drag, Modifier::Alt.into())
        );
    }

    #[test]
    fn wheel_events() {
        assert_eq!(
            Mouse::unmask(64),
            (Mouse::WheelUp, MouseState::Scroll, [].into())
        );
        assert_eq!(
            Mouse::unmask(65),
            (Mouse::WheelDown, MouseState::Scroll, [].into())
        );
        assert_eq!(
            Mouse::unmask(86), // WheelLeft + Shift + Ctrl (66 + 4 + 16)
            (
                Mouse::WheelLeft,
                MouseState::Scroll,
                [Modifier::Shift, Modifier::Ctrl].into()
            )
        );
        assert_eq!(
            Mouse::unmask(75), // WheelRight + Alt (67 + 8)
            (Mouse::WheelRight, MouseState::Scroll, Modifier::Alt.into())
        );
    }

    #[test]
    fn all_modifiers_left() {
        assert_eq!(
            Mouse::unmask(28), // Left + Shift + Alt + Ctrl
            (
                Mouse::Left,
                MouseState::Click,
                [Modifier::Shift, Modifier::Alt, Modifier::Ctrl].into()
            )
        );
    }

    #[test]
    fn move_state() {
        // Right + Drag + Shift + Alt
        let mask = 2 + 32 + 4 + 8;
        assert_eq!(
            Mouse::unmask(mask),
            (
                Mouse::Right,
                MouseState::Drag,
                [Modifier::Shift, Modifier::Alt].into()
            )
        );
    }

    #[test]
    fn display_mouse_variants() {
        assert_eq!(format!("{}", Mouse::Left), "Left Click");
        assert_eq!(format!("{}", Mouse::Middle), "Middle Click");
        assert_eq!(format!("{}", Mouse::Right), "Right Click");
        assert_eq!(format!("{}", Mouse::WheelUp), "Scroll Up");
        assert_eq!(format!("{}", Mouse::WheelDown), "Scroll Down");
        assert_eq!(format!("{}", Mouse::WheelLeft), "Scroll Left");
        assert_eq!(format!("{}", Mouse::WheelRight), "Scroll Right");
    }

    #[test]
    fn display_mouse_states_in_input() {
        let no_mods =
            crate::input::Input::Mouse(Mouse::Left, MouseState::Click, [].into(), (1, 1).into());
        assert_eq!(format!("{}", no_mods), "Left Click");

        let drag = crate::input::Input::Mouse(
            Mouse::Right,
            MouseState::Drag,
            [Modifier::Ctrl].into(),
            (2, 3).into(),
        );
        assert_eq!(format!("{}", drag), "Ctrl + Right Click (drag)");

        let release = crate::input::Input::Mouse(
            Mouse::Left,
            MouseState::Release,
            [Modifier::Shift, Modifier::Alt].into(),
            (5, 8).into(),
        );
        assert_eq!(format!("{}", release), "Shift + Alt + Left Click (release)");

        let scroll =
            crate::input::Input::Mouse(Mouse::WheelUp, MouseState::Scroll, [].into(), ().into());
        assert_eq!(format!("{}", scroll), "Scroll Up");
    }
}
