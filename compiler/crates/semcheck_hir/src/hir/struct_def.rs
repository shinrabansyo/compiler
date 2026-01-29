use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::decl::ty_register;
use sb_compiler_semcheck_impl_type::parse::ty_parse_struct;
use sb_compiler_semcheck_impl_type::{Typed, Type, Void};

use super::{FieldDef, SemCheck, InDep};

#[derive(Debug)]
pub struct StructDef<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub fields: Vec<FieldDef<'src>>,
}

impl<'src> SemCheck<InDep<'src>, ast::StructDef<'src>> for StructDef<'src> {
    async fn check0(mut ctx: InDep<'src>, struct_def: ast::StructDef<'src>) -> miette::Result<Self>
        where
            Self: Sized
    {
        // 型登録
        let struct_ty = ty_parse_struct(&ctx.r#type, &struct_def).await?;
        ty_register(&mut ctx.r#type, struct_def.ident, struct_ty).await?;

        // フィールド要素の意味解析
        let mut fields = vec![];
        for field in struct_def.fields {
            fields.push(FieldDef::check(&mut ctx, field).await?);
        }

        Ok(StructDef {
            span: struct_def.span,
            ident: struct_def.ident,
            fields,
        })
    }
}

impl<'src> Spanned<'src> for StructDef<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for StructDef<'_> {
    fn ty(&self) -> Arc<Type> {
        Void.ty()
    }
}
