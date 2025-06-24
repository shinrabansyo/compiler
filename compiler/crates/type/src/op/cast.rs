use std::sync::Arc;

use crate::error::TypeError;
use crate::r#type::*;

pub fn ty_cast(from: &Arc<Type>, to: &Arc<Type>) -> miette::Result<()> {
    match (from.as_ref(), to.as_ref()) {
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
            let from = from.as_ref().clone();
            let to = to.as_ref().clone();
            Err(TypeError::new_cast_failed(from, to))
        }
    }
}
