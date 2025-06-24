use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBToken;

use super::{Cast, Visitor};

#[derive(Debug)]
pub enum Add<'src> {
    Plus {
        span: Span<'src>,
        lhs: Box<Add<'src>>,
        rhs: Cast<'src>,
    },
    Minus {
        span: Span<'src>,
        lhs: Box<Add<'src>>,
        rhs: Cast<'src>,
    },
    Cast {
        value: Cast<'src>,
    },
}

impl<'src> From<Visitor<'src>> for Add<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Add::Cast {
                value: visitor.expect_node::<Cast>(),
            };
        }

        // 演算子付き
        let lhs = visitor.expect_node::<Add>();
        match visitor.expect_leaf().0 {
            SBToken::Plus => {
                Add::Plus {
                    span: visitor.span(),
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Cast>(),
                }
            }
            SBToken::Minus => {
                Add::Minus {
                    span: visitor.span(),
                    lhs: Box::new(lhs),
                    rhs: visitor.expect_node::<Cast>(),
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
            Add::Cast { value } => value.span(),
        }
    }
}
