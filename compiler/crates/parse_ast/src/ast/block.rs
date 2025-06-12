use super::{Stmt, Visitor};

#[derive(Debug)]
pub struct Block<'input> {
    pub stmts: Vec<Stmt<'input>>,
}

impl<'input> From<Visitor<'input>> for Block<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        Block {
            stmts: visitor.expect_nodes::<Stmt>(),
        }
    }
}
