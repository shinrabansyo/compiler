use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::op::ty_det_arith2;
use sb_compiler_semcheck_impl_type::{Typed, Type};

use super::{BitAnd, SemCheck, Dep};

#[derive(Debug)]
pub enum BitXor<'src> {
    Xor {
        span: Span<'src>,
        lhs: Box<BitXor<'src>>,
        rhs: BitAnd<'src>,
        ty: Arc<Type>,
    },
    BitAnd {
        and: BitAnd<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::BitXor<'src>> for BitXor<'src> {
    async fn check0(ctx: Dep<'_, 'src>, xor: ast::BitXor<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match xor {
            ast::BitXor::Xor { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(BitXor::check(ctx, *lhs).await?);
                let rhs = BitAnd::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_det_arith2(&lhs, &rhs)?;

                Ok(BitXor::Xor { span, lhs, rhs, ty })
            }
            ast::BitXor::BitAnd { and } => {
                Ok(BitXor::BitAnd {
                    and: BitAnd::check(ctx, and).await?,
                })
            }
        }
    }
}

impl<'src> Spanned<'src> for BitXor<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            BitXor::Xor { span, .. } => *span,
            BitXor::BitAnd { and } => and.span(),
        }
    }
}

impl Typed for BitXor<'_> {
    fn ty(&self) -> Arc<Type> {
        match self {
            BitXor::Xor { ty, .. } => ty.ty(),
            BitXor::BitAnd { and } => and.ty(),
        }
    }
}
