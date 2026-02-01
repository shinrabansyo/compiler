use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::r#struct::ty_struct_field;
use sb_compiler_semcheck_impl_type::{Typed, Type};

use super::{Value, SemCheck, Dep};

#[derive(Debug)]
pub struct StructAccess<'src> {
    pub span: Span<'src>,
    pub target: Box<Value<'src>>,
    pub offset: u32,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::StructAccess<'src>> for StructAccess<'src> {
    async fn check0(ctx: Dep<'_, 'src>, struct_access: ast::StructAccess<'src>) -> miette::Result<Self>
        where
            Self: Sized
    {
        // 左辺の意味解析
        let lhs = Value::check(ctx, *struct_access.lhs).await?;

        // アクセス対象の情報を取得
        let (rhs_ty, rhs_addr) = ty_struct_field(
            &lhs.ty(),
            struct_access.rhs,
        ).await?;

        Ok(StructAccess {
            span: struct_access.span,
            target: Box::new(lhs),
            offset: rhs_addr,
            ty: rhs_ty,
        })
    }
}

impl<'src> Spanned<'src> for StructAccess<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for StructAccess<'_> {
    fn ty(&self) -> Arc<Type> {
        self.ty.ty()
    }
}
