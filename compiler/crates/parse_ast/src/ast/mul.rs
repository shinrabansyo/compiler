use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBToken;

use super::{Cast, Visitor};

#[derive(Debug)]
pub enum Mul<'src> {
    Multiply {
        span: Span<'src>,
        lhs: Box<Mul<'src>>,
        rhs: Cast<'src>,
    },
    Divide {
        span: Span<'src>,
        lhs: Box<Mul<'src>>,
        rhs: Cast<'src>,
    },
    Modulo {
        span: Span<'src>,
        lhs: Box<Mul<'src>>,
        rhs: Cast<'src>,
    },
    Cast {
        value: Cast<'src>,
    },
}

impl<'src> From<Visitor<'src>> for Mul<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Mul::Cast {
                value: visitor.expect_node::<Cast>(),
            };
        }

        // 演算子付き
        let lhs = visitor.expect_node::<Mul>();
        match visitor.expect_leaf().0 {
            SBToken::Mul => {
                Mul::Multiply {
                    span: visitor.span(),
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Cast>(),
                }
            }
            SBToken::Div => {
                Mul::Divide {
                    span: visitor.span(),
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Cast>(),
                }
            }
            SBToken::Mod => {
                Mul::Modulo {
                    span: visitor.span(),
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Cast>(),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl<'src> Spanned<'src> for Mul<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Mul::Multiply { span, .. } => *span,
            Mul::Divide { span, .. } => *span,
            Mul::Modulo { span, .. } => *span,
            Mul::Cast { value } => value.span(),
        }
    }
}
