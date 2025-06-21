use sb_compiler_parse_ast as ast;

use super::{Expr, Block, Stmt, SemCheckFrom, Dep};

#[derive(Debug)]
pub struct If<'input> {
    pub cond: Expr<'input>,
    pub block: Block<'input>,
    pub else_stmt: Option<Box<Stmt<'input>>>,
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::If<'input>> for If<'input> {
    async fn check0(ctx: Dep<'_, 'input>, r#if: ast::If<'input>) -> anyhow::Result<Self>
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
