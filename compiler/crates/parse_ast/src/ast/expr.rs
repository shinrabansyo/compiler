use super::{Assign, Visitor};

#[derive(Debug)]
pub struct Expr {
    pub assign: Assign,
}

impl From<Visitor<'_>> for Expr {
    fn from(mut visitor: Visitor<'_>) -> Self {
        Expr {
            assign: visitor.expect_node::<Assign>(),
        }
    }
}
