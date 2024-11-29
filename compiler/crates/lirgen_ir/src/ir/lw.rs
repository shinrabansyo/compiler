use std::fmt::{Debug, Display};

pub struct Lw {
    pub dst_reg: u8,
    pub base_reg: u8,
    pub addr: u32,
}

impl Debug for Lw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{} r{}[{:<08X}]", self.dst_reg, self.base_reg, self.addr)
    }
}

impl Display for Lw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lw r{} = r{}[{}]", self.dst_reg, self.base_reg, self.addr)
    }
}

impl Lw {
    pub fn new(dst_reg: u8, base_reg: u8, addr: u32) -> Self {
        Lw { dst_reg, base_reg, addr }
    }
}
