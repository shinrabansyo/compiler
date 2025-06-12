use super::{Block, Expr, Visitor};

#[derive(Debug)]
pub struct While {
    pub cond: Expr,
    pub block: Block,
}

impl From<Visitor<'_>> for While  {
    fn from(mut visitor: Visitor<'_>) -> Self {
        While {
            cond: visitor.expect_node::<Expr>(),
            block: visitor.expect_node::<Block>(),
        }
    }
}
