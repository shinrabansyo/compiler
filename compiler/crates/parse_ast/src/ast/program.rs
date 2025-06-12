use super::{Top, Visitor};

#[derive(Debug)]
pub struct Program {
    pub top_elems: Vec<Top>,
}

impl From<Visitor<'_>> for Program {
    fn from(mut visitor: Visitor<'_>) -> Self {
        Program {
            top_elems: visitor.expect_nodes::<Top>(),
        }
    }
}
