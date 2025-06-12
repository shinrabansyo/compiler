use super::{Block, Expr, Visitor};

#[derive(Debug)]
pub struct While<'input> {
    pub cond: Expr<'input>,
    pub block: Block<'input>,
}

impl<'input> From<Visitor<'input>> for While<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        While {
            cond: visitor.expect_node::<Expr>(),
            block: visitor.expect_node::<Block>(),
        }
    }
}
