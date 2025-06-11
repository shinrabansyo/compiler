use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens};

use crate::utils::{unwrap_node, unwrap_leaf};
use super::Unary;

#[derive(Debug)]
pub enum Add {
    Plus {
        namespace: String,
        lhs: Box<Add>,
        rhs: Unary,
    },
    Minus {
        namespace: String,
        lhs: Box<Add>,
        rhs: Unary,
    },
    Unary {
        namespace: String,
        value: Unary
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for Add {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let value = Unary::from((namespace.clone(), children.pop_front().unwrap()));
            return Add::Unary { namespace, value };
        }

        // 演算子付き
        let lhs = children.pop_front().unwrap();
        let op = children.pop_front().unwrap();
        let rhs = children.pop_front().unwrap();
        match unwrap_leaf(op).0 {
            SBTokens::Plus => {
                let lhs = Box::new(Add::from((namespace.clone(), lhs)));
                let rhs = Unary::from((namespace.clone(), rhs));
                Add::Plus { namespace, lhs, rhs }
            }
            SBTokens::Minus => {
                let lhs = Box::new(Add::from((namespace.clone(), lhs)));
                let rhs = Unary::from((namespace.clone(), rhs));
                Add::Minus { namespace, lhs, rhs }
            }
            _=> unreachable!(),
        }
    }
}
