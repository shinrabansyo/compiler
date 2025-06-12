use super::{Expr, Visitor};

#[derive(Debug)]
pub struct VarDecl<'input> {
    pub ident: &'input str,
    pub ty: &'input str,
    pub expr: Expr<'input>,
}

impl<'input> From<Visitor<'input>> for VarDecl<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        let ident = visitor.expect_leaf().1;
        let ty = visitor.expect_leaf().1;
        let _ = visitor.expect_leaf();  // '='
        let expr = visitor.expect_node::<Expr>();

        VarDecl { ident, ty, expr }
    }
}
