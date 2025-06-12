use sb_compiler_parse_syntax::SBTokens;

use super::{Unary, Visitor};

#[derive(Debug)]
pub enum Add {
    Plus {
        lhs: Box<Add>,
        rhs: Unary,
    },
    Minus {
        lhs: Box<Add>,
        rhs: Unary,
    },
    Unary {
        value: Unary
    },
}

impl From<Visitor<'_>> for Add {
    fn from(mut visitor: Visitor<'_>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Add::Unary {
                value: visitor.expect_node::<Unary>(),
            };
        }

        // 演算子付き
        let lhs = visitor.expect_node::<Add>();
        match visitor.expect_leaf().0 {
            SBTokens::Plus => {
                Add::Plus {
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Unary>(),
                }
            }
            SBTokens::Minus => {
                Add::Minus {
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Unary>(),
                }
            }
            _ => unreachable!(),
        }
    }
}
