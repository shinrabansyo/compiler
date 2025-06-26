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
    pub name: String,
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
        let mod_name = ctx.name.as_str().split(".").collect::<Vec<_>>()[1];
        let fn_name = format!(".{}.{}", mod_name, call.ident.as_str());
        let fn_ty = TypeDeclChecker::find(&ctx.type_decl, &fn_name).await?;
        let ty = ty_can_call(&fn_ty, &args)?;

        Ok(Call {
            span: call.span,
            name: fn_name,
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
