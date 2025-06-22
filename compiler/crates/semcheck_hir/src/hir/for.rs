use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_type::op::ty_equals;
use sb_compiler_type::r#type::{Bool, Primitive, Type, Void};
use sb_compiler_type::Typed;

use super::{Block, Expr, SemCheck, Dep};

#[derive(Debug)]
pub struct For<'src> {
    pub init: Expr<'src>,
    pub cond: Expr<'src>,
    pub incr: Expr<'src>,
    pub block: Block<'src>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::For<'src>> for For<'src> {
    async fn check0(ctx: Dep<'_, 'src>, r#fot: ast::For<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // 初期化式, 増分式, ブロックの意味解析
        let init = Expr::check(ctx, r#fot.init).await?;
        let incr = Expr::check(ctx, r#fot.incr).await?;
        let block = Block::check(ctx.clone(), r#fot.block).await?;

        // 条件式の意味解析 & 型チェック
        let cond = Expr::check(ctx, r#fot.cond).await?;
        ty_equals(cond.ty(), &Arc::new(Primitive(Bool)))?;

        // For 文の型は Void
        let ty = Arc::new(Primitive(Void));

        Ok(For { init, cond, incr, block, ty })
    }
}

impl Typed for For<'_> {
    fn ty(&self) -> &Arc<Type> {
        &self.ty
    }
}
