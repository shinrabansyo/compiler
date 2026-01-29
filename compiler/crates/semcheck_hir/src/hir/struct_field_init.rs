use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Spanned, Span};
use sb_compiler_semcheck_impl_type::{Typed, Type};

use super::{Expr, SemCheck, Dep};

#[derive(Debug)]
pub struct StructFieldInit<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub expr: Expr<'src>,
    pub offset: u32,
    pub size: u32,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::StructFieldInit<'src>> for StructFieldInit<'src> {
    async fn check0(ctx: Dep<'_, 'src>, field_init: ast::StructFieldInit<'src>) -> miette::Result<StructFieldInit<'src>>
    where
        Self: Sized,
    {
        let expr = Expr::check(ctx, field_init.expr).await?;
        let size = expr.ty().size();

        Ok(StructFieldInit {
            span: field_init.span,
            ident: field_init.ident,
            expr,
            offset: 0, // 仮 (Struct::check で設定)
            size,
        })
    }
}

impl<'src> Spanned<'src> for StructFieldInit<'src> {
    fn span(&self) -> sb_compiler_parse_cst::Span<'src> {
        self.span
    }
}

impl Typed for StructFieldInit<'_> {
    fn ty(&self) -> Arc<Type> {
        self.expr.ty()
    }
}
