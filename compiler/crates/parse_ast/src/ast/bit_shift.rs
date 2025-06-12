use sb_compiler_parse_syntax::SBToken;

use super::{Add, Visitor};

#[derive(Debug)]
pub enum BitShift {
    L {
        lhs: Box<BitShift>,
        rhs: Add,
    },
    R {
        lhs: Box<BitShift>,
        rhs: Add,
    },
    Ra {
        lhs: Box<BitShift>,
        rhs: Add,
    },
    Add {
        add: Add,
    },
}

impl From<Visitor<'_>> for BitShift {
    fn from(mut visitor: Visitor<'_>) -> Self {
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
