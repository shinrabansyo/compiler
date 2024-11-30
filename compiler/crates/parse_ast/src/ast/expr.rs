use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::Logic;

#[derive(Debug)]
pub struct Expr {
    pub logic: Logic,
}

impl From<(String, Tree<'_, SBLangDef>)> for Expr {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);
        Expr {
            logic: Logic::from((namespace, children.pop_front().unwrap())),
        }
    }
}
