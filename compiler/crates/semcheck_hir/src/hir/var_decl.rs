use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_var::decl::var_register;
use sb_compiler_semcheck_impl_var::Var;
use sb_compiler_semcheck_impl_type::decl::ty_link;
use sb_compiler_semcheck_impl_type::op::ty_equals;
use sb_compiler_semcheck_impl_type::infer::ty_infer;
use sb_compiler_semcheck_impl_type::{Typed, Type, Void};

use super::{Expr, SemCheck, Dep};

#[derive(Debug)]
pub struct VarDecl<'src> {
    pub span: Span<'src>,
    pub var: Var<'src>,
    pub var_ty: Arc<Type>,
    pub expr: Expr<'src>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::VarDecl<'src>> for VarDecl<'src> {
    async fn check0(ctx: Dep<'_, 'src>, var_decl: ast::VarDecl<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        // 式の意味解析
        let expr = Expr::check(ctx, var_decl.expr).await?;

        // 型検査
        let var_ty = match var_decl.ty {
            Some(ty) => {
                let ty = ty_link(&ctx.r#type, ty).await?;
                ty_equals(ty.as_ref().clone(), &expr)?;
                ty
            }
            None => ty_infer(&expr)?,
        };

        // 変数宣言
        let var = var_register(&mut ctx.var, &var_decl.ident, var_ty.ty()).await?;

        Ok(VarDecl { span: var_decl.span, var, var_ty, expr })
    }
}

impl<'src> Spanned<'src> for VarDecl<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for VarDecl<'_> {
    fn ty(&self) -> Arc<Type> {
        Void.ty()
    }
}
