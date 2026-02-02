use std::sync::Arc;

use sb_compiler_parse_ast as ast;

use crate::func::TypeContext;
use crate::r#type::{Type, Struct};
use super::ty_parse_type;

pub async fn ty_parse_struct<'a, 'src>(
    ctx: &'a TypeContext<'src>,
    ast: &'a ast::StructDef<'src>,
) -> miette::Result<Arc<Type>> {
    // フィールド要素
    let mut field_tys = vec![];
    for field in &ast.fields {
        let field_ty = ty_parse_type(ctx, &field.ty).await?;
        field_tys.push((field.ident.as_str().to_string(), field_ty));
    }

    Ok(Arc::new(Struct {
        name: ast.ident.as_str().to_string(),
        fields: field_tys.into_iter().collect(),
    }))
}
