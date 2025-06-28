use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::op::ty_det_arith2;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{BitXor, SemCheck, Dep};

#[derive(Debug)]
pub enum BitOr<'src> {
    Or {
        span: Span<'src>,
        lhs: Box<BitOr<'src>>,
        rhs: BitXor<'src>,
        ty: Arc<Type>,
    },
    BitXor {
        xor: BitXor<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::BitOr<'src>> for BitOr<'src> {
    async fn check0(ctx: Dep<'_, 'src>, or: ast::BitOr<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match or {
            ast::BitOr::Or { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(BitOr::check(ctx, *lhs).await?);
                let rhs = BitXor::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_det_arith2(&lhs, &rhs)?;

                Ok(BitOr::Or { span, lhs, rhs, ty })
            }
            ast::BitOr::BitXor { xor } => {
                Ok(BitOr::BitXor {
                    xor: BitXor::check(ctx, xor).await?,
                })
            }
        }
    }
}

impl<'src> Spanned<'src> for BitOr<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            BitOr::Or { span, .. } => *span,
            BitOr::BitXor { xor } => xor.span(),
        }
    }
}

impl Typed for BitOr<'_> {
    fn ty(&self) -> Arc<Type> {
        match self {
            BitOr::Or { ty, .. } => ty.ty(),
            BitOr::BitXor { xor } => xor.ty(),
        }
    }
}
