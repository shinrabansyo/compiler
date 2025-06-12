use sb_compiler_parse_syntax::SBTokens;

use super::{BitShift, Visitor};

#[derive(Debug)]
pub enum Cond {
    Eq {
        namespace: String,
        lhs: Box<Cond>,
        rhs: BitShift,
    },
    Neq {
        namespace: String,
        lhs: Box<Cond>,
        rhs: BitShift,
    },
    Lt {
        namespace: String,
        lhs: Box<Cond>,
        rhs: BitShift,
    },
    Lte {
        namespace: String,
        lhs: Box<Cond>,
        rhs: BitShift,
    },
    Gt {
        namespace: String,
        lhs: Box<Cond>,
        rhs: BitShift,
    },
    Gte {
        namespace: String,
        lhs: Box<Cond>,
        rhs: BitShift,
    },
    BitShift {
        namespace: String,
        bit_shift: BitShift,
    },
}

impl From<(String, Visitor<'_>)> for Cond {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Cond::BitShift {
                namespace,
                bit_shift: visitor.expect_node::<BitShift>(),
            };
        }

        // 演算子付き
        let lhs = Box::new(visitor.expect_node::<Cond>());
        match visitor.expect_leaf().0 {
            SBTokens::Eq => {
                Cond::Eq {
                    namespace,
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBTokens::Neq => {
                Cond::Neq {
                    namespace,
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBTokens::Lt => {
                Cond::Lt {
                    namespace,
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBTokens::Lte => {
                Cond::Lte {
                    namespace,
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBTokens::Gt => {
                Cond::Gt {
                    namespace,
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            SBTokens::Gte => {
                Cond::Gte {
                    namespace,
                    lhs,
                    rhs: visitor.expect_node::<BitShift>(),
                }
            }
            _=> unreachable!(),
        }
    }
}
