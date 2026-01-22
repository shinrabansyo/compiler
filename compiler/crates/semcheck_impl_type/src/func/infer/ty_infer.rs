use std::sync::Arc;

use crate::r#type::*;
use crate::Typed;

pub fn ty_infer<A>(a: &A) -> miette::Result<Arc<Type>>
where
    A: Typed,
{
    let a_ty = a.ty();
    match a_ty.as_ref() {
        // プリミティブ
        Void     => Ok(a_ty),
        Bool     => Ok(a_ty),
        I8       => Ok(a_ty),
        I16      => Ok(a_ty),
        I32      => Ok(a_ty),
        Char     => Ok(a_ty),
        NumConst => Ok(I32.ty()),

        // アドレス
        Addr(_)     => Ok(a_ty),
        DataAddr(_) => Ok(a_ty),
        InstAddr(_) => Ok(a_ty),

        // 推論失敗
        _ => panic!(""),
    }
}
