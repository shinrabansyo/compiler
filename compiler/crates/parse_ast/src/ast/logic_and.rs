use super::{BitOr, Visitor};

#[derive(Debug)]
pub enum LogicAnd {
    And {
        namespace: String,
        lhs: Box<LogicAnd>,
        rhs: BitOr,
    },
    BitOr {
        namespace: String,
        or: BitOr,
    },
}

impl From<(String, Visitor<'_>)> for LogicAnd {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return LogicAnd::BitOr {
                namespace,
                or: visitor.expect_node::<BitOr>(),
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<LogicAnd>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<BitOr>();

        LogicAnd::And { namespace, lhs, rhs }
    }
}
