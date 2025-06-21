use super::{Cond, Visitor};

#[derive(Debug)]
pub enum BitAnd<'src> {
    And {
        lhs: Box<BitAnd<'src>>,
        rhs: Cond<'src>,
    },
    Cond {
        cond: Cond<'src>,
    },
}

impl<'src> From<Visitor<'src>> for BitAnd<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
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
