use std::fmt::{Debug, Display};

pub struct ShiftR {
    pub lhs_reg: u8,
    pub rhs_reg: u8,
}

impl Debug for ShiftR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}", self.lhs_reg, self.rhs_reg)
    }
}

impl Display for ShiftR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "srl r{} = r{}, r{}", self.lhs_reg, self.lhs_reg, self.rhs_reg)
    }
}

impl ShiftR {
    pub fn new(lhs_reg: u8, rhs_reg: u8) -> Self {
        ShiftR { lhs_reg, rhs_reg }
    }
}
