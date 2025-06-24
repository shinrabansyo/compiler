use std::sync::Arc;

use crate::error::TypeError;
use crate::r#type::*;

pub fn ty_infer(a: &Arc<Type>) -> miette::Result<Arc<Type>> {
    match a.as_ref() {
        // プリミティブ型
        Void     => Ok(Arc::clone(a)),
        Bool     => Ok(Arc::clone(a)),
        I8       => Ok(Arc::clone(a)),
        I16      => Ok(Arc::clone(a)),
        I32      => Ok(Arc::clone(a)),
        NumConst => Ok(Arc::clone(a)),

        // 推論失敗
        _ => panic!(""),
    }
}

pub fn ty_infer2(a: &Arc<Type>, b: &Arc<Type>) -> miette::Result<Arc<Type>> {
    match (a.as_ref(), b.as_ref()) {
        // プリミティブ型
        (Void,     Void)     => Ok(Arc::clone(a)),
        (Bool,     Bool)     => Ok(Arc::clone(a)),
        (I8,       I8)       => Ok(Arc::clone(a)),
        (I8,       NumConst) => Ok(Arc::clone(a)),
        (I16,      I16)      => Ok(Arc::clone(a)),
        (I16,      NumConst) => Ok(Arc::clone(a)),
        (I32,      I32)      => Ok(Arc::clone(a)),
        (I32,      NumConst) => Ok(Arc::clone(a)),
        (NumConst, I8)       => Ok(Arc::clone(b)),
        (NumConst, I16)      => Ok(Arc::clone(b)),
        (NumConst, I32)      => Ok(Arc::clone(b)),
        (NumConst, NumConst) => Ok(Arc::clone(a)),

        // 推論失敗
        _ => {
            let a = a.as_ref().clone();
            let b = b.as_ref().clone();
            Err(TypeError::new_infer2_failed(a, b))
        }
    }
}
