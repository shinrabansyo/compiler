use sb_compiler_parse_cst::{Span, Spanned};

use super::{Block, Expr, Visitor};

#[derive(Debug)]
pub struct While<'src> {
    pub span: Span<'src>,
    pub cond: Expr<'src>,
    pub block: Block<'src>,
}

impl<'src> From<Visitor<'src>> for While<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        While {
            span: visitor.span(),
            cond: visitor.expect_node::<Expr>(),
            block: visitor.expect_node::<Block>(),
        }
    }
}

impl<'src> Spanned<'src> for While<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
