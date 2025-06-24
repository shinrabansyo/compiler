use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::op::ty_infer2;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{Add, SemCheck, Dep};

#[derive(Debug)]
pub enum BitShift<'src> {
    L {
        span: Span<'src>,
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
        ty: Arc<Type>,
    },
    R {
        span: Span<'src>,
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
        ty: Arc<Type>,
    },
    Ra {
        span: Span<'src>,
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
        ty: Arc<Type>,
    },
    Add {
        add: Add<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::BitShift<'src>> for BitShift<'src> {
    async fn check0(ctx: Dep<'_, 'src>, shift: ast::BitShift<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match shift {
            ast::BitShift::L { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(BitShift::check(ctx, *lhs).await?);
                let rhs = Add::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(BitShift::L { span, lhs, rhs, ty })
            }
            ast::BitShift::R { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(BitShift::check(ctx, *lhs).await?);
                let rhs = Add::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(BitShift::R { span, lhs, rhs, ty })
            }
            ast::BitShift::Ra { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(BitShift::check(ctx, *lhs).await?);
                let rhs = Add::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(BitShift::Ra { span, lhs, rhs, ty })
            }
            ast::BitShift::Add { add } => {
                Ok(BitShift::Add {
                    add: Add::check(ctx, add).await?,
                })
            }
        }
    }
}

impl<'src> Spanned<'src> for BitShift<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            BitShift::L { span, .. } => *span,
            BitShift::R { span, .. } => *span,
            BitShift::Ra { span, .. } => *span,
            BitShift::Add { add } => add.span(),
        }
    }
}

impl Typed for BitShift<'_> {
    fn ty(&self) -> &Arc<Type> {
        match self {
            BitShift::L { ty, .. } => ty,
            BitShift::R { ty, .. } => ty,
            BitShift::Ra { ty, .. } => ty,
            BitShift::Add { add } => add.ty(),
        }
    }
}
