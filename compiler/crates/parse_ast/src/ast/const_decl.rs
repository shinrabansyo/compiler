use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::{unwrap_node, unwrap_leaf};
use super::Expr;

#[derive(Debug)]
pub struct ConstDecl {
    pub namespace: String,
    pub ident: String,
    pub ty: String,
    pub expr: Expr,
}

impl From<(String, Tree<'_, SBLangDef>)> for ConstDecl  {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        let (_, ident) = unwrap_leaf(children.pop_front().unwrap());
        let ident = ident.to_string();

        let (_, ty) = unwrap_leaf(children.pop_front().unwrap());
        let ty = ty.to_string();

        let _assign = children.pop_front().unwrap();

        let expr = Expr::from((namespace.clone(), children.pop_front().unwrap()));

        ConstDecl { namespace, ident, ty, expr }
    }
}
