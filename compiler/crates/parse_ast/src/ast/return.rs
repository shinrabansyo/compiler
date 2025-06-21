use super::{Expr, Visitor};

#[derive(Debug)]
pub struct Return<'src> {
    pub expr: Expr<'src>,
}

impl<'src> From<Visitor<'src>> for Return<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        Return {
            expr: visitor.expect_node::<Expr>(),
        }
    }
}
