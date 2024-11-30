use std::fmt::{Debug, Display};

pub struct ShiftRa {
    pub lhs_reg: u8,
    pub rhs_reg: u8,
}

impl Debug for ShiftRa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}", self.lhs_reg, self.rhs_reg)
    }
}

impl Display for ShiftRa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sra r{} = r{}, r{}", self.lhs_reg, self.lhs_reg, self.rhs_reg)
    }
}

impl ShiftRa {
    pub fn new(lhs_reg: u8, rhs_reg: u8) -> Self {
        ShiftRa { lhs_reg, rhs_reg }
    }
}
