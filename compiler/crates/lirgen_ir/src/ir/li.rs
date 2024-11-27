use std::fmt::Debug;

pub struct Li {
    pub reg: u8,
    pub value: i32,
}

impl Debug for Li {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}, {}", self.reg, self.value)
    }
}

impl Li {
    pub fn new(reg: u8, value: i32) -> Self {
        Li { reg, value }
    }
}
