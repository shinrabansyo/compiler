use crate::error::TypeError;
use crate::r#type::*;

pub fn ty_cast(from: &Type, to: &Type) -> anyhow::Result<()> {
    match (from, to) {
        // プリミティブ型
        (Primitive(Void),     Primitive(Void)) => Ok(()),
        (Primitive(I8),       Primitive(I8))   => Ok(()),
        (Primitive(I16),      Primitive(I16))  => Ok(()),
        (Primitive(I32),      Primitive(I32))  => Ok(()),
        (Primitive(NumConst), Primitive(I8))   => Ok(()),
        (Primitive(NumConst), Primitive(I16))  => Ok(()),
        (Primitive(NumConst), Primitive(I32))  => Ok(()),

        // キャスト失敗
        _ => Err(TypeError::new_cast_failed(*from, *to).into()),
    }
}
