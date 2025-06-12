use super::{Expr, Visitor};

#[derive(Debug)]
pub struct Return<'input> {
    pub expr: Expr<'input>,
}

impl<'input> From<Visitor<'input>> for Return<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        Return {
            expr: visitor.expect_node::<Expr>(),
        }
    }
}
