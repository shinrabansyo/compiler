use sb_compiler_parse_cst::{Span, Spanned};

use super::{Type, Visitor};

#[derive(Debug)]
pub struct ArgumentDef<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub ty: Type<'src>,
}

impl<'src> From<Visitor<'src>> for ArgumentDef<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        ArgumentDef {
            span: visitor.span(),
            ident: visitor.expect_leaf().1,
            ty: visitor.expect_node::<Type>(),
        }
    }
}

impl<'src> Spanned<'src> for ArgumentDef<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
