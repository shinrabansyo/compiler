use sb_compiler_parse_ast as ast;

use super::{Assign, SemCheck, Dep};

#[derive(Debug)]
pub struct Expr<'src> {
    pub assign: Assign<'src>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Expr<'src>> for Expr<'src> {
    async fn check0(ctx: Dep<'_, 'src>, expr: ast::Expr<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Expr {
            assign: Assign::check(ctx, expr.assign).await?,
        })
    }
}
