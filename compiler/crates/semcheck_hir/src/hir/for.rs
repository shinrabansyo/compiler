use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::op::ty_equals;
use sb_compiler_type::r#type::{Bool, Primitive, Type, Void};
use sb_compiler_type::Typed;

use super::{Block, Expr, SemCheck, VarDecl, InDep};

#[derive(Debug)]
pub struct For<'src> {
    pub span: Span<'src>,
    pub init: VarDecl<'src>,
    pub cond: Expr<'src>,
    pub incr: Expr<'src>,
    pub block: Block<'src>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<InDep<'src>, ast::For<'src>> for For<'src> {
    async fn check0(mut ctx: InDep<'src>, r#for: ast::For<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // Span 取得
        let span = r#for.span;

        // 初期化文, 増分式, ブロックの意味解析
        let init = VarDecl::check(&mut ctx, r#for.init).await?;
        let incr = Expr::check(&mut ctx, r#for.incr).await?;
        let block = Block::check(ctx.clone(), r#for.block).await?;

        // 条件式の意味解析 & 型チェック
        let cond = Expr::check(&mut ctx, r#for.cond).await?;
        ty_equals(cond.ty(), &Arc::new(Primitive(Bool)))?;

        // For 文の型は Void
        let ty = Arc::new(Primitive(Void));

        Ok(For { span, init, cond, incr, block, ty })
    }
}

impl<'src> Spanned<'src> for For<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for For<'_> {
    fn ty(&self) -> &Arc<Type> {
        &self.ty
    }
}
