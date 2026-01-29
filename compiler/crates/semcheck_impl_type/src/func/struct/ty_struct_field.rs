use std::sync::Arc;

use sb_compiler_parse_cst::Span;

use crate::r#type::{Typed, Type, Struct};

use super::error::TypeStructError;

pub async fn ty_struct_field<'src, S>(
    struct_ty: &S,
    span: Span<'src>,
) -> miette::Result<(Arc<Type>, u32)>
where
    S: Typed,
{
    match struct_ty.ty().as_ref() {
        Struct { fields, .. } => {
            // フィールドを探索しながらオフセットを計算
            let mut offset = 0;
            for (name, ty) in fields {
                if name == span.as_str() {
                    return Ok((Arc::clone(ty), offset));
                }
                offset+= ty.size();
            }

            Err(TypeStructError::new_field_not_exists(struct_ty, span).into())
        }
        _ => Err(TypeStructError::new_not_struct(struct_ty).into()),
    }
}
