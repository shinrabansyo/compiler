use std::sync::Arc;

use crate::error::TypeError;
use crate::r#type::*;

pub fn ty_infer(a: &Arc<Type>) -> anyhow::Result<Arc<Type>> {
    match a.as_ref() {
        // プリミティブ型
        Primitive(Void)     => Ok(Arc::clone(a)),
        Primitive(Bool)     => Ok(Arc::clone(a)),
        Primitive(I8)       => Ok(Arc::clone(a)),
        Primitive(I16)      => Ok(Arc::clone(a)),
        Primitive(I32)      => Ok(Arc::clone(a)),
        Primitive(NumConst) => Ok(Arc::new(Primitive(I32))),

        _ => panic!(""),
    }
}

pub fn ty_infer2(a: &Arc<Type>, b: &Arc<Type>) -> anyhow::Result<Arc<Type>> {
    match (a.as_ref(), b.as_ref()) {
        // プリミティブ型
        (Primitive(Void),     Primitive(Void))     => Ok(Arc::clone(a)),
        (Primitive(Bool),     Primitive(Bool))     => Ok(Arc::clone(a)),
        (Primitive(I8),       Primitive(I8))       => Ok(Arc::clone(a)),
        (Primitive(I16),      Primitive(I16))      => Ok(Arc::clone(a)),
        (Primitive(I32),      Primitive(I32))      => Ok(Arc::clone(a)),
        (Primitive(I8),       Primitive(NumConst)) => Ok(Arc::clone(a)),
        (Primitive(I16),      Primitive(NumConst)) => Ok(Arc::clone(a)),
        (Primitive(I32),      Primitive(NumConst)) => Ok(Arc::clone(a)),
        (Primitive(NumConst), Primitive(I8))       => Ok(Arc::clone(b)),
        (Primitive(NumConst), Primitive(I16))      => Ok(Arc::clone(b)),
        (Primitive(NumConst), Primitive(I32))      => Ok(Arc::clone(b)),
        (Primitive(NumConst), Primitive(NumConst)) => Ok(Arc::clone(b)),

        // 推論失敗
        _ => {
            let a = a.as_ref().clone();
            let b = b.as_ref().clone();
            Err(TypeError::new_infer2_failed(a, b).into())
        }
    }
}
