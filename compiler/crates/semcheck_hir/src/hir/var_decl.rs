use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};
use sb_compiler_semcheck_impl_typedecl::TypeDeclChecker;
use sb_compiler_type::op::{ty_equals, ty_infer};
use sb_compiler_type::r#type::{Primitive, Type, Void};
use sb_compiler_type::Typed;

use super::{Expr, SemCheck, Dep};

#[derive(Debug)]
pub struct VarDecl<'src> {
    pub span: Span<'src>,
    pub var: Var<'src>,
    pub var_ty: Arc<Type>,
    pub expr: Expr<'src>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::VarDecl<'src>> for VarDecl<'src> {
    async fn check0(ctx: Dep<'_, 'src>, var_decl: ast::VarDecl<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // 式の意味解析
        let expr = Expr::check(ctx, var_decl.expr).await?;

        // 型検査
        let var_ty = match &var_decl.ty {
            Some(ty) => {
                let var_ty = TypeDeclChecker::find(&ctx.type_decl, ty.as_str()).await?;
                ty_equals(&expr.ty(), &var_ty)?;
                var_ty
            }
            None => ty_infer(expr.ty())?,
        };

        // 変数宣言
        let var = VarDeclChecker::register(
            &mut ctx.var_decl,
            &var_decl.ident,
            Arc::clone(&var_ty),
        )?;

        // 変数宣言文の型は Void
        let ty = Arc::new(Primitive(Void));

        Ok(VarDecl {
            span: var_decl.span,
            var,
            var_ty,
            expr,
            ty,
        })
    }
}

impl<'src> Spanned<'src> for VarDecl<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for VarDecl<'_> {
    fn ty(&self) -> &Arc<Type> {
        &self.var_ty
    }
}
