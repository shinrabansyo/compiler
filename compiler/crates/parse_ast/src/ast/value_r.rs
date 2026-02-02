use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::{SBToken, SBRule};
use sb_compiler_utils::primitive::i32;

use super::{Expr, StructAccess, StructInit, Visitor};

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
    Call {
        span: Span<'src>,
        ident: Span<'src>,
        args: Vec<Expr<'src>>,
    },
    Expr {
        expr: Box<Expr<'src>>,
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
            // 変数 or 関数呼び出し
            (Some(SBToken::Ident), None) => {
                let span = visitor.span();
                let ident = visitor.expect_leaf().1;
                match visitor.peek() {
                    // 変数
                    (None, None) => ValueR::Var {
                        span,
                        name: ident,
                    },
                    // 関数呼び出し
                    _ => ValueR::Call {
                        span,
                        ident,
                        args: visitor.expect_nodes::<Expr>(),
                    },
                }
            }
            // 括弧
            (None, Some(SBRule::Expr)) => {
                ValueR::Expr {
                    expr: Box::new(visitor.expect_node::<Expr>()),
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
            ValueR::Call { span, .. } => *span,
            ValueR::Expr { expr } => expr.span(),
            ValueR::StructInit { struct_init } => struct_init.span(),
            ValueR::StructAccess { struct_access } => struct_access.span(),
        }
    }
}
