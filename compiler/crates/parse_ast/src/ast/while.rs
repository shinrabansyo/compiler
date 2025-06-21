use super::{Block, Expr, Visitor};

#[derive(Debug)]
pub struct While<'src> {
    pub cond: Expr<'src>,
    pub block: Block<'src>,
}

impl<'src> From<Visitor<'src>> for While<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        While {
            cond: visitor.expect_node::<Expr>(),
            block: visitor.expect_node::<Block>(),
        }
    }
}
