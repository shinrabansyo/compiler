use sb_compiler_parse_syntax::SBToken;

use super::{Cast, Visitor};

#[derive(Debug)]
pub enum Add<'src> {
    Plus {
        lhs: Box<Add<'src>>,
        rhs: Cast<'src>,
    },
    Minus {
        lhs: Box<Add<'src>>,
        rhs: Cast<'src>,
    },
    Cast {
        value: Cast<'src>,
    },
}

impl<'src> From<Visitor<'src>> for Add<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Add::Cast {
                value: visitor.expect_node::<Cast>(),
            };
        }

        // 演算子付き
        let lhs = visitor.expect_node::<Add>();
        match visitor.expect_leaf().0 {
            SBToken::Plus => {
                Add::Plus {
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Cast>(),
                }
            }
            SBToken::Minus => {
                Add::Minus {
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Cast>(),
                }
            }
            _ => unreachable!(),
        }
    }
}
