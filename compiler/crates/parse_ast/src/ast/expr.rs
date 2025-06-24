use sb_compiler_parse_cst::Spanned;

use super::{Assign, Visitor};

#[derive(Debug)]
pub struct Expr<'src> {
    pub assign: Assign<'src>,
}

impl<'src> From<Visitor<'src>> for Expr<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        Expr {
            assign: visitor.expect_node::<Assign>(),
        }
    }
}

impl<'src> Spanned<'src> for Expr<'src> {
    fn span(&self) -> sb_compiler_parse_cst::Span<'src> {
        self.assign.span()
    }
}
