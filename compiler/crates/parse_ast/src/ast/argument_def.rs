use sb_compiler_parse_cst::Span;

use super::Visitor;

#[derive(Debug)]
pub struct ArgumentDef<'src> {
    pub ident: Span<'src>,
    pub ty: Span<'src>,
}

impl<'src> From<Visitor<'src>> for ArgumentDef<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        ArgumentDef {
            ident: visitor.expect_leaf().1,
            ty: visitor.expect_leaf().1,
        }
    }
}
