use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBToken;

use super::Visitor;

#[derive(Debug)]
pub enum Type<'src> {
    // プリミティブ
    Bool(Span<'src>),
    Char(Span<'src>),
    I8(Span<'src>),
    I16(Span<'src>),
    I32(Span<'src>),
}

impl<'src> From<Visitor<'src>> for Type<'src> {
    fn from(visitor: Visitor<'src>) -> Self {
        match visitor.peek() {
            // プリミティブ
            (Some(SBToken::BoolTy), None) => Type::Bool(visitor.span()),
            (Some(SBToken::CharTy), None) => Type::Char(visitor.span()),
            (Some(SBToken::I8Ty), None) => Type::I8(visitor.span()),
            (Some(SBToken::I16Ty), None) => Type::I16(visitor.span()),
            (Some(SBToken::I32Ty), None) => Type::I32(visitor.span()),

            _ => unreachable!(),
        }
    }
}

impl<'src> Spanned<'src> for Type<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Type::Bool(span) => *span,
            Type::Char(span) => *span,
            Type::I8(span) => *span,
            Type::I16(span) => *span,
            Type::I32(span) => *span,
        }
    }
}
