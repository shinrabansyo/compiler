use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBToken;

use super::{BitShift, Visitor};

#[derive(Debug)]
pub enum Cond<'src> {
    Eq {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Neq {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Lt {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Lte {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Gt {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Gte {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    BitShift {
        bit_shift: BitShift<'src>,
    },
}

impl<'src> From<Visitor<'src>> for Cond<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Cond::BitShift {
                bit_shift: visitor.expect_node::<BitShift>(),
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<Cond>());
        match visitor.expect_leaf().0 {
            SBToken::Eq => {
                Cond::Eq {
                    span: visitor.span(),
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBToken::Neq => {
                Cond::Neq {
                    span: visitor.span(),
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBToken::Lt => {
                Cond::Lt {
                    span: visitor.span(),
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBToken::Lte => {
                Cond::Lte {
                    span: visitor.span(),
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBToken::Gt => {
                Cond::Gt {
                    span: visitor.span(),
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBToken::Gte => {
                Cond::Gte {
                    span: visitor.span(),
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            _=> unreachable!(),
        }
    }
}

impl<'src> Spanned<'src> for Cond<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Cond::Eq { span, .. } => *span,
            Cond::Neq { span, .. } => *span,
            Cond::Lt { span, .. } => *span,
            Cond::Lte { span, .. } => *span,
            Cond::Gt { span, .. } => *span,
            Cond::Gte { span, .. } => *span,
            Cond::BitShift { bit_shift } => bit_shift.span(),
        }
    }
}
