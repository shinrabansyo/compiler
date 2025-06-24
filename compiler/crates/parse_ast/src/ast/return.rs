use sb_compiler_parse_cst::Span;

use super::{Expr, Visitor};

#[derive(Debug)]
pub struct Return<'src> {
    pub span: Span<'src>,
    pub expr: Expr<'src>,
}

impl<'src> From<Visitor<'src>> for Return<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        Return {
            span: visitor.span(),
            expr: visitor.expect_node::<Expr>(),
        }
    }
}
