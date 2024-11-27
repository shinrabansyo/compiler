use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::{unwrap_node, unwrap_leaf};
use super::Expr;

#[derive(Debug)]
pub struct ConstDecl {
    pub ident: String,
    pub expr: Expr,
}

impl From<Tree<'_, SBLangDef>> for ConstDecl  {
    fn from(tree: Tree<'_, SBLangDef>) -> Self {
        let (_, mut children) = unwrap_node(tree);
        let (_, ident) = unwrap_leaf(children.pop_front().unwrap());
        let ident = ident.to_string();
        let expr = Expr::from(children.pop_front().unwrap());
        ConstDecl { ident, expr }
    }
}
