use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens};

use crate::utils::{unwrap_node, unwrap_leaf};
use super::Value;

#[derive(Debug)]
pub enum Expr {
    Plus(Box<Expr>, Value),
    Minus(Box<Expr>, Value),
    Value(Value),
}

impl From<Tree<'_, SBLangDef>> for Expr {
    fn from(tree: Tree<'_, SBLangDef>) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let value = Value::from(children.pop_front().unwrap());
            return Expr::Value(value);
        }

        // 演算子付き
        let lhs = children.pop_front().unwrap();
        let op = children.pop_front().unwrap();
        let rhs = children.pop_front().unwrap();
        match unwrap_leaf(op).0 {
            SBTokens::Plus => {
                Expr::Plus(Box::new(Expr::from(lhs)), Value::from(rhs))
            }
            SBTokens::Minus => {
                Expr::Minus(Box::new(Expr::from(lhs)), Value::from(rhs))
            }
            _=> unreachable!(),
        }
    }
}
