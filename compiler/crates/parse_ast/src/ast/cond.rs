use sb_compiler_parse_syntax::SBToken;

use super::{BitShift, Visitor};

#[derive(Debug)]
pub enum Cond<'input> {
    Eq {
        lhs: Box<Cond<'input>>,
        rhs: BitShift<'input>,
    },
    Neq {
        lhs: Box<Cond<'input>>,
        rhs: BitShift<'input>,
    },
    Lt {
        lhs: Box<Cond<'input>>,
        rhs: BitShift<'input>,
    },
    Lte {
        lhs: Box<Cond<'input>>,
        rhs: BitShift<'input>,
    },
    Gt {
        lhs: Box<Cond<'input>>,
        rhs: BitShift<'input>,
    },
    Gte {
        lhs: Box<Cond<'input>>,
        rhs: BitShift<'input>,
    },
    BitShift {
        bit_shift: BitShift<'input>,
    },
}

impl<'input> From<Visitor<'input>> for Cond<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
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
