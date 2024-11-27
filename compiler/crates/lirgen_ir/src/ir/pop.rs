use std::fmt::Debug;

pub struct Pop {
    pub reg: u8,
}

impl Debug for Pop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}", self.reg)
    }
}

impl Pop {
    pub fn new(reg: u8) -> Self {
        Pop { reg }
    }
}
