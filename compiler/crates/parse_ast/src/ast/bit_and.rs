use super::{Cond, Visitor};

#[derive(Debug)]
pub enum BitAnd {
    And {
        namespace: String,
        lhs: Box<BitAnd>,
        rhs: Cond,
    },
    Cond {
        namespace: String,
        cond: Cond,
    },
}

impl From<(String, Visitor<'_>)> for BitAnd {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return BitAnd::Cond {
                namespace,
                cond: visitor.expect_node::<Cond>(),
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<BitAnd>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<Cond>();

        BitAnd::And { namespace, lhs, rhs }
    }
}
