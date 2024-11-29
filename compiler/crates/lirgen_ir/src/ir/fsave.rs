use std::fmt::Debug;

pub struct FSave;

impl Debug for FSave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl FSave {
    pub fn new() -> Self {
        FSave
    }
}
