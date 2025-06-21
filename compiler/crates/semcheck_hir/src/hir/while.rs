use sb_compiler_parse_ast as ast;

use super::{Expr, Block, SemCheckFrom, Dep};

#[derive(Debug)]
pub struct While<'input> {
    pub cond: Expr<'input>,
    pub block: Block<'input>,
}

impl<'input> SemCheckFrom<Dep<'_>, ast::While<'input>> for While<'input> {
    async fn check0(ctx: Dep<'_>, r#while: ast::While<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(While {
            cond: Expr::check(ctx, r#while.cond).await?,
            block: Block::check(ctx.clone(), r#while.block).await?,
        })
    }
}
