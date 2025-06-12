use super::{Expr, Visitor};

#[derive(Debug)]
pub struct VarDecl {
    pub ident: String,
    pub ty: String,
    pub expr: Expr,
}

impl From<Visitor<'_>> for VarDecl  {
    fn from(mut visitor: Visitor<'_>) -> Self {
        let ident = visitor.expect_leaf().1.to_string();
        let ty = visitor.expect_leaf().1.to_string();
        let _ = visitor.expect_leaf();  // '='
        let expr = visitor.expect_node::<Expr>();

        VarDecl { ident, ty, expr }
    }
}
