use super::{Stmt, Visitor};

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl From<(String, Visitor<'_>)> for Block {
    fn from((_, mut visitor): (String, Visitor<'_>)) -> Self {
        Block {
            stmts: visitor.expect_nodes::<Stmt>(),
        }
    }
}
