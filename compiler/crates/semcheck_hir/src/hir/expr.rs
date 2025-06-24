use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Spanned;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{Assign, SemCheck, Dep};

#[derive(Debug)]
pub struct Expr<'src> {
    pub assign: Assign<'src>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Expr<'src>> for Expr<'src> {
    async fn check0(ctx: Dep<'_, 'src>, expr: ast::Expr<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        Ok(Expr {
            assign: Assign::check(ctx, expr.assign).await?,
        })
    }
}

impl<'src> Spanned<'src> for Expr<'src> {
    fn span(&self) -> sb_compiler_parse_cst::Span<'src> {
        self.assign.span()
    }
}

impl Typed for Expr<'_> {
    fn ty(&self) -> &Arc<Type> {
        self.assign.ty()
    }
}
