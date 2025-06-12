use super::{Block, Expr, Visitor};

#[derive(Debug)]
pub struct For {
    pub init: Expr,
    pub cond: Expr,
    pub incr: Expr,
    pub block: Block,
}

impl From<Visitor<'_>> for For  {
    fn from(mut visitor: Visitor<'_>) -> Self {
        For {
            init: visitor.expect_node::<Expr>(),
            cond: visitor.expect_node::<Expr>(),
            incr: visitor.expect_node::<Expr>(),
            block: visitor.expect_node::<Block>(),
        }
    }
}
