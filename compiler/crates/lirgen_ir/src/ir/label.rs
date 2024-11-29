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
        write!(f, "@{}", self.name)
    }
}

impl Label {
    pub fn new(name: String) -> Self {
        Label { name }
    }
}
