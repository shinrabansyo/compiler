use super::{BitOr, Visitor};

#[derive(Debug)]
pub enum LogicAnd<'src> {
    And {
        lhs: Box<LogicAnd<'src>>,
        rhs: BitOr<'src>,
    },
    BitOr {
        or: BitOr<'src>,
    },
}

impl<'src> From<Visitor<'src>> for LogicAnd<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
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
