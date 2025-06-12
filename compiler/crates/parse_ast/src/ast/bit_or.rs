use super::{BitXor, Visitor};

#[derive(Debug)]
pub enum BitOr<'input> {
    Or {
        lhs: Box<BitOr<'input>>,
        rhs: BitXor<'input>,
    },
    BitXor {
        xor: BitXor<'input>,
    },
}

impl<'input> From<Visitor<'input>> for BitOr<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return BitOr::BitXor {
                xor: visitor.expect_node::<BitXor>()
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<BitOr>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<BitXor>();

        BitOr::Or { lhs, rhs }
    }
}
