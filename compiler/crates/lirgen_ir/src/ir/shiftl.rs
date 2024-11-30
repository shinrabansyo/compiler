use std::fmt::{Debug, Display};

pub struct ShiftL {
    pub lhs_reg: u8,
    pub rhs_reg: u8,
}

impl Debug for ShiftL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}", self.lhs_reg, self.rhs_reg)
    }
}

impl Display for ShiftL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sll r{} = r{}, r{}", self.lhs_reg, self.lhs_reg, self.rhs_reg)
    }
}

impl ShiftL {
    pub fn new(lhs_reg: u8, rhs_reg: u8) -> Self {
        ShiftL { lhs_reg, rhs_reg }
    }
}
