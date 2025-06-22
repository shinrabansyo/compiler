pub use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_type::r#type::{Primitive, Type, Void};
use sb_compiler_type::Typed;

use super::{Expr, SemCheck, Dep};

#[derive(Debug)]
pub struct Return<'src> {
    pub expr: Expr<'src>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Return<'src>> for Return<'src> {
    async fn check0(ctx: Dep<'_, 'src>, r#return: ast::Return<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Return {
            expr: Expr::check(ctx, r#return.expr).await?,
            ty: Arc::new(Primitive(Void)),
        })
    }
}

impl Typed for Return<'_> {
    fn ty(&self) -> &Arc<Type> {
        &self.ty
    }
}
