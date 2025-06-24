use sb_compiler_parse_cst::{Span, Spanned};

use super::{InlineAsmInst, Visitor};

#[derive(Debug)]
pub struct InlineAsm<'src> {
    pub span: Span<'src>,
    pub insts: Vec<InlineAsmInst<'src>>,
}

impl<'src> From<Visitor<'src>> for InlineAsm<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        InlineAsm {
            span: visitor.span(),
            insts: visitor.expect_nodes::<InlineAsmInst>(),
        }
    }
}

impl<'src> Spanned<'src> for InlineAsm<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
