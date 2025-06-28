use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{SemCheck, Dep};

#[derive(Debug)]
pub struct ArgumentDef<'src> {
    pub span: Span<'src>,
    pub var: Var<'src>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::ArgumentDef<'src>> for ArgumentDef<'src> {
    async fn check0(ctx: Dep<'_, 'src>, arg: ast::ArgumentDef<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        // 型存在チェック
        let ty = Type::from(arg.ty).ty();

        // 変数宣言
        let var = VarDeclChecker::register(
            &mut ctx.var_decl,
            &arg.ident,
            ty,
        )?;

        Ok(ArgumentDef { span: arg.span, var })
    }
}

impl<'src> Spanned<'src> for ArgumentDef<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for ArgumentDef<'_> {
    fn ty(&self) -> Arc<Type> {
        self.var.ty.ty()
    }
}
