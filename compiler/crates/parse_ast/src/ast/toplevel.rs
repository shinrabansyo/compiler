use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBRules};

use crate::utils::unwrap_node;
use super::Expr;

#[derive(Debug)]
pub enum TopLevel {
    Expr(Box<Expr>),
}

impl From<Tree<'_, SBLangDef>> for TopLevel  {
    fn from(tree: Tree<'_, SBLangDef>) -> Self {
        let (_, mut children) = unwrap_node(tree);
        let rhs = children.pop_front().unwrap();
        match rhs {
            // 式
            Tree::Node { tag: SBRules::Expr, .. } => {
                let expr = Expr::from(rhs);
                TopLevel::Expr(Box::new(expr))
            }
            _ => unreachable!(),
        }
    }
}
