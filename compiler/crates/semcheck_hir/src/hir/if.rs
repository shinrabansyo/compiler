use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::op::ty_equals;
use sb_compiler_semcheck_impl_type::{Typed, Type, Bool, Void};

use super::{Expr, Block, Stmt, SemCheck, Dep};

#[derive(Debug)]
pub struct If<'src> {
    pub span: Span<'src>,
    pub cond: Expr<'src>,
    pub block: Block<'src>,
    pub else_stmt: Option<Box<Stmt<'src>>>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::If<'src>> for If<'src> {
    async fn check0(ctx: Dep<'_, 'src>, r#if: ast::If<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        // 条件式の意味解析 & 型チェック
        let cond = Expr::check(ctx, r#if.cond).await?;
        ty_equals(Bool, &cond)?;

        // ブロックの意味解析
        let block = Block::check(ctx.clone(), r#if.block).await?;

        // else 節があれば意味解析
        let else_stmt = match r#if.else_stmt {
            Some(else_stmt) => Some(Box::new(Stmt::check(ctx,*else_stmt).await?)),
            None => None,
        };

        Ok(If { span: r#if.span, cond, block, else_stmt })
    }
}

impl<'src> Spanned<'src> for If<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for If<'_> {
    fn ty(&self) -> Arc<Type> {
        Void.ty()
    }
}
