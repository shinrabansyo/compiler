use super::{Cond, Visitor};

#[derive(Debug)]
pub enum BitAnd<'input> {
    And {
        lhs: Box<BitAnd<'input>>,
        rhs: Cond<'input>,
    },
    Cond {
        cond: Cond<'input>,
    },
}

impl<'input> From<Visitor<'input>> for BitAnd<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
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
