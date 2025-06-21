use sb_compiler_parse_ast as ast;

use super::{Block, Expr, SemCheckFrom, Dep};

#[derive(Debug)]
pub struct For<'input> {
    pub init: Expr<'input>,
    pub cond: Expr<'input>,
    pub incr: Expr<'input>,
    pub block: Block<'input>,
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::For<'input>> for For<'input> {
    async fn check0(ctx: Dep<'_, 'input>, r#fot: ast::For<'input>) -> anyhow::Result<Self>
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
