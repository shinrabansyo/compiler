use super::{BitOr, Visitor};

#[derive(Debug)]
pub enum LogicAnd {
    And {
        lhs: Box<LogicAnd>,
        rhs: BitOr,
    },
    BitOr {
        or: BitOr,
    },
}

impl From<Visitor<'_>> for LogicAnd {
    fn from(mut visitor: Visitor<'_>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return LogicAnd::BitOr {
                or: visitor.expect_node::<BitOr>(),
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<LogicAnd>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<BitOr>();

        LogicAnd::And { lhs, rhs }
    }
}
