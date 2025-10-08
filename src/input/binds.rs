use super::Input;
use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub struct Binds {
    binds: Vec<Input>,
    actions: Vec<String>,
}

impl Binds {
    pub fn new() -> Self {
        Binds {
            binds: vec![],
            actions: vec![],
        }
    }

    pub fn len(&self) -> usize {
        format!("{}", self).len()
    }

    pub fn count(&self) -> usize {
        self.binds.len()
    }

    pub fn is_empty(&self) -> bool {
        self.binds.is_empty()
    }

    pub fn bind(mut self, input: Input, action: impl Into<String>) -> Self {
        self.binds.push(input);
        self.actions.push(action.into());
        self
    }
}

impl From<Vec<(Input, String)>> for Binds {
    fn from(v: Vec<(Input, String)>) -> Self {
        let mut s = Self::new();

        for (key, a) in v {
            s.binds.push(key);
            s.actions.push(a);
        }

        s
    }
}

impl<const N: usize> From<[(Input, String); N]> for Binds {
    fn from(binds: [(Input, String); N]) -> Self {
        let mut kb = Self::new();

        for (input, a) in binds {
            kb.binds.push(input);
            kb.actions.push(a);
        }

        kb
    }
}

impl Display for Binds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, action) in self.binds.iter().zip(self.actions.iter()) {
            write!(f, "<{}> {}", key, action)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Binds;
    use crate::input::key::{Key, KeyState};
    use crate::input::modifier::Modifier;

    #[test]
    fn new_is_empty() {
        let kb = Binds::new();
        assert!(kb.is_empty());
        assert_eq!(kb.count(), 0);
        assert_eq!(kb.len(), 0);
        assert_eq!(format!("{}", kb), "");
    }

    #[test]
    fn single_bind_and_display_len() {
        let kb = Binds::new().bind(Key::a.into(), "InsertA");
        assert!(!kb.is_empty());
        assert_eq!(kb.count(), 1);
        let expected = "<a> InsertA";
        assert_eq!(format!("{}", kb), expected);
        assert_eq!(kb.len(), expected.len());
    }

    #[test]
    fn bind_chaining() {
        let kb = Binds::new()
            .bind((Key::a, Modifier::Ctrl).into(), "ActionA")
            .bind(Key::b.into(), "ActionB");
        assert_eq!(kb.count(), 2);
        let s = format!("{}", kb);
        assert!(s.contains("<Ctrl + a> ActionA"));
        assert!(s.contains("<b> ActionB"));
    }

    #[test]
    fn from_vec() {
        let v = vec![
            ((Key::a, Modifier::Ctrl).into(), "A".to_string()),
            (Key::b.into(), "B".to_string()),
        ];
        let kb: Binds = v.clone().into();
        assert_eq!(kb.count(), 2);
        let s = format!("{}", kb);
        assert!(s.contains("<Ctrl + a> A"));
        assert!(s.contains("<b> B"));
    }

    #[test]
    fn from_array() {
        let kb: Binds = [
            ((Key::a, Modifier::Alt).into(), "AltA".to_string()),
            ((Key::Enter, KeyState::Press).into(), "Enter".to_string()),
        ]
        .into();
        assert_eq!(kb.count(), 2);
        let s = format!("{}", kb);
        assert!(s.contains("<Alt + a> AltA"));
        assert!(s.contains("<Enter> Enter"));
    }

    #[test]
    fn equality_and_clone() {
        let kb1 = Binds::new()
            .bind(Key::a.into(), "A")
            .bind((Key::b, Modifier::Shift).into(), "B");
        let mut kb2 = kb1.clone();
        assert_eq!(kb1, kb2);
        kb2 = kb2.bind(Key::c.into(), "C");
        assert_ne!(kb1, kb2);
    }

    #[test]
    fn len_matches_display() {
        let kb = Binds::new()
            .bind(Key::a.into(), "A")
            .bind((Key::b, Modifier::Ctrl).into(), "B");
        assert_eq!(kb.len(), format!("{}", kb).len());
    }
}
