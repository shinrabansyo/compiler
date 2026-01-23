use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::decl::ty_link;
use sb_compiler_semcheck_impl_type::op::ty_cast;
use sb_compiler_semcheck_impl_type::{Typed, Type};

use super::{Unary, SemCheck, Dep};

#[derive(Debug)]
pub enum Cast<'src> {
    Casting {
        span: Span<'src>,
        unary: Unary<'src>,
        ty: Arc<Type>,
    },
    Unary {
        unary: Unary<'src>
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Cast<'src>> for Cast<'src> {
    async fn check0(ctx: Dep<'_, 'src>, unary: ast::Cast<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match unary {
            ast::Cast::Casting { span, unary, ty } => {
                // 式の意味解析
                let unary = Unary::check(ctx, unary).await?;

                // 型情報取得 (ast::Type -> semcheck_impl_type::Type)
                let ty = ty_link(&ctx.r#type, ty).await?;
                ty_cast(&unary, &ty)?;

                Ok(Cast::Casting { span, unary, ty })
            }
            ast::Cast::Unary { unary } => {
                Ok(Cast::Unary {
                    unary: Unary::check(ctx, unary).await?,
                })
            }
        }
    }
}

impl<'src> Spanned<'src> for Cast<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Cast::Casting { span, .. } => *span,
            Cast::Unary { unary } => unary.span(),
        }
    }
}

impl Typed for Cast<'_> {
    fn ty(&self) -> Arc<Type> {
        match self {
            Cast::Casting { ty, .. } => ty.ty(),
            Cast::Unary { unary } => unary.ty(),
        }
    }
}
