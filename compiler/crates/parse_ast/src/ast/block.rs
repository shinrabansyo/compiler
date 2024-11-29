use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::{unwrap_node, expand_lrec};
use super::Stmt;

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl From<(String, Tree<'_, SBLangDef>)> for Block {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);
        let stmt_list = children.pop_front().unwrap();
        Block {
            stmts: expand_lrec::<Stmt>(namespace, stmt_list),
        }
    }
}
