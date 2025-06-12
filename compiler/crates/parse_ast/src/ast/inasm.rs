use super::{InlineAsmInst, Visitor};

#[derive(Debug)]
pub struct InlineAsm {
    pub namespace: String,
    pub insts: Vec<InlineAsmInst>,
}

impl From<(String, Visitor<'_>)> for InlineAsm {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        InlineAsm {
            namespace,
            insts: visitor.expect_nodes::<InlineAsmInst>(),
        }
    }
}
