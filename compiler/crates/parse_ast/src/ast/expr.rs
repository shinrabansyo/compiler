use super::{Assign, Visitor};

#[derive(Debug)]
pub struct Expr {
    pub assign: Assign,
}

impl From<(String, Visitor<'_>)> for Expr {
    fn from((_, mut visitor): (String, Visitor<'_>)) -> Self {
        Expr {
            assign: visitor.expect_node::<Assign>(),
        }
    }
}
