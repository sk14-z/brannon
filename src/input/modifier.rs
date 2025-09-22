use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Modifier {
    Ctrl,
    Alt,
    // Shift will not be sent by itself, or with Control (captured by terminal).
    // Including a modifier with only Shift is redundant, as it is implied by the Key variant.
    Shift,
    Super,
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Modifier::Ctrl => write!(f, "Ctrl"),
            Modifier::Alt => write!(f, "Alt"),
            Modifier::Shift => write!(f, "Shift"),
            Modifier::Super => write!(f, "Super"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct ModifierList(Vec<Modifier>);

impl ModifierList {
    pub fn has_ctrl(&self) -> bool {
        self.0.contains(&Modifier::Ctrl)
    }

    pub fn has_alt(&self) -> bool {
        self.0.contains(&Modifier::Alt)
    }

    pub fn has_shift(&self) -> bool {
        self.0.contains(&Modifier::Shift)
    }

    pub fn has_super(&self) -> bool {
        self.0.contains(&Modifier::Super)
    }
}

impl From<Modifier> for ModifierList {
    fn from(m: Modifier) -> Self {
        Self(vec![m])
    }
}

impl PartialEq<Modifier> for ModifierList {
    fn eq(&self, other: &Modifier) -> bool {
        *self == ModifierList(vec![*other])
    }
}

impl From<Vec<Modifier>> for ModifierList {
    fn from(mods: Vec<Modifier>) -> Self {
        Self(mods)
    }
}

impl PartialEq<Vec<Modifier>> for ModifierList {
    fn eq(&self, other: &Vec<Modifier>) -> bool {
        self.0 == *other
    }
}

impl<const N: usize> From<[Modifier; N]> for ModifierList {
    fn from(mods: [Modifier; N]) -> Self {
        Self(mods.to_vec())
    }
}

impl<const N: usize> PartialEq<[Modifier; N]> for ModifierList {
    fn eq(&self, other: &[Modifier; N]) -> bool {
        self.0 == other.to_vec()
    }
}

impl Display for ModifierList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|m| format!("{} + ", m))
                .collect::<String>(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eq() {
        let mods = ModifierList(vec![Modifier::Ctrl]);
        assert!(mods == vec![Modifier::Ctrl]);
        assert!(mods == [Modifier::Ctrl]);
        assert!(mods == Modifier::Ctrl);

        let mods2 = ModifierList(vec![Modifier::Ctrl, Modifier::Alt]);
        assert!(mods2 == vec![Modifier::Ctrl, Modifier::Alt]);
        assert!(mods2 != [Modifier::Ctrl, Modifier::Super]);
        assert!(mods2 != Modifier::Ctrl);
    }
}
