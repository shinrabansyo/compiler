pub use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::decl::ty_find;
use sb_compiler_type::op::ty_can_return;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{Expr, SemCheck, Dep};

#[derive(Debug)]
pub struct Return<'src> {
    pub span: Span<'src>,
    pub expr: Expr<'src>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Return<'src>> for Return<'src> {
    async fn check0(ctx: Dep<'_, 'src>, r#return: ast::Return<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        // 式の意味解析
        let expr = Expr::check(ctx, r#return.expr).await?;

        // 戻り値の型をチェック
        let fn_ty = ty_find(&ctx.r#type, ctx.name.as_str().into()).await?;
        let fn_ret_ty = ty_can_return(&fn_ty, &expr)?;

        Ok(Return {
            span: r#return.span,
            expr,
            ty: fn_ret_ty,
        })
    }
}

impl<'src> Spanned<'src> for Return<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for Return<'_> {
    fn ty(&self) -> Arc<Type> {
        self.ty.ty()
    }
}
