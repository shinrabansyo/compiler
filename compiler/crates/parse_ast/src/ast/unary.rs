use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens};

use crate::utils::unwrap_node;
use super::Value;

#[derive(Debug)]
pub enum Unary {
    Plus {
        namespace: String,
        value: Value,
    },
    Minus {
        namespace: String,
        value: Value,
    },
    Value {
        namespace: String,
        value: Value
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for Unary {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let value = Value::from((namespace.clone(), children.pop_front().unwrap()));
            return Unary::Value { namespace, value };
        }

        // 演算子付き
        let op = children.pop_front().unwrap();
        match op {
            Tree::Leaf { tag: SBTokens::Plus, .. } => {
                let value = Value::from((namespace.clone(), children.pop_front().unwrap()));
                Unary::Plus { namespace, value }
            }
            Tree::Leaf { tag: SBTokens::Minus, .. } => {
                let value = Value::from((namespace.clone(), children.pop_front().unwrap()));
                Unary::Minus { namespace, value }
            }
            _ => unreachable!(),
        }
    }
}
