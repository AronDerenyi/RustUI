use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr, BitXor},
};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Change(u8);

impl Change {
    pub const NONE: Self = Self(0);
    pub const VIEW: Self = Self(0b0001);
    pub const STATE: Self = Self(0b0010);
    pub const SIZE: Self = Self(0b0100);
    pub const ALL: Self = Self(u8::MAX);

    pub fn add(&mut self, change: Self) {
        self.0 |= change.0;
    }

    pub fn contains(&self, change: Self) -> bool {
        self.0 & change.0 != 0
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

impl BitOr for Change {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Change(self.0 | rhs.0)
    }
}

impl BitAnd for Change {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Change(self.0 & rhs.0)
    }
}

impl BitXor for Change {
    type Output = Change;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Change(self.0 ^ rhs.0)
    }
}

impl Debug for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let changes = [Self::VIEW, Self::STATE, Self::SIZE]
            .into_iter()
            .filter(|change| self.contains(*change))
            .map::<String, _>(|change| match change {
                Self::VIEW => "view".into(),
                Self::STATE => "state".into(),
                Self::SIZE => "size".into(),
                _ => "unknown".into(),
            })
            .join(", ");

        f.write_str("[")?;
        f.write_str(&changes)?;
        f.write_str("]")
    }
}
