use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::Cond;

#[derive(Debug)]
pub struct Expr {
    pub cond: Cond,
}

impl From<(String, Tree<'_, SBLangDef>)> for Expr {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);
        Expr {
            cond: Cond::from((namespace, children.pop_front().unwrap())),
        }
    }
}
