use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::BitXor;

#[derive(Debug)]
pub enum BitOr {
    Or {
        namespace: String,
        lhs: Box<BitOr>,
        rhs: BitXor,
    },
    BitXor {
        namespace: String,
        xor: BitXor,
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for BitOr {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let xor = BitXor::from((namespace.clone(), children.pop_front().unwrap()));
            return BitOr::BitXor { namespace, xor };
        }

        // 演算子付き
        let lhs = children.pop_front().unwrap();
        let lhs = Box::new(BitOr::from((namespace.clone(), lhs)));

        let _op = children.pop_front().unwrap();

        let rhs = children.pop_front().unwrap();
        let rhs = BitXor::from((namespace.clone(), rhs));

        BitOr::Or { namespace, lhs, rhs }
    }
}
