use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};

use super::{Expr, SemCheckFrom, Dep};

#[derive(Debug)]
pub struct VarDecl<'input> {
    pub var: Var<'input>,
    pub ty: Span<'input>,
    pub expr: Expr<'input>,
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::VarDecl<'input>> for VarDecl<'input> {
    async fn check0(ctx: Dep<'_, 'input>, var_decl: ast::VarDecl<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let expr = Expr::check(ctx, var_decl.expr).await?;
        let var = VarDeclChecker::register(&mut ctx.var_decl, &var_decl.ident)?;

        Ok(VarDecl {
            var,
            ty: var_decl.ty,
            expr,
        })
    }
}
