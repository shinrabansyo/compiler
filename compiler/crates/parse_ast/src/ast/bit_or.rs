use sb_compiler_parse_cst::{Span, Spanned};

use super::{BitXor, Visitor};

#[derive(Debug)]
pub enum BitOr<'src> {
    Or {
        span: Span<'src>,
        lhs: Box<BitOr<'src>>,
        rhs: BitXor<'src>,
    },
    BitXor {
        xor: BitXor<'src>,
    },
}

impl<'src> From<Visitor<'src>> for BitOr<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return BitOr::BitXor {
                xor: visitor.expect_node::<BitXor>()
            };
        }

        // 演算子付き
        let span = visitor.span();
        let lhs = Box::new(visitor.expect_node::<BitOr>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<BitXor>();

        BitOr::Or { span, lhs, rhs }
    }
}

impl<'src> Spanned<'src> for BitOr<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            BitOr::Or { span, .. } => *span,
            BitOr::BitXor { xor } => xor.span(),
        }
    }
}
