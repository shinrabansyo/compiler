use sb_compiler_parse_cst::Spanned;

use crate::r#type::*;
use super::error::TypeOpError;

pub fn ty_equals<'a, A, B>(a: &A, b: &B) -> miette::Result<()>
where
    A: Typed,
    B: Typed + Spanned<'a>,
{
    match (a.ty().as_ref(), b.ty().as_ref()) {
        // プリミティブ
        (Void,     Void)     => Ok(()),
        (Bool,     Bool)     => Ok(()),
        (Char,     Char)     => Ok(()),
        (I8,       I8)       => Ok(()),
        (I8,       NumConst) => Ok(()),
        (I16,      I16)      => Ok(()),
        (I16,      NumConst) => Ok(()),
        (I32,      I32)      => Ok(()),
        (I32,      NumConst) => Ok(()),
        (NumConst, NumConst) => Ok(()),
        (NumConst, I8)       => Ok(()),
        (NumConst, I16)      => Ok(()),
        (NumConst, I32)      => Ok(()),
        (NumConst, DataAddr(_)) => Ok(()),
        (NumConst, InstAddr(_)) => Ok(()),

        // アドレス
        (DataAddr(_), DataAddr(_)) => Ok(()),
        (DataAddr(_), NumConst)    => Ok(()),
        (InstAddr(_), InstAddr(_)) => Ok(()),
        (InstAddr(_), NumConst)    => Ok(()),

        // データ構造
        (Struct { .. }, Struct { .. }) => Ok(()),

        // 比較失敗
        _ => Err(TypeOpError::new_mismatch(a, b))
    }
}
