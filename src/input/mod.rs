pub mod binds;
pub mod key;
pub mod modifier;
pub mod mouse;

use key::{Key, KeyState};
use modifier::{Modifier, ModifierList};
use mouse::Mouse;
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq)]
pub enum Input {
    Key(Key, KeyState, ModifierList),
    Mouse(Mouse),
}

impl Input {
    pub fn is_key(&self) -> bool {
        matches!(self, Input::Key(_, _, _))
    }

    pub fn is_mouse(&self) -> bool {
        matches!(self, Input::Mouse(_))
    }

    pub fn key(key: Key) -> Self {
        Input::Key(key, KeyState::Press, [].into())
    }

    pub fn key_press(key: Key, mods: impl Into<ModifierList>) -> Self {
        Input::Key(key, KeyState::Press, mods.into())
    }

    pub fn key_release(key: Key, mods: impl Into<ModifierList>) -> Self {
        Input::Key(key, KeyState::Release, mods.into())
    }

    pub fn key_repeat(key: Key, mods: impl Into<ModifierList>) -> Self {
        Input::Key(key, KeyState::Repeat, mods.into())
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Key(key, _, mods) => write!(f, "{}{}", mods, key),
            Input::Mouse(_) => write!(f, "todo"),
        }
    }
}

// Parse into full input event
pub fn parse(buf: &[u8]) -> Option<Input> {
    if buf.is_empty() {
        None
    } else {
        match buf[0] {
            // Escape key
            b'\x1b' if buf.len() == 1 => Some(Input::key(Key::Escape)),

            // Key sent as escape sequence
            b'\x1b' if buf.len() != 2 => match buf[1] {
                b'[' => match buf[buf.len() - 1] {
                    b'~' if buf.len() > 3 => {
                        let val = if buf[3].is_ascii_digit() {
                            format!("{}{}", buf[2] - b'0', buf[3] - b'0')
                        } else {
                            format!("{}", buf[2] - b'0')
                        };

                        match val.parse::<usize>().unwrap_or_else(|_| panic!()) {
                            1 => Some(Input::key(Key::Home)),
                            2 => Some(Input::key(Key::Insert)),
                            3 => Some(Input::key(Key::Delete)),
                            4 => Some(Input::key(Key::End)),
                            5 => Some(Input::key(Key::PageUp)),
                            6 => Some(Input::key(Key::PageDown)),
                            v @ 11..=15 => Some(Input::key(Key::Function(v - 10))),
                            v @ 17..=21 => Some(Input::key(Key::Function(v - 11))),
                            v @ 23..=24 => Some(Input::key(Key::Function(v - 12))),
                            _ => panic!("unknown tilde csi: {val:?} from {buf:x?}\n",),
                        }
                    }
                    _ => match buf[2] {
                        b'A' => Some(Input::key(Key::Up)),
                        b'B' => Some(Input::key(Key::Down)),
                        b'C' => Some(Input::key(Key::Right)),
                        b'D' => Some(Input::key(Key::Left)),
                        b'H' => Some(Input::key(Key::Home)),
                        b'F' => Some(Input::key(Key::End)),
                        _ => None,
                    },
                },
                b'O' => match buf[2] {
                    b'A' => Some(Input::key(Key::Up)),
                    b'B' => Some(Input::key(Key::Down)),
                    b'C' => Some(Input::key(Key::Right)),
                    b'D' => Some(Input::key(Key::Left)),
                    b'H' => Some(Input::key(Key::Home)),
                    b'F' => Some(Input::key(Key::End)),
                    v @ b'P'..=b'S' => Some(Input::key(Key::Function((v - b'O') as usize))),
                    _ => None,
                },
                _ => None,
            },

            // Keys with dedicated characters
            b'\x7f' | b'\x08' => Some(Input::key(Key::Backspace)),
            b'\t' => Some(Input::key(Key::Tab)),
            b'\n' => Some(Input::key(Key::Enter)),

            // Key with control
            // Not all keys are captured this way. This is unavoidable.
            // For example, Ctrl + H-K return the above 3 values instead.
            c @ b'\x01'..=b'\x1A' => Some(Input::key_press(
                Key::from_char((c + b'`') as char).or_else(|| panic!())?,
                Modifier::Ctrl,
            )),

            // Keys sent as themselves
            c @ 0x20..=0x7E => Some(Input::key(Key::from_char(c as char).or_else(|| panic!())?)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        Input,
        key::{Key, KeyState},
        modifier::Modifier,
        parse,
    };

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
        let input1 = Input::key_press(Key::a, Modifier::Ctrl);
        let input2 = Input::key_press(Key::a, Modifier::Ctrl);
        let input3 = Input::key_release(Key::A, [Modifier::Ctrl, Modifier::Alt]);

        assert!(input1 == input2);
        assert!(input1 != input3);

        assert!(if let Input::Key(k1, s1, m1) = input1
            && let Input::Key(k2, s2, m2) = input2
            && let Input::Key(k3, s3, m3) = input3
        {
            (k1 == k2 && s1 == s2 && m1 == m2) && (k1 != k3 && s1 != s3 && m1 != m3)
        } else {
            false
        });
    }

    #[test]
    fn matching() {
        let input = Input::key_press(Key::a, Modifier::Ctrl);

        assert!(matches!(
            input,
            Input::Key(Key::a, _, mods) if mods == Modifier::Ctrl
        ));
    }

    #[test]
    fn parse_single_char() {
        assert!(parse(b"\x1b") == Some(Input::key(Key::Escape)));
        assert!(parse(b"\n") == Some(Input::key(Key::Enter)));
        assert!(parse(b"a") == Some(Input::key(Key::a)));
        assert!(parse(b"\x01") == Some(Input::key_press(Key::a, Modifier::Ctrl)));
        assert!(parse(b"\x09") != Some(Input::key_press(Key::i, Modifier::Ctrl)));
    }

    #[test]
    fn parse_tidle() {
        assert!(parse(b"\x1b[~").is_none());
        assert!(parse(b"\x1b[1~") == Some(Input::key(Key::Home)));
        assert!(parse(b"\x1b[3~") == Some(Input::key(Key::Delete)));
        assert!(parse(b"\x1b[13~") == Some(Input::key(Key::Function(3))));
        assert!(parse(b"\x1b[24~") == Some(Input::key(Key::Function(12))));
    }

    #[test]
    #[should_panic]
    fn parse_invalid() {
        parse(b"\x1b[30~");
    }
}
