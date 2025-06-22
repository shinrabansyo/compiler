use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_type::r#type::{Primitive, Type, Void};
use sb_compiler_type::Typed;

use super::{Block, Expr, SemCheck, Dep};

#[derive(Debug)]
pub struct For<'src> {
    pub init: Expr<'src>,
    pub cond: Expr<'src>,
    pub incr: Expr<'src>,
    pub block: Block<'src>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::For<'src>> for For<'src> {
    async fn check0(ctx: Dep<'_, 'src>, r#fot: ast::For<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(For {
            init: Expr::check(ctx, r#fot.init).await?,
            cond: Expr::check(ctx, r#fot.cond).await?,
            incr: Expr::check(ctx, r#fot.incr).await?,
            block: Block::check(ctx.clone(), r#fot.block).await?,
            ty: Arc::new(Primitive(Void)),
        })
    }
}

impl Typed for For<'_> {
    fn ty(&self) -> &Arc<Type> {
        &self.ty
    }
}
