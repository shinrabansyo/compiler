use sb_compiler_parse_syntax::SBToken;

use super::{Unary, Visitor};

#[derive(Debug)]
pub enum Add<'src> {
    Plus {
        lhs: Box<Add<'src>>,
        rhs: Unary<'src>,
    },
    Minus {
        lhs: Box<Add<'src>>,
        rhs: Unary<'src>,
    },
    Unary {
        value: Unary<'src>,
    },
}

impl<'src> From<Visitor<'src>> for Add<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
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
