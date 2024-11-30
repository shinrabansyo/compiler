use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::{Block, Stmt, Expr};

#[derive(Debug)]
pub struct If {
    pub namespace: String,
    pub cond: Expr,
    pub block: Block,
    pub else_stmt: Option<Box<Stmt>>,
}

impl From<(String, Tree<'_, SBLangDef>)> for If  {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        let cond = Expr::from((namespace.clone(), children.pop_front().unwrap()));

        let block = Block::from((namespace.clone(), children.pop_front().unwrap()));

        let else_stmt = if !children.is_empty() {
            Some(Box::new(Stmt::from((namespace.clone(), children.pop_front().unwrap()))))
        } else {
            None
        };

        If { namespace, cond, block, else_stmt }
    }
}
