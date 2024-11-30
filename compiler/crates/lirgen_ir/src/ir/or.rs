use std::fmt::{Debug, Display};

pub struct Or {
    pub lhs_reg: u8,
    pub rhs_reg: u8,
}

impl Debug for Or {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}", self.lhs_reg, self.rhs_reg)
    }
}

impl Display for Or {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "or r{} = r{}, r{}", self.lhs_reg, self.lhs_reg, self.rhs_reg)
    }
}

impl Or {
    pub fn new(lhs_reg: u8, rhs_reg: u8) -> Self {
        Or { lhs_reg, rhs_reg }
    }
}
