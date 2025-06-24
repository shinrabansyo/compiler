use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBToken;

use super::{Expr, Visitor};

#[derive(Debug)]
pub struct VarDecl<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub ty: Option<Span<'src>>,
    pub expr: Expr<'src>,
}

impl<'src> From<Visitor<'src>> for VarDecl<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        let span = visitor.span();
        let ident = visitor.expect_leaf().1;
        let ty = visitor
            .peek()
            .0
            .filter(|token| token != &SBToken::Assign)
            .and_then(|_| Some(visitor.expect_leaf().1));
        let _ = visitor.expect_leaf();  // '='
        let expr = visitor.expect_node::<Expr>();

        VarDecl { span, ident, ty, expr }
    }
}

impl<'src> Spanned<'src> for VarDecl<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
