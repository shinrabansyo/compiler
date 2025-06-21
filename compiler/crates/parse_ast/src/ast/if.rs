use super::{Block, Stmt, Expr, Visitor};

#[derive(Debug)]
pub struct If<'src> {
    pub cond: Expr<'src>,
    pub block: Block<'src>,
    pub else_stmt: Option<Box<Stmt<'src>>>,
}

impl<'src> From<Visitor<'src>> for If<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
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
