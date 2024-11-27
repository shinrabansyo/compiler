use std::fmt::Debug;

pub struct Sw {
    pub addr: u32,
    pub base_reg: u8,
    pub src_reg: u8,
}

impl Debug for Sw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}[{:<08X}], {}", self.base_reg, self.addr, self.src_reg)
    }
}

impl Sw {
    pub fn new(addr: u32, base_reg: u8, src_reg: u8) -> Self {
        Sw { addr, base_reg, src_reg }
    }
}
