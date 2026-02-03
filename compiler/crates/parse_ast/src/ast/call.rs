use sb_compiler_parse_cst::{Span, Spanned};

use super::{Expr, Visitor};

#[derive(Debug)]
pub struct Call<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub args: Vec<Expr<'src>>,
}

impl<'src> From<Visitor<'src>> for Call<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
         Call {
            span: visitor.span(),
            ident: visitor.expect_leaf().1,
            args: visitor.expect_nodes::<Expr>(),
        }
    }
}

impl<'src> Spanned<'src> for Call<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
