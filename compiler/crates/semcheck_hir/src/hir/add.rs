use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::op::ty_infer2;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{Cast, SemCheck, Dep};

#[derive(Debug)]
pub enum Add<'src> {
    Plus {
        span: Span<'src>,
        lhs: Box<Add<'src>>,
        rhs: Cast<'src>,
        ty: Arc<Type>,
    },
    Minus {
        span: Span<'src>,
        lhs: Box<Add<'src>>,
        rhs: Cast<'src>,
        ty: Arc<Type>,
    },
    Cast {
        value: Cast<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Add<'src>> for Add<'src> {
    async fn check0(ctx: Dep<'_, 'src>, add: ast::Add<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match add {
            ast::Add::Plus { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(Add::check(ctx, *lhs).await?);
                let rhs = Cast::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(Add::Plus { span, lhs, rhs, ty })
            }
            ast::Add::Minus { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(Add::check(ctx, *lhs).await?);
                let rhs = Cast::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(Add::Minus { span, lhs, rhs, ty })
            }
            ast::Add::Cast { value } => {
                Ok(Add::Cast {
                    value: Cast::check(ctx, value).await?,
                })
            }
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

impl Typed for Add<'_> {
    fn ty(&self) -> &Arc<Type> {
        match self {
            Add::Plus { ty, .. } => ty,
            Add::Minus { ty, .. } => ty,
            Add::Cast { value } => value.ty(),
        }
    }
}
