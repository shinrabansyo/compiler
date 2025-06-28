use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBRule;

use super::{Expr, Type, Visitor};

#[derive(Debug)]
pub struct VarDecl<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub ty: Option<Type<'src>>,
    pub expr: Expr<'src>,
}

impl<'src> From<Visitor<'src>> for VarDecl<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        let span = visitor.span();
        let ident = visitor.expect_leaf().1;
        let ty = visitor
            .peek()
            .1
            .filter(|rule| rule == &SBRule::Type)
            .and_then(|_| Some(visitor.expect_node::<Type>()));
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
