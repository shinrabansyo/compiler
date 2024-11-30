use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::LogicOr;

#[derive(Debug)]
pub struct Expr {
    pub or: LogicOr,
}

impl From<(String, Tree<'_, SBLangDef>)> for Expr {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);
        Expr {
            or: LogicOr::from((namespace, children.pop_front().unwrap())),
        }
    }
}
