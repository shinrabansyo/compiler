use std::fmt::Debug;

pub struct Jmp {
    pub label: String,
}

impl Debug for Jmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl Jmp {
    pub fn new(label: String) -> Self {
        Jmp { label }
    }
}
