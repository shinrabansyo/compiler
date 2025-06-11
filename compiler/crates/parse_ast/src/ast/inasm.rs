use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::{unwrap_node, expand_lrec};
use super::InlineAsmInst;

#[derive(Debug)]
pub struct InlineAsm {
    pub namespace: String,
    pub insts: Vec<InlineAsmInst>,
}

impl From<(String, Tree<'_, SBLangDef>)> for InlineAsm {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        let insts = expand_lrec::<InlineAsmInst>(
            namespace.clone(),
            children.pop_front().unwrap(),
        );

        InlineAsm { namespace, insts }
    }
}
