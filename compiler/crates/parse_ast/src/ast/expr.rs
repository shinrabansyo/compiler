use super::{Assign, Visitor};

#[derive(Debug)]
pub struct Expr<'input> {
    pub assign: Assign<'input>,
}

impl<'input> From<Visitor<'input>> for Expr<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        Expr {
            assign: visitor.expect_node::<Assign>(),
        }
    }
}
