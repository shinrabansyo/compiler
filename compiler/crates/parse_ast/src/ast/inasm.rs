use super::{InlineAsmInst, Visitor};

#[derive(Debug)]
pub struct InlineAsm<'src> {
    pub insts: Vec<InlineAsmInst<'src>>,
}

impl<'src> From<Visitor<'src>> for InlineAsm<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        InlineAsm {
            insts: visitor.expect_nodes::<InlineAsmInst>(),
        }
    }
}
