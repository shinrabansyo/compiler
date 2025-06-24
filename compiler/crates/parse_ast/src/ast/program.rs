use sb_compiler_parse_cst::{Span, Spanned};

use super::{Top, Visitor};

#[derive(Debug)]
pub struct Program<'src> {
    pub span: Span<'src>,
    pub top_elems: Vec<Top<'src>>,
}

impl<'src> From<Visitor<'src>> for Program<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        Program {
            span: visitor.span(),
            top_elems: visitor.expect_nodes::<Top>(),
        }
    }
}

impl<'src> Spanned<'src> for Program<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
