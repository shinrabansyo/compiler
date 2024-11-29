use std::fmt::{Debug, Display};

pub struct VarAlloc {
    pub bytes: u32,
}

impl Debug for VarAlloc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bytes", self.bytes)
    }
}

impl Display for VarAlloc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "subi r2 = r2, {}", self.bytes)?;
        writeln!(f, "add r6 = r0, r2")?;
        writeln!(f, "subi r2 = r2, 4")?;
        writeln!(f, "addi r4 = r0, {}",self.bytes)?;
        write!(f, "sw r2[0] = r4")
    }
}

impl VarAlloc {
    pub fn new(bytes: u32) -> Self {
        VarAlloc { bytes }
    }
}
