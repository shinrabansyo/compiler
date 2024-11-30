use std::fmt::{Debug, Display};

pub struct Ble {
    pub lhs_reg: u8,
    pub rhs_reg: u8,
    pub diff: u32,
}

impl Debug for Ble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}, {}", self.lhs_reg, self.rhs_reg, self.diff)
    }
}

impl Display for Ble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ble r0, (r{}, r{}) -> {}", self.lhs_reg, self.rhs_reg, self.diff)
    }
}

impl Ble {
    pub fn new(lhs_reg: u8, rhs_reg: u8, diff: u32) -> Self {
        Ble { lhs_reg, rhs_reg, diff }
    }
}
