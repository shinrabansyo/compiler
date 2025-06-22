use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};
use sb_compiler_semcheck_impl_typedecl::TypeDeclChecker;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{SemCheck, Dep};

#[derive(Debug)]
pub struct ArgumentDef<'src> {
    pub var: Var<'src>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::ArgumentDef<'src>> for ArgumentDef<'src> {
    async fn check0(ctx: Dep<'_, 'src>, arg: ast::ArgumentDef<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // 型存在チェック
        let ty = TypeDeclChecker::find(&ctx.type_decl, &arg.ty.as_str()).await?;

        // 変数宣言
        let var = VarDeclChecker::register(
            &mut ctx.var_decl,
            &arg.ident,
            ty,
        )?;

        Ok(ArgumentDef { var })
    }
}

impl Typed for ArgumentDef<'_> {
    fn ty(&self) -> &Type {
        &self.var.ty
    }
}
