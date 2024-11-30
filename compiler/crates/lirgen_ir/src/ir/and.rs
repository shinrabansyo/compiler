use std::fmt::{Debug, Display};

pub struct And {
    pub lhs_reg: u8,
    pub rhs_reg: u8,
}

impl Debug for And {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}", self.lhs_reg, self.rhs_reg)
    }
}

impl Display for And {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "and r{} = r{}, r{}", self.lhs_reg, self.lhs_reg, self.rhs_reg)
    }
}

impl And {
    pub fn new(lhs_reg: u8, rhs_reg: u8) -> Self {
        And { lhs_reg, rhs_reg }
    }
}
