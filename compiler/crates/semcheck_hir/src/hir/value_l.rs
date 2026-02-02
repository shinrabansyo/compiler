use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_var::decl::var_find;
use sb_compiler_semcheck_impl_var::Var;
use sb_compiler_semcheck_impl_type::{Typed, Type};

use super::{StructAccess, SemCheck, Dep};

#[derive(Debug)]
pub enum ValueL<'src> {
    Var {
        span: Span<'src>,
        var: Var<'src>,
    },
    StructAccess {
        struct_access: StructAccess<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::ValueL<'src>> for ValueL<'src> {
    async fn check0(ctx: Dep<'_, 'src>, value: ast::ValueL<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match value {
            ast::ValueL::Var { span } => {
                Ok(ValueL::Var {
                    span,
                    var: var_find(&mut ctx.var, &span).await?,
                })
            }
            ast::ValueL::StructAccess { struct_access } => {
                Ok(ValueL::StructAccess {
                    struct_access: StructAccess::check(ctx, struct_access).await?,
                })
            }
        }
    }
}

impl<'src> Spanned<'src> for ValueL<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            ValueL::Var { span, .. } => *span,
            ValueL::StructAccess { struct_access } => struct_access.span(),
        }
    }
}

impl Typed for ValueL<'_> {
    fn ty(&self) -> Arc<Type> {
        match self {
            ValueL::Var { var, .. } => var.ty(),
            ValueL::StructAccess { struct_access } => struct_access.ty(),
        }
    }
}
