use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::op::ty_equals;
use sb_compiler_semcheck_impl_type::{Typed, Type, Bool, Void};

use super::{Expr, Block, SemCheck, Dep};

#[derive(Debug)]
pub struct While<'src> {
    pub span: Span<'src>,
    pub cond: Expr<'src>,
    pub block: Block<'src>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::While<'src>> for While<'src> {
    async fn check0(ctx: Dep<'_, 'src>, r#while: ast::While<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        // 状建設の意味解析 & 型チェック
        let cond = Expr::check(ctx, r#while.cond).await?;
        ty_equals(&Bool, &cond)?;

        // ブロックの意味解析
        let block = Block::check(ctx.clone(), r#while.block).await?;

        Ok(While { span: r#while.span, cond, block })
    }
}

impl<'src> Spanned<'src> for While<'src> {
    fn span(&self) -> sb_compiler_parse_cst::Span<'src> {
        self.span
    }
}

impl Typed for While<'_> {
    fn ty(&self) -> Arc<Type> {
        Void.ty()
    }
}
