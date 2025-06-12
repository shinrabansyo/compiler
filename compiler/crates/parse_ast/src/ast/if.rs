use super::{Block, Stmt, Expr, Visitor};

#[derive(Debug)]
pub struct If {
    pub cond: Expr,
    pub block: Block,
    pub else_stmt: Option<Box<Stmt>>,
}

impl From<Visitor<'_>> for If  {
    fn from(mut visitor: Visitor<'_>) -> Self {
        let cond = visitor.expect_node::<Expr>();
        let block = visitor.expect_node::<Block>();
        let else_stmt = visitor
            .peek()
            .1
            .and_then(|_| {
                Some(Box::new(visitor.expect_node::<Stmt>()))
            });

        If { cond, block, else_stmt }
    }
}
