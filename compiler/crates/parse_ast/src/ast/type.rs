use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBToken;

use super::Visitor;

#[derive(Debug)]
pub enum Type<'src> {
    // アドレス
    DataAddr {
        span: Span<'src>,
        inner_ty: Option<Box<Type<'src>>>,
    },
    InstAddr {
        span: Span<'src>,
        inner_ty: Option<Box<Type<'src>>>,
    },

    // ユーザ指定 or プリミティブ
    Term(Span<'src>),
}

impl<'src> From<Visitor<'src>> for Type<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        let span = visitor.span();
        match (visitor.expect_leaf(), visitor.peek().0) {
            // アドレス
            ((SBToken::DataAddrTy, _), None) => {
                Type::DataAddr { span, inner_ty: None }
            }
            ((SBToken::DataAddrTy, _), Some(_)) => {
                let _ = visitor.expect_leaf(); // '<'
                let inner_ty = Some(Box::new(visitor.expect_node::<Type>()));
                let _ = visitor.expect_leaf(); // '>'
                Type::DataAddr { span, inner_ty }
            }
            ((SBToken::InstAddrTy, _), None) => {
                Type::InstAddr { span, inner_ty: None }
            }
            ((SBToken::InstAddrTy, _), Some(_)) => {
                let _ = visitor.expect_leaf(); // '<'
                let inner_ty = Some(Box::new(visitor.expect_node::<Type>()));
                let _ = visitor.expect_leaf(); // '>'
                Type::InstAddr { span, inner_ty }
            }

            // ユーザ指定 or プリミティブ
            ((_, span), _) => Type::Term(span),
        }
    }
}

impl<'src> Spanned<'src> for Type<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            // アドレス
            Type::DataAddr { span, .. } => *span,
            Type::InstAddr { span, .. } => *span,

            // ユーザ指定 or プリミティブ
            Type::Term(span) => *span,
        }
    }
}
