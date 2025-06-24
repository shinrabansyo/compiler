use sb_compiler_parse_cst::Span;

use super::{LogicAnd, Visitor};

#[derive(Debug)]
pub enum LogicOr<'src> {
    Or {
        span: Span<'src>,
        lhs: Box<LogicOr<'src>>,
        rhs: LogicAnd<'src>,
    },
    LogicAnd {
        and: LogicAnd<'src>,
    },
}

impl<'src> From<Visitor<'src>> for LogicOr<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return LogicOr::LogicAnd {
                and: visitor.expect_node::<LogicAnd>(),
            };
        }

        // 演算子付き
        let span = visitor.span();
        let lhs = Box::new(visitor.expect_node::<LogicOr>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<LogicAnd>();

        LogicOr::Or { span, lhs, rhs }
    }
}
