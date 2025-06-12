use super::{Block, Expr, Visitor};

#[derive(Debug)]
pub struct While {
    pub namespace: String,
    pub cond: Expr,
    pub block: Block,
}

impl From<(String, Visitor<'_>)> for While  {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        While {
            namespace,
            cond: visitor.expect_node::<Expr>(),
            block: visitor.expect_node::<Block>(),
        }
    }
}
