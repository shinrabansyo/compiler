use std::fmt::{Debug, Display};

pub struct Xor {
    pub lhs_reg: u8,
    pub rhs_reg: u8,
}

impl Debug for Xor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}", self.lhs_reg, self.rhs_reg)
    }
}

impl Display for Xor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "xor r{} = r{}, r{}", self.lhs_reg, self.lhs_reg, self.rhs_reg)
    }
}

impl Xor {
    pub fn new(lhs_reg: u8, rhs_reg: u8) -> Self {
        Xor { lhs_reg, rhs_reg }
    }
}
