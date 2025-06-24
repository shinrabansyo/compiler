use std::sync::Arc;

use crate::error::TypeError;
use crate::r#type::*;
use crate::Typed;

pub fn ty_infer<A>(a: &A) -> miette::Result<Arc<Type>>
where
    A: Typed,
{
    let a_ty = a.ty();
    match a_ty.as_ref() {
        // プリミティブ型
        Void     => Ok(a_ty),
        Bool     => Ok(a_ty),
        I8       => Ok(a_ty),
        I16      => Ok(a_ty),
        I32      => Ok(a_ty),
        NumConst => Ok(a_ty),

        // 推論失敗
        _ => panic!(""),
    }
}

pub fn ty_infer2<A, B>(a: &A, b: &B) -> miette::Result<Arc<Type>>
where
    A: Typed,
    B: Typed,
{
    let a_ty = a.ty();
    let b_ty = b.ty();
    match (a_ty.as_ref(), b_ty.as_ref()) {
        // プリミティブ型
        (Void,     Void)     => Ok(a_ty),
        (Bool,     Bool)     => Ok(a_ty),
        (I8,       I8)       => Ok(a_ty),
        (I8,       NumConst) => Ok(a_ty),
        (I16,      I16)      => Ok(a_ty),
        (I16,      NumConst) => Ok(a_ty),
        (I32,      I32)      => Ok(a_ty),
        (I32,      NumConst) => Ok(a_ty),
        (NumConst, I8)       => Ok(b_ty),
        (NumConst, I16)      => Ok(b_ty),
        (NumConst, I32)      => Ok(b_ty),
        (NumConst, NumConst) => Ok(a_ty),

        // 推論失敗
        _ => {
            let a_ty = (*a_ty).clone();
            let b_ty = (*b_ty).clone();
            Err(TypeError::new_infer2_failed(a_ty, b_ty))
        }
    }
}
