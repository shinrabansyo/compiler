use super::{Top, Visitor};

#[derive(Debug)]
pub struct Program<'src> {
    pub top_elems: Vec<Top<'src>>,
}

impl<'src> From<Visitor<'src>> for Program<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        Program {
            top_elems: visitor.expect_nodes::<Top>(),
        }
    }
}
