use std::fmt::{Debug, Display};

pub struct Label {
    pub name: String,
}

impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "@{}", self.name)?;
        write!(f, "add r0 = r0, r0")
    }
}

impl Label {
    pub fn new(name: String) -> Self {
        Label { name }
    }
}
