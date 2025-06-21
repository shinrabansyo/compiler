use super::{BitXor, Visitor};

#[derive(Debug)]
pub enum BitOr<'src> {
    Or {
        lhs: Box<BitOr<'src>>,
        rhs: BitXor<'src>,
    },
    BitXor {
        xor: BitXor<'src>,
    },
}

impl<'src> From<Visitor<'src>> for BitOr<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
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
