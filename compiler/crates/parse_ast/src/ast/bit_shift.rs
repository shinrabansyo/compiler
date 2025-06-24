use sb_compiler_parse_cst::Span;
use sb_compiler_parse_syntax::SBToken;

use super::{Add, Visitor};

#[derive(Debug)]
pub enum BitShift<'src> {
    L {
        span: Span<'src>,
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
    },
    R {
        span: Span<'src>,
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
    },
    Ra {
        span: Span<'src>,
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
    },
    Add {
        add: Add<'src>,
    },
}

impl<'src> From<Visitor<'src>> for BitShift<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return BitShift::Add {
                add: visitor.expect_node::<Add>(),
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<BitShift>());
        match visitor.expect_leaf().0 {
            SBToken::ShiftL => {
                BitShift::L {
                    span: visitor.span(),
                    lhs,
                    rhs: visitor.expect_node::<Add>(),
                }
            }
            SBToken::ShiftR => {
                BitShift::R {
                    span: visitor.span(),
                    lhs,
                    rhs: visitor.expect_node::<Add>(),
                }
            }
            SBToken::ShiftRa => {
                BitShift::Ra {
                    span: visitor.span(),
                    lhs,
                    rhs: visitor.expect_node::<Add>(),
                }
            }
            _=> unreachable!(),
        }
    }
}
