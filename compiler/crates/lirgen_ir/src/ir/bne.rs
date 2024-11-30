use std::fmt::{Debug, Display};

pub struct Bne {
    pub lhs_reg: u8,
    pub rhs_reg: u8,
    pub diff: u32,
}

impl Debug for Bne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}, {}", self.lhs_reg, self.rhs_reg, self.diff)
    }
}

impl Display for Bne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bne r0, (r{}, r{}) -> {}", self.lhs_reg, self.rhs_reg, self.diff)
    }
}

impl Bne {
    pub fn new(lhs_reg: u8, rhs_reg: u8, diff: u32) -> Self {
        Bne { lhs_reg, rhs_reg, diff }
    }
}
