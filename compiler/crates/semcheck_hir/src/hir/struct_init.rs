use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Spanned, Span};
use sb_compiler_semcheck_impl_type::decl::ty_find;
use sb_compiler_semcheck_impl_type::op::ty_equals;
use sb_compiler_semcheck_impl_type::r#struct::ty_struct_field;
use sb_compiler_semcheck_impl_type::{Typed, Type, DataAddr};

use super::{Expr, StructFieldInit, SemCheck, Dep};

#[derive(Debug)]
pub struct StructInit<'src> {
    pub span: Span<'src>,
    pub addr: Box<Expr<'src>>,
    pub fields: Vec<StructFieldInit<'src>>,
    pub ty: Arc<Type>,
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
        ty_equals(&DataAddr(Some(Arc::clone(&struct_ty))), &addr)?;

        // フィールド初期化部分の意味解析
        let mut fields = Vec::new();
        for field_init in struct_init.fields {
            let field_init = StructFieldInit::check(ctx, field_init).await?;
            fields.push(field_init);
        }

        // フィールド初期化部分の型チェック
        for field_init in fields.iter_mut() {
            let (field_ty, offset) = ty_struct_field(&struct_ty, field_init.ident).await?;
            ty_equals(&field_ty, &field_init.expr)?;
            field_init.offset = offset;
        }

        Ok(StructInit {
            span: struct_init.span,
            addr: Box::new(addr),
            fields,
            ty: struct_ty,
        })
    }
}

impl<'src> Spanned<'src> for StructInit<'src> {
    fn span(&self) -> sb_compiler_parse_cst::Span<'src> {
        self.span
    }
}

impl Typed for StructInit<'_> {
    fn ty(&self) -> Arc<Type> {
        self.ty.ty()
    }
}
