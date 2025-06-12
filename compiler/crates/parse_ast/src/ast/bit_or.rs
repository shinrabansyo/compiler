use super::{BitXor, Visitor};

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

impl From<(String, Visitor<'_>)> for BitOr {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return BitOr::BitXor {
                namespace,
                xor: visitor.expect_node::<BitXor>()
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<BitOr>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<BitXor>();

        BitOr::Or { namespace, lhs, rhs }
    }
}
