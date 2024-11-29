use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::{unwrap_node, unwrap_leaf};
use super::Block;

#[derive(Debug)]
pub struct FuncDef {
    pub namespace: String,
    pub ident: String,
    pub block: Block,
}

impl From<(String, Tree<'_, SBLangDef>)> for FuncDef {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        let (_, ident) = unwrap_leaf(children.pop_front().unwrap());
        let ident = ident.to_string();

        let func_namespace = format!("{}.{}", ident, namespace);
        let block = Block::from((func_namespace.clone(), children.pop_front().unwrap()));

        FuncDef { namespace, ident, block }
    }
}
