use std::fmt::{Debug, Display};

pub struct Out {
    pub id_reg: u8,
    pub src_reg: u8,
}

impl Debug for Out {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, r{}", self.id_reg, self.src_reg)
    }
}

impl Display for Out {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "out r{}[0] = r{}", self.id_reg, self.src_reg)
    }
}

impl Out {
    pub fn new(id_reg: u8, src_reg: u8) -> Self {
        Out { id_reg, src_reg }
    }
}
