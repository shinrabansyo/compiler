use crate::error::TypeError;
use crate::r#type::*;
use crate::Typed;

pub fn ty_equals<A>(ty: Type, a: &A) -> miette::Result<()>
where
    A: Typed,
{
    ty_equals2(&ty.ty(), a)
}

pub fn ty_equals2<A, B>(a: &A, b: &B) -> miette::Result<()>
where
    A: Typed,
    B: Typed,
{
    match (a.ty().as_ref(), b.ty().as_ref()) {
        // プリミティブ型
        (Void,     Void)     => Ok(()),
        (Bool,     Bool)     => Ok(()),
        (I8,       I8)       => Ok(()),
        (I8,       NumConst) => Ok(()),
        (I16,      I16)      => Ok(()),
        (I16,      NumConst) => Ok(()),
        (I32,      I32)      => Ok(()),
        (I32,      NumConst) => Ok(()),
        (NumConst, I8)       => Ok(()),
        (NumConst, I16)      => Ok(()),
        (NumConst, I32)      => Ok(()),
        (NumConst, NumConst) => Ok(()),

        // 比較失敗
        _ => {
            let a_ty = (*a.ty()).clone();
            let b_ty = (*b.ty()).clone();
            Err(TypeError::new_mismatch(a_ty, b_ty))
        }
    }
}
