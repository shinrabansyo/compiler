use std::fmt::{Debug, Display};

pub struct In {
    pub dst_reg: u8,
    pub id_reg: u8,
}

impl Debug for In {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}", self.dst_reg, self.id_reg)
    }
}

impl Display for In {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "in r{} = r{}[0]", self.dst_reg, self.id_reg)
    }
}

impl In {
    pub fn new(dst_reg: u8, id_reg: u8) -> Self {
        In { dst_reg, id_reg }
    }
}
