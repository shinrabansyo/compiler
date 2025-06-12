use super::{BitAnd, Visitor};

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

impl From<(String, Visitor<'_>)> for BitXor {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return BitXor::BitAnd {
                namespace,
                and: visitor.expect_node::<BitAnd>(),
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<BitXor>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<BitAnd>();

        BitXor::Xor { namespace, lhs, rhs }
    }
}
