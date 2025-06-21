use super::{Assign, Visitor};

#[derive(Debug)]
pub struct Expr<'src> {
    pub assign: Assign<'src>,
}

impl<'src> From<Visitor<'src>> for Expr<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        Expr {
            assign: visitor.expect_node::<Assign>(),
        }
    }
}
