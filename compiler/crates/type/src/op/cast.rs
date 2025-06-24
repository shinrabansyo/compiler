use crate::error::TypeError;
use crate::r#type::*;
use crate::Typed;

pub fn ty_cast<F, T>(from: &F, to: &T) -> miette::Result<()>
where
    F: Typed,
    T: Typed,
{
    match (from.ty().as_ref(), to.ty().as_ref()) {
        // プリミティブ型
        (Void,     Void) => Ok(()),
        (Bool,     Bool) => Ok(()),
        (I8,       Bool) => Ok(()),
        (I16,      Bool) => Ok(()),
        (I32,      Bool) => Ok(()),
        (NumConst, Bool) => Ok(()),
        (I8,       I8)   => Ok(()),
        (I16,      I16)  => Ok(()),
        (I32,      I32)  => Ok(()),
        (NumConst, I8)   => Ok(()),
        (NumConst, I16)  => Ok(()),
        (NumConst, I32)  => Ok(()),

        // キャスト失敗
        _ => {
            let from_ty = (*from.ty()).clone();
            let to_ty = (*to.ty()).clone();
            Err(TypeError::new_cast_failed(from_ty, to_ty))
        }
    }
}
