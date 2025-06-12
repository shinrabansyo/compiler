use super::{BitAnd, Visitor};

#[derive(Debug)]
pub enum BitXor {
    Xor {
        lhs: Box<BitXor>,
        rhs: BitAnd,
    },
    BitAnd {
        and: BitAnd,
    },
}

impl From<Visitor<'_>> for BitXor {
    fn from(mut visitor: Visitor<'_>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return BitXor::BitAnd {
                and: visitor.expect_node::<BitAnd>(),
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<BitXor>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<BitAnd>();

        BitXor::Xor { lhs, rhs }
    }
}
