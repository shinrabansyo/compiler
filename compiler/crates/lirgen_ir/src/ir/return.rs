use std::fmt::Debug;

pub struct Return;

impl Debug for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Return {
    pub fn new() -> Self {
        Return
    }
}
