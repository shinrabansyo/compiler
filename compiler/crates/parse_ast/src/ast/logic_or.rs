use super::{LogicAnd, Visitor};

#[derive(Debug)]
pub enum LogicOr {
    Or {
        lhs: Box<LogicOr>,
        rhs: LogicAnd,
    },
    LogicAnd {
        and: LogicAnd,
    },
}

impl From<Visitor<'_>> for LogicOr {
    fn from(mut visitor: Visitor<'_>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return LogicOr::LogicAnd {
                and: visitor.expect_node::<LogicAnd>(),
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<LogicOr>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<LogicAnd>();

        LogicOr::Or { lhs, rhs }
    }
}
