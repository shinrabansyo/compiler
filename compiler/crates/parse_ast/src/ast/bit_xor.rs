use super::{BitAnd, Visitor};

#[derive(Debug)]
pub enum BitXor<'input> {
    Xor {
        lhs: Box<BitXor<'input>>,
        rhs: BitAnd<'input>,
    },
    BitAnd {
        and: BitAnd<'input>,
    },
}

impl<'input> From<Visitor<'input>> for BitXor<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
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
