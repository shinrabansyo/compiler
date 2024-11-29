use std::fmt::{Debug, Display};

pub struct VarFree {
    pub bytes: u32,
}

impl Debug for VarFree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bytes", self.bytes)
    }
}

impl Display for VarFree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "addi r2 = r2, {}", self.bytes)
    }
}

impl VarFree {
    pub fn new(bytes: u32) -> Self {
        VarFree { bytes }
    }
}
