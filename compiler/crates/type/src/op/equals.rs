use std::sync::Arc;

use crate::error::TypeError;
use crate::r#type::*;

pub fn ty_equals(a: &Arc<Type>, b: &Arc<Type>) -> miette::Result<()> {
    match (a.as_ref(), b.as_ref()) {
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
            let a = a.as_ref().clone();
            let b = b.as_ref().clone();
            Err(TypeError::new_mismatch(a, b))
        }
    }
}
