use sb_compiler_parse_syntax::SBToken;

use super::{Value, Visitor};

#[derive(Debug)]
pub enum Unary<'input> {
    Plus {
        value: Value<'input>,
    },
    Minus {
        value: Value<'input>,
    },
    Value {
        value: Value<'input>
    },
}

impl<'input> From<Visitor<'input>> for Unary<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Unary::Value {
                value: visitor.expect_node::<Value>(),
            };
        }

        // 演算子付き
        match visitor.expect_leaf().0 {
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
