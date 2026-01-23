use std::future::Future;
use std::sync::Arc;

use sb_compiler_parse_ast as ast;

use crate::func::decl::ty_find;
use crate::func::TypeContext;
use crate::r#type::{Type, Typed};

pub fn ty_parse_type<'a, 'src>(
    ctx: &'a TypeContext,
    ast: &'a ast::Type<'src>,
) -> impl Future<Output = miette::Result<Arc<Type>>> + use <'a, 'src> {
    Box::pin(async move {
        match ast {
            // アドレス
            ast::Type::Addr { inner_ty, .. } => {
                let inner_ty = ty_parse_type(ctx, inner_ty).await?;
                Ok(Type::Addr(inner_ty).ty())
            }
            ast::Type::DataAddr { inner_ty, .. } => {
                let inner_ty = ty_parse_type(ctx, inner_ty).await?;
                Ok(Type::DataAddr(inner_ty).ty())
            }
            ast::Type::InstAddr { inner_ty, .. } => {
                let inner_ty = ty_parse_type(ctx, inner_ty).await?;
                Ok(Type::InstAddr(inner_ty).ty())
            }

            // ユーザ指定 or プリミティブ
            ast::Type::Term(span) => {
                ty_find(ctx, *span).await
            }
        }
    })
}
