use std::fmt::{Debug, Display};

pub struct Add {
    pub lhs_reg: u8,
    pub rhs_reg: u8,
}

impl Debug for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}", self.lhs_reg, self.rhs_reg)
    }
}

impl Display for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "add r{} = r{}, r{}", self.lhs_reg, self.lhs_reg, self.rhs_reg)
    }
}

impl Add {
    pub fn new(lhs_reg: u8, rhs_reg: u8) -> Self {
        Add { lhs_reg, rhs_reg }
    }
}
