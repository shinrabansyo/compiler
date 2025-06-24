use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_typedecl::TypeDeclChecker;
use sb_compiler_type::op::ty_can_call;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{Value, SemCheck, Dep};

#[derive(Debug)]
pub struct Call<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub args: Vec<Value<'src>>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Call<'src>> for Call<'src> {
    async fn check0(ctx: Dep<'_, 'src>, call: ast::Call<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        // 実引数を順に意味解析
        let mut args = vec![];
        for arg in call.args {
            args.push(Value::check(ctx, arg).await?);
        }

        // 型チェック
        let fn_name = format!(".{}", call.ident.as_str());
        let fn_ty = TypeDeclChecker::find(&ctx.type_decl, &fn_name).await?;
        let arg_tys = args
            .iter()
            .map(|a| a.ty())
            .collect::<Vec<_>>();
        let ty = ty_can_call(&fn_ty, &arg_tys)?;

        Ok(Call {
            span: call.span,
            ident: call.ident,
            args,
            ty,
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
