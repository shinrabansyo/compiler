use std::sync::Arc;

use sb_compiler_parse_cst::Spanned;
use crate::r#type::*;
use super::error::TypeInferError;

pub fn ty_infer<'a, F>(from: &F) -> miette::Result<Arc<Type>>
where
    F: Typed + Spanned<'a>,
{
    let from_ty = from.ty();
    match from_ty.as_ref() {
        // プリミティブ
        Void     => Ok(from_ty),
        Bool     => Ok(from_ty),
        I8       => Ok(from_ty),
        I16      => Ok(from_ty),
        I32      => Ok(from_ty),
        Char     => Ok(from_ty),
        NumConst => Ok(I32.ty()),

        // アドレス
        Addr(_)     => Ok(from_ty),
        DataAddr(_) => Ok(from_ty),
        InstAddr(_) => Ok(from_ty),

        // データ構造
        Struct { .. } => Ok(from_ty),

        // 推論失敗
        _ => Err(TypeInferError::new_infer_failed(from)),
    }
}
