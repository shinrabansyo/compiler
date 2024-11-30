use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens, SBRules};

use crate::utils::unwrap_node;
use super::{Expr, Call};

#[derive(Debug)]
pub enum Value {
    Const {
        namespace: String,
        value: i32,
    },
    Var {
        namespace: String,
        name: String,
    },
    Expr {
        namespace: String,
        expr: Box<Expr>,
    },
    Call {
        namespace: String,
        call: Call,
    }
}

impl From<(String, Tree<'_, SBLangDef>)> for Value {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);
        let rhs = children.pop_front().unwrap();
        match rhs {
            // 定数
            Tree::Leaf { tag: SBTokens::Num, text, .. } => {
                let value = text.parse().unwrap();
                Value::Const{ namespace, value }
            }
            // 変数
            Tree::Leaf { tag: SBTokens::Ident, text, .. } => {
                let name = text.to_string();
                Value::Var{ namespace, name }
            }
            // 括弧
            Tree::Node { tag: SBRules::Expr, .. } => {
                let expr = Box::new(Expr::from((namespace.clone(), rhs)));
                Value::Expr{ namespace, expr }
            }
            // 関数呼び出し
            Tree::Node { tag: SBRules::Call, .. } => {
                let call = Call::from((namespace.clone(), rhs));
                Value::Call{ namespace, call }
            }
            _ => unreachable!(),
        }
    }
}
