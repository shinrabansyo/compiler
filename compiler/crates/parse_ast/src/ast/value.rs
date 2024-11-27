use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::Expr;

#[derive(Debug)]
pub enum Value {
    Expr {
        namespace: String,
        expr: Box<Expr>,
    },
    Const {
        namespace: String,
        value: i32,
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for Value {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);
        let rhs = children.pop_front().unwrap();
        match rhs {
            // 括弧
            Tree::Node { .. } => {
                let expr = Box::new(Expr::from((namespace.clone(), rhs)));
                Value::Expr{ namespace, expr }
            }
            // 定数
            Tree::Leaf { text, .. } => {
                let value = text.parse().unwrap();
                Value::Const{ namespace, value }
            }
        }
    }
}
