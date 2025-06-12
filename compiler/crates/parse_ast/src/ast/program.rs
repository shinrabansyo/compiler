use super::{Top, Visitor};

#[derive(Debug)]
pub struct Program {
    pub top_elems: Vec<Top>,
}

impl From<(String, Visitor<'_>)> for Program {
    fn from((_, mut visitor): (String, Visitor<'_>)) -> Self {
        Program {
            top_elems: visitor.expect_nodes::<Top>(),
        }
    }
}
