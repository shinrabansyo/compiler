use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens};

use crate::utils::{unwrap_node, unwrap_leaf};
use super::Value;

#[derive(Debug)]
pub enum Add {
    Plus {
        namespace: String,
        lhs: Box<Add>,
        rhs: Value,
    },
    Minus {
        namespace: String,
        lhs: Box<Add>,
        rhs: Value,
    },
    Value {
        namespace: String,
        value: Value
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for Add {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let value = Value::from((namespace.clone(), children.pop_front().unwrap()));
            return Add::Value { namespace, value };
        }

        // 演算子付き
        let lhs = children.pop_front().unwrap();
        let op = children.pop_front().unwrap();
        let rhs = children.pop_front().unwrap();
        match unwrap_leaf(op).0 {
            SBTokens::Plus => {
                let lhs = Box::new(Add::from((namespace.clone(), lhs)));
                let rhs = Value::from((namespace.clone(), rhs));
                Add::Plus { namespace, lhs, rhs }
            }
            SBTokens::Minus => {
                let lhs = Box::new(Add::from((namespace.clone(), lhs)));
                let rhs = Value::from((namespace.clone(), rhs));
                Add::Minus { namespace, lhs, rhs }
            }
            _=> unreachable!(),
        }
    }
}
