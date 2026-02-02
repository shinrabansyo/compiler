use sb_compiler_parse_cst::{Span, Spanned};

use super::{Type, Visitor};

#[derive(Debug)]
pub struct FieldDef<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub ty: Type<'src>,
}

impl<'src> From<Visitor<'src>> for FieldDef<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        FieldDef {
            span: visitor.span(),
            ident: visitor.expect_leaf().1,
            ty: visitor.expect_node::<Type>(),
        }
    }
}

impl<'src> Spanned<'src> for FieldDef<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
