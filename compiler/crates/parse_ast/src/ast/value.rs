use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::{SBToken, SBRule};

use super::{Expr, Call, Visitor};

#[derive(Debug)]
pub enum Value<'src> {
    CBool {
        span: Span<'src>,
        value: bool,
    },
    CChar {
        span: Span<'src>,
        value: char,
    },
    CNum {
        span: Span<'src>,
        value: i32,
    },
    Var {
        span: Span<'src>,
        name: Span<'src>,
    },
    Expr {
        expr: Box<Expr<'src>>,
    },
    Call {
        call: Call<'src>,
    }
}

impl<'src> From<Visitor<'src>> for Value<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        match visitor.peek() {
            // 論理値
            (Some(SBToken::True), None) => {
                Value::CBool {
                    span: visitor.span(),
                    value: true,
                }
            }
            (Some(SBToken::False), None) => {
                Value::CBool {
                    span: visitor.span(),
                    value: false,
                }
            }
            // 文字
            (Some(SBToken::Char), None) => {
                let c = visitor.expect_leaf().1.as_str().trim_matches('\'');
                let value = match c {
                    "\\n" => '\n',
                    "\\t" => '\t',
                    c => c.chars().next().unwrap(),
                };
                Value::CChar { span: visitor.span(), value }
            }
            // 定数
            (Some(SBToken::Num), None) => {
                Value::CNum {
                    span: visitor.span(),
                    value: visitor.expect_leaf().1.as_str().parse().unwrap(),
                }
            }
            // 変数
            (Some(SBToken::Ident), None) => {
                Value::Var {
                    span: visitor.span(),
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

impl<'src> Spanned<'src> for Value<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Value::CBool { span, .. } => *span,
            Value::CChar { span, .. } => *span,
            Value::CNum { span, .. } => *span,
            Value::Var { span, .. } => *span,
            Value::Expr { expr } => expr.span(),
            Value::Call { call } => call.span(),
        }
    }
}
