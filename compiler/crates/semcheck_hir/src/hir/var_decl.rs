use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};

use super::{Expr, SemCheckFrom, Dep};

#[derive(Debug)]
pub struct VarDecl<'src> {
    pub var: Var<'src>,
    pub ty: Span<'src>,
    pub expr: Expr<'src>,
}

impl<'src> SemCheckFrom<Dep<'_, 'src>, ast::VarDecl<'src>> for VarDecl<'src> {
    async fn check0(ctx: Dep<'_, 'src>, var_decl: ast::VarDecl<'src>) -> anyhow::Result<Self>
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
