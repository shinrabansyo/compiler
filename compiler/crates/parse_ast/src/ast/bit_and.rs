use sb_compiler_parse_cst::{Span, Spanned};

use super::{Cond, Visitor};

#[derive(Debug)]
pub enum BitAnd<'src> {
    And {
        span: Span<'src>,
        lhs: Box<BitAnd<'src>>,
        rhs: Cond<'src>,
    },
    Cond {
        cond: Cond<'src>,
    },
}

impl<'src> From<Visitor<'src>> for BitAnd<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return BitAnd::Cond {
                cond: visitor.expect_node::<Cond>(),
            };
        }

        // 演算子付き
        let span = visitor.span();
        let lhs = Box::new(visitor.expect_node::<BitAnd>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_node::<Cond>();

        BitAnd::And { span, lhs, rhs }
    }
}

impl<'src> Spanned<'src> for BitAnd<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            BitAnd::And { span, .. } => *span,
            BitAnd::Cond { cond } => cond.span(),
        }
    }
}
