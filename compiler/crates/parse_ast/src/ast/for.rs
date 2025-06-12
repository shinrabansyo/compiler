use super::{Block, Expr, Visitor};

#[derive(Debug)]
pub struct For<'input> {
    pub init: Expr<'input>,
    pub cond: Expr<'input>,
    pub incr: Expr<'input>,
    pub block: Block<'input>,
}

impl<'input> From<Visitor<'input>> for For<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        For {
            init: visitor.expect_node::<Expr>(),
            cond: visitor.expect_node::<Expr>(),
            incr: visitor.expect_node::<Expr>(),
            block: visitor.expect_node::<Block>(),
        }
    }
}
