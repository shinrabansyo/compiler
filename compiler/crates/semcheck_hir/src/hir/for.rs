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
        })
    }
}

impl Typed for For<'_> {
    fn ty(&self) -> &Type {
        &Primitive(Void)
    }
}
