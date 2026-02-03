use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBToken;

use super::Visitor;

#[derive(Debug)]
pub enum Type<'src> {
    // アドレス
    RawAddr {
        span: Span<'src>,
    },
    DataAddr {
        span: Span<'src>,
        inner_ty: Box<Type<'src>>
    },
    InstAddr {
        span: Span<'src>,
        inner_ty: Box<Type<'src>>
    },

    // ユーザ指定 or プリミティブ
    Term(Span<'src>),
}

impl<'src> From<Visitor<'src>> for Type<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        let span = visitor.span();
        match visitor.expect_leaf() {
            // アドレス
            (SBToken::RawAddrTy, _) => Type::RawAddr { span },
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

            // ユーザ指定 or プリミティブ
            (_, span) => Type::Term(span),
        }
    }
}

impl<'src> Spanned<'src> for Type<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            // アドレス
            Type::RawAddr { span, .. } => *span,
            Type::DataAddr { span, .. } => *span,
            Type::InstAddr { span, .. } => *span,

            // その他
            Type::Term(span) => *span,
        }
    }
}
