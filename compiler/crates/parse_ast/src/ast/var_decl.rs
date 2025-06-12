use super::{Expr, Visitor};

#[derive(Debug)]
pub struct VarDecl {
    pub namespace: String,
    pub ident: String,
    pub ty: String,
    pub expr: Expr,
}

impl From<(String, Visitor<'_>)> for VarDecl  {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        let ident = visitor.expect_leaf().1.to_string();
        let ty = visitor.expect_leaf().1.to_string();
        let _ = visitor.expect_leaf();  // '='
        let expr = visitor.expect_node::<Expr>();

        VarDecl { namespace, ident, ty, expr }
    }
}
