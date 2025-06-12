use sb_compiler_parse_syntax::SBToken;

use super::{Unary, Visitor};

#[derive(Debug)]
pub enum Add<'input> {
    Plus {
        lhs: Box<Add<'input>>,
        rhs: Unary<'input>,
    },
    Minus {
        lhs: Box<Add<'input>>,
        rhs: Unary<'input>,
    },
    Unary {
        value: Unary<'input>,
    },
}

impl<'input> From<Visitor<'input>> for Add<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Add::Unary {
                value: visitor.expect_node::<Unary>(),
            };
        }

        // 演算子付き
        let lhs = visitor.expect_node::<Add>();
        match visitor.expect_leaf().0 {
            SBToken::Plus => {
                Add::Plus {
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Unary>(),
                }
            }
            SBToken::Minus => {
                Add::Minus {
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Unary>(),
                }
            }
            _ => unreachable!(),
        }
    }
}
