use std::fmt::{Debug, Display};

pub struct Push {
    pub reg: u8,
}

impl Debug for Push {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}", self.reg)
    }
}

impl Display for Push {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "subi r2 = r2, 4")?;
        write!(f, "sw r2[0] = r{}", self.reg)
    }
}

impl Push {
    pub fn new(reg: u8) -> Self {
        Push { reg }
    }
}
