use crate::error::TypeError;
use crate::r#type::*;

pub fn ty_infer2(a: &Type, b: &Type) -> anyhow::Result<Type> {
    match (a, b) {
        // プリミティブ型
        (Primitive(Void),     Primitive(Void))     => Ok(Primitive(Void)),
        (Primitive(I8),       Primitive(I8))       => Ok(Primitive(I8)),
        (Primitive(I16),      Primitive(I16))      => Ok(Primitive(I16)),
        (Primitive(I32),      Primitive(I32))      => Ok(Primitive(I32)),
        (Primitive(NumConst), Primitive(NumConst)) => Ok(Primitive(NumConst)),

        // 推論失敗
        _ => Err(TypeError::new_infer2_failed(*a, *b).into()),
    }
}
