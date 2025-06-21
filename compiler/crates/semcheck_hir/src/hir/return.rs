use sb_compiler_parse_ast as ast;

use super::{Expr, SemCheckFrom, Dep};

#[derive(Debug)]
pub struct Return<'input> {
    pub expr: Expr<'input>,
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::Return<'input>> for Return<'input> {
    async fn check0(ctx: Dep<'_, 'input>, r#return: ast::Return<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Return {
            expr: Expr::check(ctx, r#return.expr).await?,
        })
    }
}