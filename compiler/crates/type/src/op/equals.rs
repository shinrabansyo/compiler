use std::sync::Arc;

use crate::error::TypeError;
use crate::r#type::*;

pub fn ty_equals(a: &Arc<Type>, b: &Arc<Type>) -> anyhow::Result<()> {
    match (a.as_ref(), b.as_ref()) {
        // プリミティブ型
        (Primitive(Void),     Primitive(Void))     => Ok(()),
        (Primitive(Bool),     Primitive(Bool))     => Ok(()),
        (Primitive(I8),       Primitive(I8))       => Ok(()),
        (Primitive(I16),      Primitive(I16))      => Ok(()),
        (Primitive(I32),      Primitive(I32))      => Ok(()),
        (Primitive(I8),       Primitive(NumConst)) => Ok(()),
        (Primitive(NumConst), Primitive(I8))       => Ok(()),
        (Primitive(I16),      Primitive(NumConst)) => Ok(()),
        (Primitive(NumConst), Primitive(I16))      => Ok(()),
        (Primitive(I32),      Primitive(NumConst)) => Ok(()),
        (Primitive(NumConst), Primitive(I32))      => Ok(()),

        // 比較失敗
        _ => {
            let a = a.as_ref().clone();
            let b = b.as_ref().clone();
            Err(TypeError::new_mismatch(a, b).into())
        }
    }
}
