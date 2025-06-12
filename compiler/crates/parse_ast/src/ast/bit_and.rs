use super::{Cond, Visitor};

#[derive(Debug)]
pub enum BitAnd {
    And {
        lhs: Box<BitAnd>,
        rhs: Cond,
    },
    Cond {
        cond: Cond,
    },
}

impl From<Visitor<'_>> for BitAnd {
    fn from(mut visitor: Visitor<'_>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return BitAnd::Cond {
                cond: visitor.expect_node::<Cond>(),
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<BitAnd>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<Cond>();

        BitAnd::And { lhs, rhs }
    }
}
