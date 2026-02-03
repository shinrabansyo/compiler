use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::op::ty_arith2;
use sb_compiler_semcheck_impl_type::{Typed, Type};

use super::{Cast, SemCheck, Dep};

#[derive(Debug)]
pub enum Mul<'src> {
    Multiply {
        span: Span<'src>,
        lhs: Box<Mul<'src>>,
        rhs: Cast<'src>,
        ty: Arc<Type>,
    },
    Divide {
        span: Span<'src>,
        lhs: Box<Mul<'src>>,
        rhs: Cast<'src>,
        ty: Arc<Type>,
    },
    Modulo {
        span: Span<'src>,
        lhs: Box<Mul<'src>>,
        rhs: Cast<'src>,
        ty: Arc<Type>,
    },
    Cast {
        value: Cast<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Mul<'src>> for Mul<'src> {
    async fn check0(ctx: Dep<'_, 'src>, add: ast::Mul<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match add {
            ast::Mul::Multiply { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(Mul::check(ctx, *lhs).await?);
                let rhs = Cast::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_arith2(&lhs, &rhs)?;

                Ok(Mul::Multiply { span, lhs, rhs, ty })
            }
            ast::Mul::Divide { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(Mul::check(ctx, *lhs).await?);
                let rhs = Cast::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_arith2(&lhs, &rhs)?;

                Ok(Mul::Divide { span, lhs, rhs, ty })
            }
            ast::Mul::Modulo { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(Mul::check(ctx, *lhs).await?);
                let rhs = Cast::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_arith2(&lhs, &rhs)?;

                Ok(Mul::Modulo { span, lhs, rhs, ty })
            }
            ast::Mul::Cast { value } => {
                Ok(Mul::Cast {
                    value: Cast::check(ctx, value).await?,
                })
            }
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

impl Typed for Mul<'_> {
    fn ty(&self) -> Arc<Type> {
        match self {
            Mul::Multiply { ty, .. } => ty.ty(),
            Mul::Divide { ty, .. } => ty.ty(),
            Mul::Modulo { ty, .. } => ty.ty(),
            Mul::Cast { value } => value.ty(),
        }
    }
}
