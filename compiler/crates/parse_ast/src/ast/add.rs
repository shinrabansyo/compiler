use sb_compiler_parse_syntax::SBTokens;

use super::{Unary, Visitor};

#[derive(Debug)]
pub enum Add {
    Plus {
        namespace: String,
        lhs: Box<Add>,
        rhs: Unary,
    },
    Minus {
        namespace: String,
        lhs: Box<Add>,
        rhs: Unary,
    },
    Unary {
        namespace: String,
        value: Unary
    },
}

impl From<(String, Visitor<'_>)> for Add {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Add::Unary {
                namespace,
                value: visitor.expect_node::<Unary>(),
            };
        }

        // 演算子付き
        let lhs = visitor.expect_node::<Add>();
        match visitor.expect_leaf().0 {
            SBTokens::Plus => {
                Add::Plus {
                    namespace,
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Unary>(),
                }
            }
            SBTokens::Minus => {
                Add::Minus {
                    namespace,
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Unary>(),
                }
            }
            _ => unreachable!(),
        }
    }
}
