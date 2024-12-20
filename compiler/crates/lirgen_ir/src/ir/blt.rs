use std::fmt::{Debug, Display};

pub struct Blt {
    pub lhs_reg: u8,
    pub rhs_reg: u8,
    pub diff: i32,
}

impl Debug for Blt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}, {}", self.lhs_reg, self.rhs_reg, self.diff)
    }
}

impl Display for Blt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "blt r0, (r{}, r{}) -> {}", self.lhs_reg, self.rhs_reg, self.diff)
    }
}

impl Blt {
    pub fn new(lhs_reg: u8, rhs_reg: u8, diff: i32) -> Self {
        Blt { lhs_reg, rhs_reg, diff }
    }
}
