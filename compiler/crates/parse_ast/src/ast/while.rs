use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::{Block, Expr};

#[derive(Debug)]
pub struct While {
    pub namespace: String,
    pub cond: Expr,
    pub block: Block,
}

impl From<(String, Tree<'_, SBLangDef>)> for While  {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);
        While {
            namespace: namespace.clone(),
            cond: Expr::from((namespace.clone(), children.pop_front().unwrap())),
            block: Block::from((namespace, children.pop_front().unwrap())),
        }
    }
}
