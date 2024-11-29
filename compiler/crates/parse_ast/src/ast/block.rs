use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
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
            stmts: expand_lrec(namespace, stmt_list),
        }
    }
}

fn expand_lrec(namespace: String, tree: Tree<'_, SBLangDef>) -> Vec<Stmt> {
    let (_, mut children) = unwrap_node(tree);
    if children.len() == 1 {
        vec![Stmt::from((
            namespace,
            children.pop_front().unwrap()
        ))]
    } else {
        let mut stmts = expand_lrec(namespace.clone(), children.pop_front().unwrap());
        stmts.push(Stmt::from((
            namespace,
            children.pop_front().unwrap()
        )));
        stmts
    }
}
