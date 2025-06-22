use sb_compiler_parse_syntax::SBToken;

use super::{Value, Visitor};

#[derive(Debug)]
pub enum Unary<'src> {
    Not {
        value: Value<'src>,
    },
    Plus {
        value: Value<'src>,
    },
    Minus {
        value: Value<'src>,
    },
    Value {
        value: Value<'src>
    },
}

impl<'src> From<Visitor<'src>> for Unary<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Unary::Value {
                value: visitor.expect_node::<Value>(),
            };
        }

        // 演算子付き
        match visitor.expect_leaf().0 {
            SBToken::Not => {
                Unary::Not {
                    value: visitor.expect_node::<Value>(),
                }
            }
            SBToken::Plus => {
                Unary::Plus {
                    value: visitor.expect_node::<Value>(),
                }
            }
            SBToken::Minus => {
                Unary::Minus {
                    value: visitor.expect_node::<Value>(),
                }
            }
            _ => unreachable!(),
        }
    }
}
