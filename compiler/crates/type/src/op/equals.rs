use crate::error::TypeError;
use crate::r#type::*;

pub fn ty_equals(a: &Type, b: &Type) -> anyhow::Result<()> {
    match (a, b) {
        // プリミティブ型
        (Primitive(Void),     Primitive(Void))     => Ok(()),
        (Primitive(I8),       Primitive(I8))       => Ok(()),
        (Primitive(I32),      Primitive(I32))      => Ok(()),
        (Primitive(I8),       Primitive(NumConst)) => Ok(()),
        (Primitive(NumConst), Primitive(I8))       => Ok(()),
        (Primitive(I32),      Primitive(NumConst)) => Ok(()),
        (Primitive(NumConst), Primitive(I32))      => Ok(()),

        // 比較失敗
        _ => Err(TypeError::new_mismatch(*a, *b).into()),
    }
}
