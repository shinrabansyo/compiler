use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::Expr;

#[derive(Debug)]
pub enum Value {
    Expr(Box<Expr>),
    Const(i32),
}

impl From<Tree<'_, SBLangDef>> for Value {
    fn from(tree: Tree<'_, SBLangDef>) -> Self {
        let (_, mut children) = unwrap_node(tree);
        let rhs = children.pop_front().unwrap();
        match rhs {
            // 括弧
            Tree::Node { .. } => {
                let expr = Expr::from(rhs);
                Value::Expr(Box::new(expr))
            }
            // 定数
            Tree::Leaf { text, .. } => {
                let value = text.parse().unwrap();
                Value::Const(value)
            }
        }
    }
}
