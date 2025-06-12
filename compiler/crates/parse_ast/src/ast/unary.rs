use sb_compiler_parse_syntax::SBToken;

use super::{Value, Visitor};

#[derive(Debug)]
pub enum Unary {
    Plus {
        value: Value,
    },
    Minus {
        value: Value,
    },
    Value {
        value: Value
    },
}

impl From<Visitor<'_>> for Unary {
    fn from(mut visitor: Visitor<'_>) -> Self {
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
