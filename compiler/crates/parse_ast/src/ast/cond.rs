use sb_compiler_parse_syntax::SBToken;

use super::{BitShift, Visitor};

#[derive(Debug)]
pub enum Cond<'src> {
    Eq {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Neq {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Lt {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Lte {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Gt {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Gte {
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
