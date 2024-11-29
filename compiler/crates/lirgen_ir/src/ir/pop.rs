use std::fmt::{Debug, Display};

pub struct Pop {
    pub reg: u8,
}

impl Debug for Pop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}", self.reg)
    }
}

impl Display for Pop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "lw r{} = r2[0]", self.reg)?;
        write!(f, "addi r2 = r2, 4")
    }
}

impl Pop {
    pub fn new(reg: u8) -> Self {
        Pop { reg }
    }
}
