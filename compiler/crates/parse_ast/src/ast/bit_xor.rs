use sb_compiler_parse_cst::Span;

use super::{BitAnd, Visitor};

#[derive(Debug)]
pub enum BitXor<'src> {
    Xor {
        span: Span<'src>,
        lhs: Box<BitXor<'src>>,
        rhs: BitAnd<'src>,
    },
    BitAnd {
        and: BitAnd<'src>,
    },
}

impl<'src> From<Visitor<'src>> for BitXor<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return BitXor::BitAnd {
                and: visitor.expect_node::<BitAnd>(),
            };
        }

        // 演算子付き
        let span = visitor.span();
        let lhs = Box::new(visitor.expect_node::<BitXor>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<BitAnd>();

        BitXor::Xor { span, lhs, rhs }
    }
}
