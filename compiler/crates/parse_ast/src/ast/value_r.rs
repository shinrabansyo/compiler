use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::{SBToken, SBRule};
use sb_compiler_utils::primitive::i32;

use super::{Call, Expr, StructAccess, StructInit, Visitor};

#[derive(Debug)]
pub enum ValueR<'src> {
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
    },
    StructInit {
        struct_init: StructInit<'src>,
    },
    StructAccess {
        struct_access: StructAccess<'src>,
    },
}

impl<'src> From<Visitor<'src>> for ValueR<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        match visitor.peek() {
            // 論理値
            (Some(SBToken::True), None) => {
                ValueR::CBool {
                    span: visitor.span(),
                    value: true,
                }
            }
            (Some(SBToken::False), None) => {
                ValueR::CBool {
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
                ValueR::CChar { span: visitor.span(), value }
            }
            // 定数
            (Some(SBToken::Num), None) => {
                let num_s = visitor.expect_leaf().1.as_str();
                let value = i32::from_str(num_s).unwrap();
                ValueR::CNum { span: visitor.span(), value }
            }
            // 変数
            (Some(SBToken::Ident), None) => {
                ValueR::Var {
                    span: visitor.span(),
                    name: visitor.expect_leaf().1,
                }
            }
            // 括弧付きの式
            (None, Some(SBRule::Expr)) => {
                ValueR::Expr {
                    expr: Box::new(visitor.expect_node::<Expr>()),
                }
            }
            // 関数呼び出し
            (None, Some(SBRule::Call)) => {
                ValueR::Call {
                    call: visitor.expect_node::<Call>(),
                }
            }
            // 構造体
            (None, Some(SBRule::StructInit)) => {
                ValueR::StructInit {
                    struct_init: visitor.expect_node::<StructInit>(),
                }
            }
            (None, Some(SBRule::StructAccess)) => {
                ValueR::StructAccess {
                    struct_access: visitor.expect_node::<StructAccess>(),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl<'src> Spanned<'src> for ValueR<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            ValueR::CBool { span, .. } => *span,
            ValueR::CChar { span, .. } => *span,
            ValueR::CNum { span, .. } => *span,
            ValueR::Var { span, .. } => *span,
            ValueR::Expr { expr } => expr.span(),
            ValueR::Call { call } => call.span(),
            ValueR::StructInit { struct_init } => struct_init.span(),
            ValueR::StructAccess { struct_access } => struct_access.span(),
        }
    }
}
