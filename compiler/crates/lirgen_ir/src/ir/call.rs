use std::fmt::{Debug, Display};

pub struct Call {
    pub label: String,
}

impl Debug for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "beq r1, (r0, r0) -> @{}", self.label)
    }
}

impl Call {
    pub fn new(label: String) -> Self {
        Call { label }
    }
}
