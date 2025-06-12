use sb_compiler_parse_syntax::SBToken;

use super::{BitShift, Visitor};

#[derive(Debug)]
pub enum Cond {
    Eq {
        lhs: Box<Cond>,
        rhs: BitShift,
    },
    Neq {
        lhs: Box<Cond>,
        rhs: BitShift,
    },
    Lt {
        lhs: Box<Cond>,
        rhs: BitShift,
    },
    Lte {
        lhs: Box<Cond>,
        rhs: BitShift,
    },
    Gt {
        lhs: Box<Cond>,
        rhs: BitShift,
    },
    Gte {
        lhs: Box<Cond>,
        rhs: BitShift,
    },
    BitShift {
        bit_shift: BitShift,
    },
}

impl From<Visitor<'_>> for Cond {
    fn from(mut visitor: Visitor<'_>) -> Self {
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
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBToken::Neq => {
                Cond::Neq {
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBToken::Lt => {
                Cond::Lt {
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBToken::Lte => {
                Cond::Lte {
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBToken::Gt => {
                Cond::Gt {
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBToken::Gte => {
                Cond::Gte {
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            _=> unreachable!(),
        }
    }
}
