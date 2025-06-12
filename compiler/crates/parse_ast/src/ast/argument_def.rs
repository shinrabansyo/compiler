use sb_compiler_parse_cst::Span;

use super::Visitor;

#[derive(Debug)]
pub struct ArgumentDef<'input> {
    pub ident: Span<'input>,
    pub ty: Span<'input>,
}

impl<'input> From<Visitor<'input>> for ArgumentDef<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        ArgumentDef {
            ident: visitor.expect_leaf().1,
            ty: visitor.expect_leaf().1,
        }
    }
}
