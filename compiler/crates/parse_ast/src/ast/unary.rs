use sb_compiler_parse_syntax::SBTokens;

use super::{Value, Visitor};

#[derive(Debug)]
pub enum Unary {
    Plus {
        namespace: String,
        value: Value,
    },
    Minus {
        namespace: String,
        value: Value,
    },
    Value {
        namespace: String,
        value: Value
    },
}

impl From<(String, Visitor<'_>)> for Unary {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Unary::Value {
                namespace,
                value: visitor.expect_node::<Value>(),
            };
        }

        // 演算子付き
        match visitor.expect_leaf().0 {
            SBTokens::Plus => {
                Unary::Plus {
                    namespace,
                    value: visitor.expect_node::<Value>(),
                }
            }
            SBTokens::Minus => {
                Unary::Minus {
                    namespace,
                    value: visitor.expect_node::<Value>(),
                }
            }
            _ => unreachable!(),
        }
    }
}
