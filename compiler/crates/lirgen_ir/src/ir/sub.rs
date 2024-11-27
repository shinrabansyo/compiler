use std::fmt::Debug;

pub struct Sub {
    pub lhs_reg: u8,
    pub rhs_reg: u8,
}

impl Debug for Sub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}", self.lhs_reg, self.rhs_reg)
    }
}

impl Sub {
    pub fn new(lhs_reg: u8, rhs_reg: u8) -> Self {
        Sub { lhs_reg, rhs_reg }
    }
}
