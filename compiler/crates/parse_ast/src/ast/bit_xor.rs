use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::BitAnd;

#[derive(Debug)]
pub enum BitXor {
    Xor {
        namespace: String,
        lhs: Box<BitXor>,
        rhs: BitAnd,
    },
    BitAnd {
        namespace: String,
        and: BitAnd,
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for BitXor {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let and = BitAnd::from((namespace.clone(), children.pop_front().unwrap()));
            return BitXor::BitAnd { namespace, and };
        }

        // 演算子付き
        let lhs = children.pop_front().unwrap();
        let lhs = Box::new(BitXor::from((namespace.clone(), lhs)));

        let _op = children.pop_front().unwrap();

        let rhs = children.pop_front().unwrap();
        let rhs = BitAnd::from((namespace.clone(), rhs));

        BitXor::Xor { namespace, lhs, rhs }
    }
}
