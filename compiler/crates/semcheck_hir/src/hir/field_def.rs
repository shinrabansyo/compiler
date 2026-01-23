use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::decl::ty_link;
use sb_compiler_semcheck_impl_type::{Typed, Type};

use super::{SemCheck, Dep};

#[derive(Debug)]
pub struct FieldDef<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::FieldDef<'src>> for FieldDef<'src> {
    async fn check0(ctx: Dep<'_, 'src>, arg: ast::FieldDef<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        // 型情報取得 (ast::Type -> semcheck_impl_type::Type)
        let ty = ty_link(&ctx.r#type, arg.ty).await?;

        Ok(FieldDef { span: arg.span, ident: arg.ident, ty })
    }
}

impl<'src> Spanned<'src> for FieldDef<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for FieldDef<'_> {
    fn ty(&self) -> Arc<Type> {
        self.ty.ty()
    }
}
