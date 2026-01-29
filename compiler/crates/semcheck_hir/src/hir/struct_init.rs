use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Spanned, Span};
use sb_compiler_semcheck_impl_type::decl::ty_find;
use sb_compiler_semcheck_impl_type::op::ty_equals;
use sb_compiler_semcheck_impl_type::{Typed, Type, DataAddr};

use super::{Expr, StructFieldInit, SemCheck, Dep};

#[derive(Debug)]
pub struct StructInit<'src> {
    pub span: Span<'src>,
    pub addr: Box<Expr<'src>>,
    pub fields: Vec<StructFieldInit<'src>>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::StructInit<'src>> for StructInit<'src> {
    async fn check0(ctx: Dep<'_, 'src>, struct_init: ast::StructInit<'src>) -> miette::Result<StructInit<'src>>
    where
        Self: Sized,
    {
        // 構造体の型取得
        let struct_ty = ty_find(&ctx.r#type, struct_init.ident).await?;

        // アドレス指定部分の意味解析 & 型チェック
        let addr = Expr::check(ctx, *struct_init.addr).await?;
        ty_equals(&DataAddr(struct_ty), &addr)?;

        unimplemented!()
    }
}

impl<'src> Spanned<'src> for StructInit<'src> {
    fn span(&self) -> sb_compiler_parse_cst::Span<'src> {
        unimplemented!()
    }
}

impl Typed for StructInit<'_> {
    fn ty(&self) -> Arc<Type> {
        unimplemented!()
    }
}
