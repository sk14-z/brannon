use std::{fmt::Display, ops};

// Unless kitty protocol is active:
// - Shift will not be sent by itself, or only with Control (captured by terminal).
// Including a modifier with only Shift is redundant, as it is implied by the Key variant.
// The only exception is Shift + Tab, which is sent as a distinct escape sequence.
// - Hyper and Meta are special to X11/Wayland and to be honest I've never heard of them but knock
// yourself out
// - Not all combinations with control are captured. This is unavoidable.
// For example, Ctrl + H, I, or J return backspace, tab, enter respectively.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Modifier {
    Shift = 0b000001,
    Alt = 0b000010,
    Ctrl = 0b000100,
    Super = 0b001000,
    Hyper = 0b010000,
    Meta = 0b100000,
}

impl ops::BitAnd for Modifier {
    type Output = usize;

    fn bitand(self, rhs: Self) -> Self::Output {
        (self as usize) & (rhs as usize)
    }
}

impl ops::BitOr for Modifier {
    type Output = usize;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as usize) | (rhs as usize)
    }
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Modifier::Shift => write!(f, "Shift"),
            Modifier::Alt => write!(f, "Alt"),
            Modifier::Ctrl => write!(f, "Ctrl"),
            Modifier::Super => write!(f, "Super"),
            Modifier::Hyper => write!(f, "Hyper"),
            Modifier::Meta => write!(f, "Meta"),
        }
    }
}

#[derive(Clone, Default)]
pub struct ModifierList(pub(crate) Vec<Modifier>);

impl ModifierList {
    pub(crate) fn mask(&self) -> usize {
        let mut n: usize = 0;

        for m in self.clone().0 {
            n |= m as usize;
        }

        1 + n
    }

    pub(crate) fn unmask(mask: usize) -> ModifierList {
        let mask = mask.saturating_sub(1);
        let mut mods: ModifierList = [].into();

        if mask & (Modifier::Shift as usize) != 0 {
            mods += Modifier::Shift;
        }

        if mask & (Modifier::Alt as usize) != 0 {
            mods += Modifier::Alt;
        }

        if mask & (Modifier::Ctrl as usize) != 0 {
            mods += Modifier::Ctrl;
        }

        if mask & (Modifier::Super as usize) != 0 {
            mods += Modifier::Super;
        }

        if mask & (Modifier::Hyper as usize) != 0 {
            mods += Modifier::Hyper;
        }

        if mask & (Modifier::Meta as usize) != 0 {
            mods += Modifier::Meta;
        }

        mods
    }
}

impl ops::Add<Modifier> for ModifierList {
    type Output = Self;

    fn add(mut self, rhs: Modifier) -> Self::Output {
        if !self.0.contains(&rhs) {
            self.0.push(rhs);
        }
        self
    }
}

impl ops::AddAssign<Modifier> for ModifierList {
    fn add_assign(&mut self, rhs: Modifier) {
        if !self.0.contains(&rhs) {
            self.0.push(rhs);
        }
    }
}

impl PartialEq for ModifierList {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        for m in &other.0 {
            if !self.0.contains(m) {
                return false;
            }
        }

        true
    }
}

impl From<usize> for ModifierList {
    fn from(n: usize) -> Self {
        Self::unmask(n)
    }
}

impl PartialEq<usize> for ModifierList {
    fn eq(&self, other: &usize) -> bool {
        self.mask() == *other
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
        if self.0.len() != other.len() {
            return false;
        }

        for m in other {
            if !self.0.contains(m) {
                return false;
            }
        }
        true
    }
}

impl<const N: usize> From<[Modifier; N]> for ModifierList {
    fn from(mods: [Modifier; N]) -> Self {
        Self(mods.to_vec())
    }
}

impl<const N: usize> PartialEq<[Modifier; N]> for ModifierList {
    fn eq(&self, other: &[Modifier; N]) -> bool {
        if self.0.len() != other.len() {
            return false;
        }

        for m in other {
            if !self.0.contains(m) {
                return false;
            }
        }
        true
    }
}

impl Display for ModifierList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|m| format!("{}", m))
                .collect::<Vec<String>>()
                .join(" + "),
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

        let mods2 = ModifierList(vec![Modifier::Alt, Modifier::Ctrl]);
        assert!(mods2 == vec![Modifier::Ctrl, Modifier::Alt]);
        assert!(mods2 != [Modifier::Ctrl, Modifier::Super]);
        assert!(mods2 != Modifier::Ctrl);

        let mods3 = ModifierList(vec![Modifier::Ctrl, Modifier::Alt]);
        assert!(mods2 == mods3);
    }

    #[test]
    fn op() {
        assert_eq!(Modifier::Shift as usize, 0b0001);
        assert_eq!(Modifier::Alt & Modifier::Ctrl, 0b0000);
        assert_eq!(Modifier::Shift | Modifier::Ctrl, 0b0101);
        assert_ne!(Modifier::Alt | Modifier::Ctrl, 0b0101);
    }

    #[test]
    fn mask() {
        assert!(ModifierList(vec![]).mask() == 0b1);
        assert!(ModifierList(vec![Modifier::Shift]).mask() == 0b10);
        assert!(ModifierList(vec![Modifier::Alt]).mask() == 0b11);
        assert!(ModifierList(vec![Modifier::Shift, Modifier::Alt]).mask() == 0b100);
        assert!(ModifierList(vec![Modifier::Ctrl]).mask() == 0b101);
        assert!(ModifierList(vec![Modifier::Shift, Modifier::Ctrl]).mask() == 0b110);
        assert!(ModifierList(vec![Modifier::Ctrl, Modifier::Alt]).mask() == 0b111);
        assert!(
            ModifierList(vec![Modifier::Ctrl, Modifier::Shift, Modifier::Alt]).mask() == 0b1000
        );
        assert!(ModifierList(vec![Modifier::Super]).mask() == 0b1001);
    }

    #[test]
    fn unmask() {
        assert!(ModifierList::unmask(0b1) == []);
        assert!(ModifierList::unmask(0b10) == [Modifier::Shift]);
        assert!(ModifierList::unmask(0b11) == [Modifier::Alt]);
        assert!(ModifierList::unmask(0b100) == [Modifier::Shift, Modifier::Alt]);
        assert!(ModifierList::unmask(0b101) == [Modifier::Ctrl]);
        assert!(ModifierList::unmask(0b110) == [Modifier::Shift, Modifier::Ctrl]);
        assert!(ModifierList::unmask(0b111) == [Modifier::Ctrl, Modifier::Alt]);
        assert!(ModifierList::unmask(0b1000) == [Modifier::Ctrl, Modifier::Shift, Modifier::Alt]);
        assert!(ModifierList::unmask(0b1001) == [Modifier::Super]);
    }
}
