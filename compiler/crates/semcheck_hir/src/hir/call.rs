use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::r#fn::ty_can_call;
use sb_compiler_semcheck_impl_type::{Typed, Type};

use super::{Expr, SemCheck, Dep};

#[derive(Debug)]
pub struct Call<'src> {
    pub span: Span<'src>,
    pub name: String,
    pub args: Vec<Expr<'src>>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Call<'src>> for Call<'src> {
    async fn check0(ctx: Dep<'_, 'src>, call: ast::Call<'src>) -> miette::Result<Self>
        where
            Self: Sized
    {
        // 関数名(フルパス)
        let fn_name = format!(".main.{}", call.ident.as_str());

        // 実引数を順に意味解析
        let mut checked_args = vec![];
        for arg in call.args {
            checked_args.push(Expr::check(ctx, arg).await?);
        }

        // 型チェック
        let fn_ty = ty_can_call(&ctx.r#type, call.ident, &checked_args).await?;

        Ok(Call {
            span: call.span,
            name: fn_name,
            args: checked_args,
            ty: fn_ty,
        })
    }
}

impl<'src> Spanned<'src> for Call<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for Call<'_> {
    fn ty(&self) -> Arc<Type> {
        self.ty.ty()
    }
}
