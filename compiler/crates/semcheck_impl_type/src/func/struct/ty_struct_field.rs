use std::sync::Arc;

use sb_compiler_parse_cst::Span;

use crate::r#type::{Typed, Type, Struct};

use super::error::TypeStructError;

pub async fn ty_struct_field<'src, S>(
    struct_ty: &S,
    span: Span<'src>,
) -> miette::Result<Arc<Type>>
where
    S: Typed,
{
    match struct_ty.ty().as_ref() {
        Struct { fields, .. } => {
            fields.get(span.as_str())
                .map(Arc::clone)
                .ok_or_else(|| TypeStructError::new_field_not_exists(struct_ty, span))
        }
        _ => Err(TypeStructError::new_not_struct(struct_ty).into()),
    }
}
