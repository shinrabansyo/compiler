use super::{Block, Stmt, Expr, Visitor};

#[derive(Debug)]
pub struct If<'input> {
    pub cond: Expr<'input>,
    pub block: Block<'input>,
    pub else_stmt: Option<Box<Stmt<'input>>>,
}

impl<'input> From<Visitor<'input>> for If<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
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
