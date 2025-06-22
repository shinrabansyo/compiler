use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_type::r#type::{Primitive, Type, Void};
use sb_compiler_type::Typed;

use super::{Expr, Block, SemCheck, Dep};

#[derive(Debug)]
pub struct While<'src> {
    pub cond: Expr<'src>,
    pub block: Block<'src>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::While<'src>> for While<'src> {
    async fn check0(ctx: Dep<'_, 'src>, r#while: ast::While<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(While {
            cond: Expr::check(ctx, r#while.cond).await?,
            block: Block::check(ctx.clone(), r#while.block).await?,
            ty: Arc::new(Primitive(Void)),
        })
    }
}

impl Typed for While<'_> {
    fn ty(&self) -> &Arc<Type> {
        &self.ty
    }
}
