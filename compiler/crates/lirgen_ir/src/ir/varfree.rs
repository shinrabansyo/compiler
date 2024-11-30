use std::fmt::{Debug, Display};

pub struct VarFree;

impl Debug for VarFree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Display for VarFree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "lw r4 = r2[0]")?;
        writeln!(f, "add r2 = r2, r4")?;
        write!(f, "addi r2 = r2, 4")
    }
}

impl VarFree {
    pub fn new() -> Self {
        VarFree
    }
}
