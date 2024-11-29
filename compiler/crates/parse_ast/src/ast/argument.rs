use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::{unwrap_node, unwrap_leaf};

#[derive(Debug)]
pub struct Argument {
    pub namespace: String,
    pub ident: String,
    pub ty: String,
}

impl From<(String, Tree<'_, SBLangDef>)> for Argument {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        let (_, ident) = unwrap_leaf(children.pop_front().unwrap());
        let ident = ident.to_string();

        let (_, ty) = unwrap_leaf(children.pop_front().unwrap());
        let ty = ty.to_string();

        Argument { namespace, ident, ty }
    }
}
