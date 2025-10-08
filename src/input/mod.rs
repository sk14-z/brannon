pub mod binds;
pub mod key;
pub(crate) mod mask;
pub mod modifier;
pub mod mouse;

use key::{Key, KeyState};
use mask::Mask;
use modifier::{Modifier, ModifierList};
use mouse::{Mouse, MouseState};
use std::{fmt::Display, str};

use crate::unit::Point;

#[derive(Clone, PartialEq, Debug)]
pub enum Input {
    Key(Key, KeyState, ModifierList),
    Mouse(Mouse, MouseState, ModifierList, Point),
}

impl From<Key> for Input {
    fn from(key: Key) -> Self {
        Input::Key(key, KeyState::Press, [].into())
    }
}

impl From<(Key, KeyState)> for Input {
    fn from((key, state): (Key, KeyState)) -> Self {
        Input::Key(key, state, [].into())
    }
}

impl<T: Into<ModifierList>> From<(Key, T)> for Input {
    fn from((key, mods): (Key, T)) -> Self {
        Input::Key(key, KeyState::Press, mods.into())
    }
}

impl<T: Into<ModifierList>> From<(Key, KeyState, T)> for Input {
    fn from((key, state, mods): (Key, KeyState, T)) -> Self {
        Input::Key(key, state, mods.into())
    }
}

impl From<Mouse> for Input {
    fn from(mouse: Mouse) -> Self {
        Input::Mouse(
            mouse,
            match mouse {
                Mouse::Left | Mouse::Middle | Mouse::Right => MouseState::Click,
                _ => MouseState::Scroll,
            },
            [].into(),
            ().into(),
        )
    }
}

impl From<(Mouse, MouseState)> for Input {
    fn from((mouse, state): (Mouse, MouseState)) -> Self {
        Input::Mouse(mouse, state, [].into(), ().into())
    }
}

impl<T: Into<ModifierList>> From<(Mouse, T)> for Input {
    fn from((mouse, mods): (Mouse, T)) -> Self {
        Input::Mouse(
            mouse,
            match mouse {
                Mouse::Left | Mouse::Middle | Mouse::Right => MouseState::Click,
                _ => MouseState::Scroll,
            },
            mods.into(),
            ().into(),
        )
    }
}

impl<T: Into<ModifierList>> From<(Mouse, MouseState, T)> for Input {
    fn from((mouse, state, mods): (Mouse, MouseState, T)) -> Self {
        Input::Mouse(mouse, state, mods.into(), ().into())
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Key(key, state, mods) => {
                if mods.0.is_empty()
                    || (*mods == ModifierList(vec![Modifier::Shift])
                        && (' '..='~').contains(&key.to_char().unwrap_or('\0')))
                {
                    write!(f, "{}{}", key, state)
                } else {
                    write!(f, "{} + {}{}", mods, key, state)
                }
            }
            Input::Mouse(btn, state, mods, _) => {
                if mods.0.is_empty() {
                    write!(f, "{}{}", btn, state)
                } else {
                    write!(f, "{} + {}{}", mods, btn, state)
                }
            }
        }
    }
}

macro_rules! single_byte_key_pat {
    () => {
        0x1b | 0x08 | b'\t' | b'\n' | 0x20..=0x7f
    };
}

// Parse into full input event
// Panics represent where errors would be returned when implemented
pub fn parse(buf: &[u8]) -> Option<Input> {
    if buf.is_empty() {
        None
    } else {
        match buf[0] {
            // Keys with alt
            0x1b if buf.len() == 2 => {
                if let Some(Input::Key(key, KeyState::Press, mods)) = parse(&buf[1..]) {
                    Some((key, mods + Modifier::Alt).into())
                } else {
                    None
                }
            }

            // Key sent as escape sequence
            0x1b if buf.len() > 2 => match buf[1] {
                b'[' => match buf[buf.len() - 1] {
                    // Functional Key
                    b'~' if buf.len() > 3 => {
                        let val = if buf[3].is_ascii_digit() {
                            format!("{}{}", buf[2] - b'0', buf[3] - b'0')
                        } else {
                            format!("{}", buf[2] - b'0')
                        };

                        match val.parse::<u8>().unwrap_or_else(|_| panic!()) {
                            1 => Some(Key::Home.into()),
                            2 => Some(Key::Insert.into()),
                            3 => Some(Key::Delete.into()),
                            4 => Some(Key::End.into()),
                            5 => Some(Key::PageUp.into()),
                            6 => Some(Key::PageDown.into()),
                            v @ 11..=15 => Some(Key::Function(v - 10).into()),
                            v @ 17..=21 => Some(Key::Function(v - 11).into()),
                            v @ 23..=24 => Some(Key::Function(v - 12).into()),
                            _ => panic!("unknown tilde csi: {val:?} from {buf:x?}\n",),
                        }
                    }

                    // Kitty keyboard protocol
                    // Sent as \x1b[key_code:alt_key_code;modifier_mask:event_typeu
                    b'u' => {
                        let mut params = str::from_utf8(&buf[2..])
                            .unwrap_or_else(|_| panic!())
                            // Temporary solution to avoid crashing when multiple key presses are
                            // sent at once. Eventually, will be handled separately and split
                            // before passed.
                            .split('u')
                            .next()
                            .unwrap_or_else(|| panic!())
                            .split(';');

                        let maybe_key = if let Some(code) = params.next() {
                            if let Some((_, alt_code)) = code.split_once(':')
                                && let Ok(key @ single_byte_key_pat!()) = alt_code.parse::<u8>()
                            {
                                Key::from_char(key as char)
                            } else if let Ok(key @ single_byte_key_pat!()) = code.parse::<u8>() {
                                Key::from_char(key as char)
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        let (mods, state) = if let Some(key_info) = params.next() {
                            let mut key_info = key_info.split(":");

                            if let Some(mask) = key_info.next() {
                                (
                                    ModifierList::unmask(
                                        mask.parse::<usize>().unwrap_or_else(|_| panic!()),
                                    ),
                                    if let Some(state) = key_info.next() {
                                        KeyState::from(
                                            state.parse::<u8>().unwrap_or_else(|_| panic!()),
                                        )
                                    } else {
                                        KeyState::Press
                                    },
                                )
                            } else {
                                ([].into(), KeyState::Press)
                            }
                        } else {
                            ([].into(), KeyState::Press)
                        };

                        maybe_key.map(|key| Input::Key(key, state, mods))
                    }

                    // Mouse Input (digits format)
                    // Sent as \x1b[<button;col;rowM or \x1b[<button;col;rowm
                    b @ (b'M' | b'm') => {
                        assert!(buf.starts_with(b"\x1b[<"));

                        let params: Vec<usize> = str::from_utf8(&buf[3..])
                            .unwrap_or_else(|_| panic!())
                            // Same here, temporary solution to avoid crashing
                            .split(b as char)
                            .next()
                            .unwrap_or_else(|| panic!())
                            .split(';')
                            .map(|n| n.parse::<usize>().unwrap_or_else(|_| panic!()))
                            .collect();

                        if params.len() == 3 {
                            let (mask, col, row) = (params[0], params[1], params[2]);
                            let (btn, mut state, mods) = Mouse::unmask(mask);

                            if b == b'm' {
                                state = MouseState::Release;
                            }

                            Some(Input::Mouse(btn, state, mods, (col, row).into()))
                        } else {
                            None
                        }
                    }

                    _ => match buf[2] {
                        b'A' => Some(Key::Up.into()),
                        b'B' => Some(Key::Down.into()),
                        b'C' => Some(Key::Right.into()),
                        b'D' => Some(Key::Left.into()),
                        b'H' => Some(Key::Home.into()),
                        b'F' => Some(Key::End.into()),
                        b'P' => Some(Key::Function(1).into()),
                        b'Q' => Some(Key::Function(2).into()),
                        b'R' => Some(Key::Function(4).into()),
                        // Focus Gain/Loss
                        // b'I' => {}
                        // b'O' => {}
                        b'Z' => Some((Key::Tab, Modifier::Shift).into()),

                        _ => None,
                    },
                },
                b'O' => match buf[2] {
                    b'A' => Some(Key::Up.into()),
                    b'B' => Some(Key::Down.into()),
                    b'C' => Some(Key::Right.into()),
                    b'D' => Some(Key::Left.into()),
                    b'H' => Some(Key::Home.into()),
                    b'F' => Some(Key::End.into()),
                    v @ b'P'..=b'S' => Some(Key::Function(v - b'O').into()),
                    _ => panic!("could not parse escape sequence: {buf:x?}\n"),
                },
                _ => None,
            },

            // Keys sent in a single character
            c @ single_byte_key_pat!() => {
                Some(Key::from_char(c as char).or_else(|| panic!())?.into())
            }

            // Key with control
            c @ 0x01..=0x1A => Some(
                (
                    Key::from_char((c + 0x60) as char).or_else(|| panic!())?,
                    Modifier::Ctrl,
                )
                    .into(),
            ),

            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", Input::Key(Key::a, KeyState::Press, [].into())),
            "a"
        );

        assert_eq!(
            format!(
                "{}",
                Input::Key(Key::a, KeyState::Press, Modifier::Ctrl.into())
            ),
            "Ctrl + a"
        );

        assert_eq!(
            format!(
                "{}",
                Input::Key(
                    Key::a,
                    KeyState::Press,
                    [Modifier::Ctrl, Modifier::Alt].into()
                )
            ),
            "Ctrl + Alt + a"
        );

        // todo
        // assert_eq!(format!("{}", Input::Mouse(...), "...");
    }

    #[test]
    fn equality() {
        let input1 = Input::from((Key::a, Modifier::Ctrl));
        let input2 = Input::from((Key::a, Modifier::Ctrl));
        let input3 = Input::from((Key::A, KeyState::Repeat, [Modifier::Ctrl, Modifier::Alt]));

        assert!(input1 == input2);
        assert!(input1 != input3);

        if let Input::Key(k1, s1, m1) = input1
            && let Input::Key(k2, s2, m2) = input2
            && let Input::Key(k3, s3, m3) = input3
        {
            assert!(k1 == k2);
            assert!(s1 == s2);
            assert!(m1 == m2);

            assert!(k1 != k3);
            assert!(s1 != s3);
            assert!(m1 != m3);
        }
    }

    #[test]
    fn matching() {
        let input: Input = (Key::a, Modifier::Ctrl).into();

        assert!(matches!(
            input,
            Input::Key(Key::a, _, mods) if mods == Modifier::Ctrl
        ));
    }

    #[test]
    fn parse_single_char() {
        assert!(parse(b"\x1b") == Some(Key::Escape.into()));
        assert!(parse(b"\n") == Some(Key::Enter.into()));
        assert!(parse(b"a") == Some(Key::a.into()));
        assert!(parse(b"\x01") == Some((Key::a, Modifier::Ctrl).into()));
        assert!(parse(b"\x09") != Some((Key::i, Modifier::Ctrl).into()));
    }

    #[test]
    fn parse_tidle() {
        assert!(parse(b"\x1b[~").is_none());
        assert!(parse(b"\x1b[1~") == Some(Key::Home.into()));
        assert!(parse(b"\x1b[3~") == Some(Key::Delete.into()));
        assert!(parse(b"\x1b[13~") == Some(Key::Function(3).into()));
        assert!(parse(b"\x1b[24~") == Some(Key::Function(12).into()));
    }

    #[test]
    #[should_panic]
    fn parse_invalid() {
        parse(b"\x1b[30~");
    }

    #[test]
    fn parse_mods() {
        assert!(parse(b"\x01") == Some((Key::a, [Modifier::Ctrl]).into()));
        assert!(parse(b"\x1bc") == Some((Key::c, [Modifier::Alt]).into()));
        assert!(parse(b"\x1b\x1b") == Some((Key::Escape, [Modifier::Alt]).into()));
    }

    #[test]
    fn parse_kitty_protocol() {
        assert!(parse(b"\x1b[97u") == Some(Key::a.into()));
        assert!(
            parse(b"\x1b[9;2:3u") == Some((Key::Tab, KeyState::Release, Modifier::Shift).into())
        );
        assert!(parse(b"\x1b[97:65;2u") == Some((Key::A, Modifier::Shift).into()));
        assert!(parse(b"\x1b[97;5u") == Some((Key::a, Modifier::Ctrl).into()));
        assert!(parse(b"\x1b[97;5:2u") == Some((Key::a, KeyState::Repeat, Modifier::Ctrl).into()));
    }

    #[test]
    fn parse_mouse_basic() {
        assert!(
            parse(b"\x1b[<0;10;20M")
                == Some(Input::Mouse(
                    Mouse::Left,
                    MouseState::Click,
                    [].into(),
                    (10, 20).into()
                ))
        );

        assert!(
            parse(b"\x1b[<0;10;20m")
                == Some(Input::Mouse(
                    Mouse::Left,
                    MouseState::Release,
                    [].into(),
                    (10, 20).into()
                ))
        );

        assert!(
            parse(b"\x1b[<42;5;6M")
                == Some(Input::Mouse(
                    Mouse::Right,
                    MouseState::Drag,
                    Modifier::Alt.into(),
                    (5, 6).into()
                ))
        );

        assert!(
            parse(b"\x1b[<64;30;12M")
                == Some(Input::Mouse(
                    Mouse::WheelUp,
                    MouseState::Scroll,
                    [].into(),
                    (30, 12).into()
                ))
        );
    }

    #[test]
    fn parse_mouse_modifiers() {
        assert!(
            parse(b"\x1b[<28;1;1M")
                == Some(Input::Mouse(
                    Mouse::Left,
                    MouseState::Click,
                    [Modifier::Shift, Modifier::Alt, Modifier::Ctrl].into(),
                    (1, 1).into()
                ))
        );

        assert!(
            parse(b"\x1b[<86;4;7M")
                == Some(Input::Mouse(
                    Mouse::WheelLeft,
                    MouseState::Scroll,
                    [Modifier::Shift, Modifier::Ctrl].into(),
                    (4, 7).into()
                ))
        );

        assert!(
            parse(b"\x1b[<4;2;3m")
                == Some(Input::Mouse(
                    Mouse::Left,
                    MouseState::Release,
                    Modifier::Shift.into(),
                    (2, 3).into()
                ))
        );
    }
}
