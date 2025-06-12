use sb_compiler_parse_syntax::SBTokens;

use super::{Add, Visitor};

#[derive(Debug)]
pub enum BitShift {
    L {
        namespace: String,
        lhs: Box<BitShift>,
        rhs: Add,
    },
    R {
        namespace: String,
        lhs: Box<BitShift>,
        rhs: Add,
    },
    Ra {
        namespace: String,
        lhs: Box<BitShift>,
        rhs: Add,
    },
    Add {
        namespace: String,
        add: Add,
    },
}

impl From<(String, Visitor<'_>)> for BitShift {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return BitShift::Add {
                namespace,
                add: visitor.expect_node::<Add>(),
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<BitShift>());
        match visitor.expect_leaf().0 {
            SBTokens::ShiftL => {
                BitShift::L {
                    namespace,
                    lhs,
                    rhs: visitor.expect_node::<Add>(),
                }
            }
            SBTokens::ShiftR => {
                BitShift::R {
                    namespace,
                    lhs,
                    rhs: visitor.expect_node::<Add>(),
                }
            }
            SBTokens::ShiftRa => {
                BitShift::Ra {
                    namespace,
                    lhs,
                    rhs: visitor.expect_node::<Add>(),
                }
            }
            _=> unreachable!(),
        }
    }
}
