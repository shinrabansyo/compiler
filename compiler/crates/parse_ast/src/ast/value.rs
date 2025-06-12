use sb_compiler_parse_cst::Span;
use sb_compiler_parse_syntax::{SBToken, SBRule};

use super::{Expr, Call, Visitor};

#[derive(Debug)]
pub enum Value<'input> {
    Const {
        value: i32,
    },
    Var {
        name: Span<'input>,
    },
    Expr {
        expr: Box<Expr<'input>>,
    },
    Call {
        call: Call<'input>,
    }
}

impl<'input> From<Visitor<'input>> for Value<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        match visitor.peek() {
            // 定数
            (Some(SBToken::Num), None) => {
                Value::Const{
                    value: visitor.expect_leaf().1.as_str().parse().unwrap(),
                }
            }
            // 変数
            (Some(SBToken::Ident), None) => {
                Value::Var {
                    name: visitor.expect_leaf().1,
                }
            }
            // 括弧
            (None, Some(SBRule::Expr)) => {
                Value::Expr {
                    expr: Box::new(visitor.expect_node::<Expr>()),
                }
            }
            // 関数呼び出し
            (None, Some(SBRule::Call)) => {
                Value::Call {
                    call: visitor.expect_node::<Call>(),
                }
            }
            _ => unreachable!(),
        }
    }
}
