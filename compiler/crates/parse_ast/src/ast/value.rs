use sb_compiler_parse_syntax::{SBTokens, SBRules};

use super::{Expr, Call, Visitor};

#[derive(Debug)]
pub enum Value {
    Const {
        namespace: String,
        value: i32,
    },
    Var {
        namespace: String,
        name: String,
    },
    Expr {
        namespace: String,
        expr: Box<Expr>,
    },
    Call {
        namespace: String,
        call: Call,
    }
}

impl From<(String, Visitor<'_>)> for Value {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        match visitor.peek() {
            // 定数
            (Some(SBTokens::Num), None) => {
                Value::Const{
                    namespace,
                    value: visitor.expect_leaf().1.parse().unwrap(),
                }
            }
            // 変数
            (Some(SBTokens::Ident), None) => {
                Value::Var {
                    namespace,
                    name: visitor.expect_leaf().1.to_string(),
                }
            }
            // 括弧
            (None, Some(SBRules::Expr)) => {
                Value::Expr {
                    namespace,
                    expr: Box::new(visitor.expect_node::<Expr>()),
                }
            }
            // 関数呼び出し
            (None, Some(SBRules::Call)) => {
                Value::Call {
                    namespace,
                    call: visitor.expect_node::<Call>(),
                }
            }
            _ => unreachable!(),
        }
    }
}
