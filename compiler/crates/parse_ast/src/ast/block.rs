use super::{Stmt, Visitor};

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl From<Visitor<'_>> for Block {
    fn from(mut visitor: Visitor<'_>) -> Self {
        Block {
            stmts: visitor.expect_nodes::<Stmt>(),
        }
    }
}
