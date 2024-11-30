use std::fmt::{Debug, Display};

pub struct Beq {
    pub lhs_reg: u8,
    pub rhs_reg: u8,
    pub diff: i32,
}

impl Debug for Beq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}, {}", self.lhs_reg, self.rhs_reg, self.diff)
    }
}

impl Display for Beq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "beq r0, (r{}, r{}) -> {}", self.lhs_reg, self.rhs_reg, self.diff)
    }
}

impl Beq {
    pub fn new(lhs_reg: u8, rhs_reg: u8, diff: i32) -> Self {
        Beq { lhs_reg, rhs_reg, diff }
    }
}
