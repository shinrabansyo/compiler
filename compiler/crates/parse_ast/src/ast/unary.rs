use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBToken;

use super::{Type, ValueR, Visitor};

#[derive(Debug)]
pub enum Unary<'src> {
    Not {
        span: Span<'src>,
        value: ValueR<'src>,
    },
    Plus {
        span: Span<'src>,
        value: ValueR<'src>,
    },
    Minus {
        span: Span<'src>,
        value: ValueR<'src>,
    },
    Addr {
        span: Span<'src>,
        value: ValueR<'src>,
    },
    SizeOf {
        span: Span<'src>,
        ty: Type<'src>,
    },
    ValueR {
        value: ValueR<'src>
    },
}

impl<'src> From<Visitor<'src>> for Unary<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Unary::ValueR {
                value: visitor.expect_node::<ValueR>(),
            };
        }

        // 演算子付き
        match visitor.expect_leaf().0 {
            SBToken::Not => {
                Unary::Not {
                    span: visitor.span(),
                    value: visitor.expect_node::<ValueR>(),
                }
            }
            SBToken::Plus => {
                Unary::Plus {
                    span: visitor.span(),
                    value: visitor.expect_node::<ValueR>(),
                }
            }
            SBToken::Minus => {
                Unary::Minus {
                    span: visitor.span(),
                    value: visitor.expect_node::<ValueR>(),
                }
            }
            SBToken::BitAnd => {
                Unary::Addr {
                    span: visitor.span(),
                    value: visitor.expect_node::<ValueR>(),
                }
            }
            SBToken::SizeOf => {
                Unary::SizeOf {
                    span: visitor.span(),
                    ty: visitor.expect_node::<Type>(),
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
            Unary::Addr { span, .. } => *span,
            Unary::SizeOf { span, .. } => *span,
            Unary::ValueR { value } => value.span(),
        }
    }
}
