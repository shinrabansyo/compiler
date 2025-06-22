use super::{Block, Expr, VarDecl, Visitor};

#[derive(Debug)]
pub struct For<'src> {
    pub init: VarDecl<'src>,
    pub cond: Expr<'src>,
    pub incr: Expr<'src>,
    pub block: Block<'src>,
}

impl<'src> From<Visitor<'src>> for For<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        For {
            init: visitor.expect_node::<VarDecl>(),
            cond: visitor.expect_node::<Expr>(),
            incr: visitor.expect_node::<Expr>(),
            block: visitor.expect_node::<Block>(),
        }
    }
}
