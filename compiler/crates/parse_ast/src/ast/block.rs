use sb_compiler_parse_cst::{Span, Spanned};

use super::{Stmt, Visitor};

#[derive(Debug)]
pub struct Block<'src> {
    pub span: Span<'src>,
    pub stmts: Vec<Stmt<'src>>,
}

impl<'src> From<Visitor<'src>> for Block<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        Block {
            span: visitor.span(),
            stmts: visitor.expect_nodes::<Stmt>(),
        }
    }
}

impl<'src> Spanned<'src> for Block<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
