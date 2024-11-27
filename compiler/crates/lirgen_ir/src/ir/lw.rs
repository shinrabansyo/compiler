use std::fmt::Debug;

pub struct Lw {
    pub addr: u32,
    pub base_reg: u8,
    pub dst_reg: u8,
}

impl Debug for Lw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{} r{}[{:<08X}]", self.dst_reg, self.base_reg, self.addr)
    }
}

impl Lw {
    pub fn new(addr: u32, base_reg: u8, dst_reg: u8) -> Self {
        Lw { addr, base_reg, dst_reg }
    }
}
