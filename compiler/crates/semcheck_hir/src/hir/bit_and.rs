use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::op::ty_det_arith2;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{Cond, SemCheck, Dep};

#[derive(Debug)]
pub enum BitAnd<'src> {
    And {
        span: Span<'src>,
        lhs: Box<BitAnd<'src>>,
        rhs: Cond<'src>,
        ty: Arc<Type>,
    },
    Cond {
        cond: Cond<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::BitAnd<'src>> for BitAnd<'src> {
    async fn check0(ctx: Dep<'_, 'src>, and: ast::BitAnd<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match and {
            ast::BitAnd::And { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(BitAnd::check(ctx, *lhs).await?);
                let rhs = Cond::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_det_arith2(&lhs, &rhs)?;

                Ok(BitAnd::And { span, lhs, rhs, ty })
            }
            ast::BitAnd::Cond { cond } => {
                Ok(BitAnd::Cond {
                    cond: Cond::check(ctx, cond).await?,
                })
            }
        }
    }
}

impl<'src> Spanned<'src> for BitAnd<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            BitAnd::And { span, .. } => *span,
            BitAnd::Cond { cond } => cond.span(),
        }
    }
}

impl Typed for BitAnd<'_> {
    fn ty(&self) -> Arc<Type> {
        match self {
            BitAnd::And { ty, .. } => ty.ty(),
            BitAnd::Cond { cond } => cond.ty(),
        }
    }
}
