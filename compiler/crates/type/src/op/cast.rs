use std::sync::Arc;

use crate::error::TypeError;
use crate::r#type::*;

pub fn ty_cast(from: &Arc<Type>, to: &Arc<Type>) -> miette::Result<()> {
    match (from.as_ref(), to.as_ref()) {
        // プリミティブ型
        (Primitive(Void),     Primitive(Void)) => Ok(()),
        (Primitive(Bool),     Primitive(Bool)) => Ok(()),
        (Primitive(I8),       Primitive(Bool)) => Ok(()),
        (Primitive(I16),      Primitive(Bool)) => Ok(()),
        (Primitive(I32),      Primitive(Bool)) => Ok(()),
        (Primitive(NumConst), Primitive(Bool)) => Ok(()),
        (Primitive(I8),       Primitive(I8))   => Ok(()),
        (Primitive(I16),      Primitive(I16))  => Ok(()),
        (Primitive(I32),      Primitive(I32))  => Ok(()),
        (Primitive(NumConst), Primitive(I8))   => Ok(()),
        (Primitive(NumConst), Primitive(I16))  => Ok(()),
        (Primitive(NumConst), Primitive(I32))  => Ok(()),

        // キャスト失敗
        _ => {
            let from = from.as_ref().clone();
            let to = to.as_ref().clone();
            Err(TypeError::new_cast_failed(from, to))
        }
    }
}
