use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBToken;

use super::{Mul, Visitor};

#[derive(Debug)]
pub enum Add<'src> {
    Plus {
        span: Span<'src>,
        lhs: Box<Add<'src>>,
        rhs: Mul<'src>,
    },
    Minus {
        span: Span<'src>,
        lhs: Box<Add<'src>>,
        rhs: Mul<'src>,
    },
    Mul {
        value: Mul<'src>,
    },
}

impl<'src> From<Visitor<'src>> for Add<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Add::Mul {
                value: visitor.expect_node::<Mul>(),
            };
        }

        // 演算子付き
        let lhs = visitor.expect_node::<Add>();
        match visitor.expect_leaf().0 {
            SBToken::Plus => {
                Add::Plus {
                    span: visitor.span(),
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Mul>(),
                }
            }
            SBToken::Minus => {
                Add::Minus {
                    span: visitor.span(),
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Mul>(),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl<'src> Spanned<'src> for Add<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Add::Plus { span, .. } => *span,
            Add::Minus { span, .. } => *span,
            Add::Mul { value } => value.span(),
        }
    }
}
