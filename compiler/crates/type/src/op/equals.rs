use sb_compiler_parse_cst::Spanned;

use crate::error::TypeError;
use crate::r#type::*;
use crate::Typed;

pub fn ty_equals<'src, A>(ty: Type, a: &A) -> miette::Result<()>
where
    A: Typed + Spanned<'src>,
{
    match (&ty, a.ty().as_ref()) {
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
        (NumConst, I8)       => Ok(()),
        (NumConst, I16)      => Ok(()),
        (NumConst, I32)      => Ok(()),
        (NumConst, NumConst) => Ok(()),

        // アドレス
        (Addr(_),     Addr(_))     => Ok(()),
        (DataAddr(_), DataAddr(_)) => Ok(()),
        (InstAddr(_), InstAddr(_)) => Ok(()),

        // 比較失敗
        _ => Err(TypeError::new_mismatch(ty, a))
    }
}

pub fn ty_equals2<'src, A, B>(a: &A, b: &B) -> miette::Result<()>
where
    A: Typed + Spanned<'src>,
    B: Typed + Spanned<'src>,
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
        (NumConst, I8)       => Ok(()),
        (NumConst, I16)      => Ok(()),
        (NumConst, I32)      => Ok(()),
        (NumConst, NumConst) => Ok(()),

        // アドレス
        (Addr(_),     Addr(_))     => Ok(()),
        (DataAddr(_), DataAddr(_)) => Ok(()),
        (InstAddr(_), InstAddr(_)) => Ok(()),

        // 比較失敗
        _ => Err(TypeError::new_mismatch2(a, b))
    }
}
