use super::{LogicAnd, Visitor};

#[derive(Debug)]
pub enum LogicOr {
    Or {
        namespace: String,
        lhs: Box<LogicOr>,
        rhs: LogicAnd,
    },
    LogicAnd {
        namespace: String,
        and: LogicAnd,
    },
}

impl From<(String, Visitor<'_>)> for LogicOr {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return LogicOr::LogicAnd {
                namespace,
                and: visitor.expect_node::<LogicAnd>(),
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<LogicOr>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<LogicAnd>();

        LogicOr::Or { namespace, lhs, rhs }
    }
}
