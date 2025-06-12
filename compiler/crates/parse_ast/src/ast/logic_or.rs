use super::{LogicAnd, Visitor};

#[derive(Debug)]
pub enum LogicOr<'input> {
    Or {
        lhs: Box<LogicOr<'input>>,
        rhs: LogicAnd<'input>,
    },
    LogicAnd {
        and: LogicAnd<'input>,
    },
}

impl<'input> From<Visitor<'input>> for LogicOr<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
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
