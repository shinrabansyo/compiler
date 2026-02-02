use sb_compiler_parse_cst::{Span, Spanned};

use super::{Expr, Visitor};

#[derive(Debug)]
pub struct StructFieldInit<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub expr: Expr<'src>,
}

impl<'src> From<Visitor<'src>> for StructFieldInit<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        StructFieldInit {
            span: visitor.span(),
            ident: visitor.expect_leaf().1,
            expr: visitor.expect_node::<Expr>(),
        }
    }
}

impl<'src> Spanned<'src> for StructFieldInit<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
