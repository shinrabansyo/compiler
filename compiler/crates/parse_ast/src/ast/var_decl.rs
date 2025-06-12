use sb_compiler_parse_cst::Span;

use super::{Expr, Visitor};

#[derive(Debug)]
pub struct VarDecl<'input> {
    pub ident: Span<'input>,
    pub ty: Span<'input>,
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
