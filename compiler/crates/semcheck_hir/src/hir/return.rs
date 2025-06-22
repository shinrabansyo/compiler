use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_type_decl::r#type::primitive::*;
use sb_compiler_semcheck_impl_type_decl::r#type::*;

use super::{Expr, SemCheck, Dep};

#[derive(Debug)]
pub struct Return<'src> {
    pub expr: Expr<'src>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Return<'src>> for Return<'src> {
    async fn check0(ctx: Dep<'_, 'src>, r#return: ast::Return<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Return {
            expr: Expr::check(ctx, r#return.expr).await?,
        })
    }

    fn ty(&self) -> &Type {
        &Primitive(Void)
    }
}
