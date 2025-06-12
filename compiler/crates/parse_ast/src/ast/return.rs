use super::{Expr, Visitor};

#[derive(Debug)]
pub struct Return {
    pub expr: Expr,
}

impl From<Visitor<'_>> for Return {
    fn from(mut visitor: Visitor<'_>) -> Self {
        Return {
            expr: visitor.expect_node::<Expr>(),
        }
    }
}
