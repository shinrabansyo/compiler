use sb_compiler_parse_cst::Span;

use super::{Expr, Visitor};

#[derive(Debug)]
pub struct VarDecl<'src> {
    pub ident: Span<'src>,
    pub ty: Span<'src>,
    pub expr: Expr<'src>,
}

impl<'src> From<Visitor<'src>> for VarDecl<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        let ident = visitor.expect_leaf().1;
        let ty = visitor.expect_leaf().1;
        let _ = visitor.expect_leaf();  // '='
        let expr = visitor.expect_node::<Expr>();

        VarDecl { ident, ty, expr }
    }
}
