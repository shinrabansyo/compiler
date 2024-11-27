use std::fmt::Debug;

pub struct Push {
    pub reg: u8,
}

impl Debug for Push {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}", self.reg)
    }
}

impl Push {
    pub fn new(reg: u8) -> Self {
        Push { reg }
    }
}
