use std::fmt::{Debug, Display};

pub struct Return;

impl Debug for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "jal r0, r1[0]")
    }
}

impl Return {
    pub fn new() -> Self {
        Return
    }
}
