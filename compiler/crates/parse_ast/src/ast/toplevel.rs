use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBRules};

use crate::utils::unwrap_node;
use super::Expr;

#[derive(Debug)]
pub enum TopLevel {
    Expr(Box<Expr>),
    Const(i32),
}

impl From<Tree<'_, SBLangDef>> for TopLevel  {
    fn from(tree: Tree<'_, SBLangDef>) -> Self {
        let (_, mut children) = unwrap_node(tree);
        let top = children.pop_front().unwrap();
        match top {
            // 式
            Tree::Node { tag: SBRules::Expr, .. } => {
                let expr = Expr::from(top);
                TopLevel::Expr(Box::new(expr))
            }
            _ => unreachable!(),
        }
    }
}
