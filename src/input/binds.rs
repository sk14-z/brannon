use super::{Input, key::Key};
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq)]
pub struct KeyBinds {
    keys: Vec<Input>,
    actions: Vec<String>,
}

impl KeyBinds {
    pub fn new() -> Self {
        KeyBinds {
            keys: vec![],
            actions: vec![],
        }
    }

    pub fn len(&self) -> usize {
        format!("{}", self).len()
    }

    pub fn count(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn bind(mut self, input: Input, action: impl Into<String>) -> Self {
        self.keys.push(input);
        self.actions.push(action.into());
        self
    }
}

impl From<Vec<(Input, String)>> for KeyBinds {
    fn from(v: Vec<(Input, String)>) -> Self {
        let mut s = Self::new();

        for (key, a) in v {
            s.keys.push(key);
            s.actions.push(a);
        }

        s
    }
}

impl<const N: usize> From<[(Input, String); N]> for KeyBinds {
    fn from(binds: [(Input, String); N]) -> Self {
        let mut kb = Self::new();

        for (input, a) in binds {
            kb.keys.push(input);
            kb.actions.push(a);
        }

        kb
    }
}

impl Display for KeyBinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.count() {
            write!(f, " {} <{}>", self.actions[i], self.keys[i])?;
        }

        Ok(())
    }
}
