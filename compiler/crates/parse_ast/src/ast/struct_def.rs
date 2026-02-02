use sb_compiler_parse_cst::{Span, Spanned};

use super::{FieldDef, Visitor};

#[derive(Debug)]
pub struct StructDef<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub fields: Vec<FieldDef<'src>>,
}

impl<'src> From<Visitor<'src>> for StructDef<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
         StructDef {
            span: visitor.span(),
            ident: visitor.expect_leaf().1,
            fields: visitor.expect_nodes::<FieldDef>(),
        }
    }
}

impl<'src> Spanned<'src> for StructDef<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
