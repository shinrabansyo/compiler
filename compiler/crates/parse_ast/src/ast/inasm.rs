use super::{InlineAsmInst, Visitor};

#[derive(Debug)]
pub struct InlineAsm<'input> {
    pub insts: Vec<InlineAsmInst<'input>>,
}

impl<'input> From<Visitor<'input>> for InlineAsm<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        InlineAsm {
            insts: visitor.expect_nodes::<InlineAsmInst>(),
        }
    }
}
