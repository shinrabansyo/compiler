use std::fmt::Debug;

pub struct FLoad;

impl Debug for FLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl FLoad {
    pub fn new() -> Self {
        FLoad
    }
}
