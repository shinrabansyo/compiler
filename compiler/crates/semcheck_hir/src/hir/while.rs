use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::op::ty_equals;
use sb_compiler_type::r#type::{Bool, Primitive, Type, Void};
use sb_compiler_type::Typed;

use super::{Expr, Block, SemCheck, Dep};

#[derive(Debug)]
pub struct While<'src> {
    pub span: Span<'src>,
    pub cond: Expr<'src>,
    pub block: Block<'src>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::While<'src>> for While<'src> {
    async fn check0(ctx: Dep<'_, 'src>, r#while: ast::While<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // 状建設の意味解析 & 型チェック
        let cond = Expr::check(ctx, r#while.cond).await?;
        ty_equals(cond.ty(), &Arc::new(Primitive(Bool)))?;

        // ブロックの意味解析
        let block = Block::check(ctx.clone(), r#while.block).await?;

        // While 文の型は Void
        let ty = Arc::new(Primitive(Void));

        Ok(While { span: r#while.span, cond, block, ty })
    }
}

impl<'src> Spanned<'src> for While<'src> {
    fn span(&self) -> sb_compiler_parse_cst::Span<'src> {
        self.span
    }
}

impl Typed for While<'_> {
    fn ty(&self) -> &Arc<Type> {
        &self.ty
    }
}
