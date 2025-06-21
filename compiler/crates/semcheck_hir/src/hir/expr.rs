use sb_compiler_parse_ast as ast;

use super::{Assign, SemCheckFrom, Dep};

#[derive(Debug)]
pub struct Expr<'input> {
    pub assign: Assign<'input>,
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::Expr<'input>> for Expr<'input> {
    async fn check0(ctx: Dep<'_, 'input>, expr: ast::Expr<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Expr {
            assign: Assign::check(ctx, expr.assign).await?,
        })
    }
}
