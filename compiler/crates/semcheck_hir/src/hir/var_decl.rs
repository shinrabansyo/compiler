use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_impl_vardecl::{VarDeclChecker, VarId};

use super::{Expr, SemCheckFrom, Dep};

#[derive(Debug)]
pub struct VarDecl<'input> {
    pub ident: VarId,
    pub ty: Span<'input>,
    pub expr: Expr<'input>,
}

impl<'input> SemCheckFrom<Dep<'_>, ast::VarDecl<'input>> for VarDecl<'input> {
    async fn check0(ctx: Dep<'_>, var_decl: ast::VarDecl<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let var_id = VarDeclChecker::register(
            &mut ctx.var_decl,
            var_decl.ident.as_str(),
        )?;

        Ok(VarDecl {
            ident: var_id,
            ty: var_decl.ty,
            expr: Expr::check(ctx, var_decl.expr).await?,
        })
    }
}
