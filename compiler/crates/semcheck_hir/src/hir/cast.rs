use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_impl_typedecl::TypeDeclChecker;
use sb_compiler_type::op::ty_cast;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

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
    async fn check0(ctx: Dep<'_, 'src>, unary: ast::Cast<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match unary {
            ast::Cast::Casting { span, unary, ty } => {
                // 式の意味解析
                let unary = Unary::check(ctx, unary).await?;

                // 型チェック
                let ty = TypeDeclChecker::find(&ctx.type_decl, ty.as_str()).await?;
                ty_cast(unary.ty(), &ty)?;

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

impl Typed for Cast<'_> {
    fn ty(&self) -> &Arc<Type> {
        match self {
            Cast::Casting { ty, .. } => ty,
            Cast::Unary { unary } => unary.ty(),
        }
    }
}
