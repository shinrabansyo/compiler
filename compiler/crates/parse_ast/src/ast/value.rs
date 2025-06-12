use sb_compiler_parse_syntax::{SBTokens, SBRules};

use super::{Expr, Call, Visitor};

#[derive(Debug)]
pub enum Value {
    Const {
        value: i32,
    },
    Var {
        name: String,
    },
    Expr {
        expr: Box<Expr>,
    },
    Call {
        call: Call,
    }
}

impl From<Visitor<'_>> for Value {
    fn from(mut visitor: Visitor<'_>) -> Self {
        match visitor.peek() {
            // 定数
            (Some(SBTokens::Num), None) => {
                Value::Const{
                    value: visitor.expect_leaf().1.parse().unwrap(),
                }
            }
            // 変数
            (Some(SBTokens::Ident), None) => {
                Value::Var {
                    name: visitor.expect_leaf().1.to_string(),
                }
            }
            // 括弧
            (None, Some(SBRules::Expr)) => {
                Value::Expr {
                    expr: Box::new(visitor.expect_node::<Expr>()),
                }
            }
            // 関数呼び出し
            (None, Some(SBRules::Call)) => {
                Value::Call {
                    call: visitor.expect_node::<Call>(),
                }
            }
            _ => unreachable!(),
        }
    }
}
