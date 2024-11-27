use std::fmt::Debug;

pub struct Sw {
    pub base_reg: u8,
    pub addr: u32,
    pub src_reg: u8,
}

impl Debug for Sw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}[{:<08X}], r{}", self.base_reg, self.addr, self.src_reg)
    }
}

impl Sw {
    pub fn new(base_reg: u8, addr: u32, src_reg: u8) -> Self {
        Sw { base_reg, addr, src_reg }
    }
}
