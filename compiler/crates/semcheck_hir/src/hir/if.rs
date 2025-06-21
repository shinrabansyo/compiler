use sb_compiler_parse_ast as ast;

use super::{Expr, Block, Stmt, SemCheck, Dep};

#[derive(Debug)]
pub struct If<'src> {
    pub cond: Expr<'src>,
    pub block: Block<'src>,
    pub else_stmt: Option<Box<Stmt<'src>>>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::If<'src>> for If<'src> {
    async fn check0(ctx: Dep<'_, 'src>, r#if: ast::If<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let cond = Expr::check(ctx, r#if.cond).await?;
        let block = Block::check(ctx.clone(), r#if.block).await?;
        let else_stmt = match r#if.else_stmt {
            Some(else_stmt) => Some(Box::new(Stmt::check(ctx,*else_stmt).await?)),
            None => None,
        };

        Ok(If { cond, block, else_stmt })
    }
}
