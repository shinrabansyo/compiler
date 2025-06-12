use super::{Top, Visitor};

#[derive(Debug)]
pub struct Program<'input> {
    pub top_elems: Vec<Top<'input>>,
}

impl<'input> From<Visitor<'input>> for Program<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        Program {
            top_elems: visitor.expect_nodes::<Top>(),
        }
    }
}
