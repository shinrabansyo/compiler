use super::{InlineAsmInst, Visitor};

#[derive(Debug)]
pub struct InlineAsm {
    pub insts: Vec<InlineAsmInst>,
}

impl From<Visitor<'_>> for InlineAsm {
    fn from(mut visitor: Visitor<'_>) -> Self {
        InlineAsm {
            insts: visitor.expect_nodes::<InlineAsmInst>(),
        }
    }
}
