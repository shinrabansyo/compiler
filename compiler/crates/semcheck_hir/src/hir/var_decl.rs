use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};
use sb_compiler_semcheck_impl_type_decl::TypeDeclChecker;
use sb_compiler_type::{Primitive, Type, Typed, Void};

use super::{Expr, SemCheck, Dep};

#[derive(Debug)]
pub struct VarDecl<'src> {
    pub var: Var<'src>,
    pub var_ty: Type,
    pub expr: Expr<'src>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::VarDecl<'src>> for VarDecl<'src> {
    async fn check0(ctx: Dep<'_, 'src>, var_decl: ast::VarDecl<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // 式の意味解析
        let expr = Expr::check(ctx, var_decl.expr).await?;

        // 型検査
        let var_ty = TypeDeclChecker::find(
            &ctx.type_decl,
            &var_decl.ty.as_str()
        ).await?;

        // 変数宣言
        let var = VarDeclChecker::register(
            &mut ctx.var_decl,
            &var_decl.ident,
            var_ty
        )?;

        Ok(VarDecl {
            var,
            var_ty,
            expr,
        })
    }
}

impl Typed for VarDecl<'_> {
    fn ty(&self) -> &Type {
        &Primitive(Void)
    }
}
