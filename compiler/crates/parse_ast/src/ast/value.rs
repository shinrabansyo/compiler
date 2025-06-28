use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::{SBToken, SBRule};

use super::{Expr, Visitor};

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
    Call {
        span: Span<'src>,
        ident: Span<'src>,
        args: Vec<Value<'src>>,
    },
    Expr {
        expr: Box<Expr<'src>>,
    },
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
                let num_s = visitor.expect_leaf().1.as_str();
                let value = match num_s {
                    "0" => Ok(0),
                    _ if num_s.starts_with("0b") => i32::from_str_radix(&num_s[2..], 2),
                    _ if num_s.starts_with("0x") => i32::from_str_radix(&num_s[2..], 16),
                    _ if num_s.starts_with("0") => i32::from_str_radix(&num_s[1..], 8),
                    _ => num_s.parse::<i32>(),
                }.unwrap();
                Value::CNum { span: visitor.span(), value }
            }
            // 変数 or 関数呼び出し
            (Some(SBToken::Ident), None) => {
                let span = visitor.span();
                let ident = visitor.expect_leaf().1;
                match visitor.peek() {
                    // 変数
                    (None, None) => Value::Var {
                        span,
                        name: ident,
                    },
                    // 関数呼び出し
                    _ => Value::Call {
                        span,
                        ident,
                        args: visitor.expect_nodes::<Value>(),
                    },
                }
            }
            // 括弧
            (None, Some(SBRule::Expr)) => {
                Value::Expr {
                    expr: Box::new(visitor.expect_node::<Expr>()),
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
            Value::Call { span, .. } => *span,
            Value::Expr { expr } => expr.span(),
        }
    }
}
