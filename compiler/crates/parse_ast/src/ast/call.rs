use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::{unwrap_node, unwrap_leaf, expand_lrec};
use super::Value;

#[derive(Debug)]
pub struct Call {
    pub namespace: String,
    pub ident: String,
    pub args: Vec<Value>,
}

impl From<(String, Tree<'_, SBLangDef>)> for Call {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        let (_, ident) = unwrap_leaf(children.pop_front().unwrap());
        let ident = ident.to_string();

        let args = expand_lrec::<Value>(
            namespace.clone(),
            children.pop_front().unwrap()
        );

        Call { namespace, ident, args }
    }
}
