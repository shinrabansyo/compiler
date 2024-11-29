use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens};

use crate::utils::{unwrap_node, unwrap_leaf, expand_lrec};
use super::{Block, ArgumentDef};

#[derive(Debug)]
pub struct FuncDef {
    pub namespace: String,
    pub ident: String,
    pub args: Vec<ArgumentDef>,
    pub ret_ty: Option<String>,
    pub block: Block,
}

impl From<(String, Tree<'_, SBLangDef>)> for FuncDef {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        let (_, ident) = unwrap_leaf(children.pop_front().unwrap());
        let ident = ident.to_string();
        let func_namespace = format!("{}.{}", ident, namespace);

        let args = expand_lrec::<ArgumentDef>(
            func_namespace.clone(),
            children.pop_front().unwrap()
        );

        let (ret_ty, block) = if matches!(children[0], Tree::Leaf { tag: SBTokens::Type, .. }) {
            let (_, ret_type) = unwrap_leaf(children.pop_front().unwrap());
            let ret_ty = ret_type.to_string();
            let block = Block::from((func_namespace, children.pop_front().unwrap()));
            (Some(ret_ty), block)
        } else {
            let block = Block::from((func_namespace, children.pop_front().unwrap()));
            (None, block)
        };

        FuncDef { namespace, ident, args, ret_ty, block }
    }
}
