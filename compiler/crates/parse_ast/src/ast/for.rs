use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::{Stmt, Expr};

#[derive(Debug)]
pub struct For {
    pub namespace: String,
    pub init: Expr,
    pub cond: Expr,
    pub incr: Expr,
    pub stmt: Box<Stmt>,
}

impl From<(String, Tree<'_, SBLangDef>)> for For  {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);
        For {
            namespace: namespace.clone(),
            init: Expr::from((namespace.clone(), children.pop_front().unwrap())),
            cond: Expr::from((namespace.clone(), children.pop_front().unwrap())),
            incr: Expr::from((namespace.clone(), children.pop_front().unwrap())),
            stmt: Box::new(Stmt::from((namespace, children.pop_front().unwrap()))),
        }
    }
}
