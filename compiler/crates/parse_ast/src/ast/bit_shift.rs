use sb_compiler_parse_syntax::SBToken;

use super::{Add, Visitor};

#[derive(Debug)]
pub enum BitShift<'src> {
    L {
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
    },
    R {
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
    },
    Ra {
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
                    lhs,
                    rhs: visitor.expect_node::<Add>(),
                }
            }
            SBToken::ShiftR => {
                BitShift::R {
                    lhs,
                    rhs: visitor.expect_node::<Add>(),
                }
            }
            SBToken::ShiftRa => {
                BitShift::Ra {
                    lhs,
                    rhs: visitor.expect_node::<Add>(),
                }
            }
            _=> unreachable!(),
        }
    }
}
