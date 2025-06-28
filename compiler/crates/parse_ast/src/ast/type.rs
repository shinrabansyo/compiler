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

    // アドレス
    Addr {
        span: Span<'src>,
        inner_ty: Box<Type<'src>>
    },
    DataAddr {
        span: Span<'src>,
        inner_ty: Box<Type<'src>>
    },
    InstAddr {
        span: Span<'src>,
        inner_ty: Box<Type<'src>>
    },
}

impl<'src> From<Visitor<'src>> for Type<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        let span = visitor.span();
        match visitor.expect_leaf() {
            // プリミティブ
            (SBToken::BoolTy, _) => Type::Bool(span),
            (SBToken::CharTy, _) => Type::Char(span),
            (SBToken::I8Ty, _) => Type::I8(span),
            (SBToken::I16Ty, _) => Type::I16(span),
            (SBToken::I32Ty, _) => Type::I32(span),

            // アドレス
            (SBToken::AddrTy, _) => {
                let _ = visitor.expect_leaf(); // '<'
                let inner_ty = Box::new(visitor.expect_node::<Type>());
                let _ = visitor.expect_leaf(); // '>'
                Type::Addr { span, inner_ty }
            }
            (SBToken::DataAddrTy, _) => {
                let _ = visitor.expect_leaf(); // '<'
                let inner_ty = Box::new(visitor.expect_node::<Type>());
                let _ = visitor.expect_leaf(); // '>'
                Type::DataAddr { span, inner_ty }
            }
            (SBToken::InstAddrTy, _) => {
                let _ = visitor.expect_leaf(); // '<'
                let inner_ty = Box::new(visitor.expect_node::<Type>());
                let _ = visitor.expect_leaf(); // '>'
                Type::InstAddr { span, inner_ty }
            }

            _ => unreachable!(),
        }
    }
}

impl<'src> Spanned<'src> for Type<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            // プリミティブ
            Type::Bool(span) => *span,
            Type::Char(span) => *span,
            Type::I8(span) => *span,
            Type::I16(span) => *span,
            Type::I32(span) => *span,

            // アドレス
            Type::Addr { span, .. } => *span,
            Type::DataAddr { span, .. } => *span,
            Type::InstAddr { span, .. } => *span,
        }
    }
}
