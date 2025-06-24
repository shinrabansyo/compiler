use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBToken;

use super::{Value, Visitor};

#[derive(Debug)]
pub enum Unary<'src> {
    Not {
        span: Span<'src>,
        value: Value<'src>,
    },
    Plus {
        span: Span<'src>,
        value: Value<'src>,
    },
    Minus {
        span: Span<'src>,
        value: Value<'src>,
    },
    Value {
        value: Value<'src>
    },
}

impl<'src> From<Visitor<'src>> for Unary<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Unary::Value {
                value: visitor.expect_node::<Value>(),
            };
        }

        // 演算子付き
        match visitor.expect_leaf().0 {
            SBToken::Not => {
                Unary::Not {
                    span: visitor.span(),
                    value: visitor.expect_node::<Value>(),
                }
            }
            SBToken::Plus => {
                Unary::Plus {
                    span: visitor.span(),
                    value: visitor.expect_node::<Value>(),
                }
            }
            SBToken::Minus => {
                Unary::Minus {
                    span: visitor.span(),
                    value: visitor.expect_node::<Value>(),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl<'src> Spanned<'src> for Unary<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Unary::Not { span, .. } => *span,
            Unary::Plus { span, .. } => *span,
            Unary::Minus { span, .. } => *span,
            Unary::Value { value } => value.span(),
        }
    }
}
