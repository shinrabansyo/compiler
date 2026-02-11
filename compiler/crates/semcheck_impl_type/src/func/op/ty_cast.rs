use sb_compiler_parse_cst::Spanned;

use crate::r#type::*;
use super::error::TypeOpError;

pub fn ty_cast<'a, F, T>(from: &F, to: &T) -> miette::Result<()>
where
    F: Typed + Spanned<'a>,
    T: Typed,
{
    match (from.ty().as_ref(), to.ty().as_ref()) {
        // プリミティブ
        (Void,     Void)        => Ok(()),
        (Bool,     Bool)        => Ok(()),
        (Char,     Char)        => Ok(()),
        (Char,     I8)          => Ok(()),
        (Char,     I16)         => Ok(()),
        (Char,     I32)         => Ok(()),
        (I8,       I8)          => Ok(()),
        (I8,       Bool)        => Ok(()),
        (I8,       Char)        => Ok(()),
        (I8,       I16)         => Ok(()),
        (I8,       I32)         => Ok(()),
        (I16,      I16)         => Ok(()),
        (I16,      Bool)        => Ok(()),
        (I16,      I32)         => Ok(()),
        (I32,      I32)         => Ok(()),
        (I32,      Bool)        => Ok(()),
        (NumConst, Bool)        => Ok(()),
        (NumConst, Char)        => Ok(()),
        (NumConst, I8)          => Ok(()),
        (NumConst, I16)         => Ok(()),
        (NumConst, I32)         => Ok(()),

        // アドレス
        (DataAddr(_), DataAddr(_))   => Ok(()),
        (DataAddr(_), Struct { .. }) => Ok(()),
        (InstAddr(_), InstAddr(_))   => Ok(()),

        // キャスト失敗
        _ => Err(TypeOpError::new_cast_failed(from, to)),
    }
}
