use super::{Stmt, Visitor};

#[derive(Debug)]
pub struct Block<'src> {
    pub stmts: Vec<Stmt<'src>>,
}

impl<'src> From<Visitor<'src>> for Block<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        Block {
            stmts: visitor.expect_nodes::<Stmt>(),
        }
    }
}
