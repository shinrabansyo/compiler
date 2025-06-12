use super::{Block, Expr, Visitor};

#[derive(Debug)]
pub struct For {
    pub namespace: String,
    pub init: Expr,
    pub cond: Expr,
    pub incr: Expr,
    pub block: Block,
}

impl From<(String, Visitor<'_>)> for For  {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        For {
            namespace: namespace.clone(),
            init: visitor.expect_node::<Expr>(),
            cond: visitor.expect_node::<Expr>(),
            incr: visitor.expect_node::<Expr>(),
            block: visitor.expect_node::<Block>(),
        }
    }
}
