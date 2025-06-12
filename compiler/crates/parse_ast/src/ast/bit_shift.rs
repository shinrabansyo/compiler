use sb_compiler_parse_syntax::SBToken;

use super::{Add, Visitor};

#[derive(Debug)]
pub enum BitShift<'input> {
    L {
        lhs: Box<BitShift<'input>>,
        rhs: Add<'input>,
    },
    R {
        lhs: Box<BitShift<'input>>,
        rhs: Add<'input>,
    },
    Ra {
        lhs: Box<BitShift<'input>>,
        rhs: Add<'input>,
    },
    Add {
        add: Add<'input>,
    },
}

impl<'input> From<Visitor<'input>> for BitShift<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
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
