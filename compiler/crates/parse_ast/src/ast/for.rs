use sb_compiler_parse_cst::{Span, Spanned};

use super::{Block, Expr, VarDecl, Visitor};

#[derive(Debug)]
pub struct For<'src> {
    pub span: Span<'src>,
    pub init: VarDecl<'src>,
    pub cond: Expr<'src>,
    pub incr: Expr<'src>,
    pub block: Block<'src>,
}

impl<'src> From<Visitor<'src>> for For<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        For {
            span: visitor.span(),
            init: visitor.expect_node::<VarDecl>(),
            cond: visitor.expect_node::<Expr>(),
            incr: visitor.expect_node::<Expr>(),
            block: visitor.expect_node::<Block>(),
        }
    }
}

impl<'src> Spanned<'src> for For<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
