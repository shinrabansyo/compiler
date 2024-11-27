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
        match children.pop_front().unwrap() {
            // 括弧
            Tree::Node { mut children, .. } => {
                let expr = Expr::from(children.pop_front().unwrap());
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
