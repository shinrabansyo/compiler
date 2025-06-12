use super::{BitOr, Visitor};

#[derive(Debug)]
pub enum LogicAnd<'input> {
    And {
        lhs: Box<LogicAnd<'input>>,
        rhs: BitOr<'input>,
    },
    BitOr {
        or: BitOr<'input>,
    },
}

impl<'input> From<Visitor<'input>> for LogicAnd<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
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
