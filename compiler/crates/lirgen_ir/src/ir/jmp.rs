use std::fmt::{Debug, Display};

pub struct Jmp {
    pub label: String,
}

impl Debug for Jmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl Display for Jmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "beq r0, (r0, r0) -> @{}", self.label)
    }
}

impl Jmp {
    pub fn new(label: String) -> Self {
        Jmp { label }
    }
}
