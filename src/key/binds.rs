use std::fmt::Display;

use super::Key;

#[derive(Clone)]
pub struct KeyBinds {
    keys: Vec<Key>,
    actions: Vec<String>,
}

impl From<Vec<(Key, String)>> for KeyBinds {
    fn from(v: Vec<(Key, String)>) -> Self {
        let mut s = Self::new();

        for (key, a) in v {
            s.keys.push(key);
            s.actions.push(a);
        }

        s
    }
}

impl Display for KeyBinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.keys.len() {
            write!(f, " {} <{}>", self.actions[i], self.keys[i])?;
        }

        Ok(())
    }
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

    pub fn add_bind(&mut self, key: Key, action: impl Into<String>) -> &mut Self {
        self.keys.push(key);
        self.actions.push(action.into());
        self
    }

    pub fn remove_bind(&mut self, key: Key) -> &mut Self {
        if let Some(pos) = self.keys.iter().position(|kfc| *kfc == key) {
            self.keys.remove(pos);
            self.actions.remove(pos);
        }
        self
    }

    pub fn edit_bind(&mut self, key: Key) -> Option<(&mut Key, &mut String)> {
        if let Some(pos) = self.keys.iter().position(|kfc| *kfc == key) {
            (self.keys.get_mut(pos), self.actions.get_mut(pos));
        }
        None
    }

    pub fn bind(mut self, key: Key, action: impl Into<String>) -> Self {
        self.add_bind(key, action.into());
        self
    }
}
