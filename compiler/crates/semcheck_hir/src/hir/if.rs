use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_type::r#type::{Primitive, Type, Void};
use sb_compiler_type::Typed;

use super::{Expr, Block, Stmt, SemCheck, Dep};

#[derive(Debug)]
pub struct If<'src> {
    pub cond: Expr<'src>,
    pub block: Block<'src>,
    pub else_stmt: Option<Box<Stmt<'src>>>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::If<'src>> for If<'src> {
    async fn check0(ctx: Dep<'_, 'src>, r#if: ast::If<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // 条件式とブロックの意味解析
        let cond = Expr::check(ctx, r#if.cond).await?;
        let block = Block::check(ctx.clone(), r#if.block).await?;

        // else 節があれば意味解析
        let else_stmt = match r#if.else_stmt {
            Some(else_stmt) => Some(Box::new(Stmt::check(ctx,*else_stmt).await?)),
            None => None,
        };

        // If 文の型は Void
        let ty = Arc::new(Primitive(Void));

        Ok(If { cond, block, else_stmt, ty })
    }
}

impl Typed for If<'_> {
    fn ty(&self) -> &Arc<Type> {
        &self.ty
    }
}
