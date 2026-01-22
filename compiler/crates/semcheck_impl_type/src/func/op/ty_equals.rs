use sb_compiler_parse_cst::Spanned;

use crate::r#type::*;
use super::error::TypeOpError;

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
        _ => Err(TypeOpError::new_mismatch(ty, a))
    }
}

pub fn ty_equals_arith2<'src, L, R>(lhs: &L, rhs: &R) -> miette::Result<()>
where
    L: Typed + Spanned<'src>,
    R: Typed + Spanned<'src>,
{
    match (lhs.ty().as_ref(), rhs.ty().as_ref()) {
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
        (Addr(_),     I32)         => Ok(()),
        (Addr(_),     NumConst)    => Ok(()),
        (DataAddr(_), DataAddr(_)) => Ok(()),
        (DataAddr(_), I32)         => Ok(()),
        (DataAddr(_), NumConst)    => Ok(()),
        (InstAddr(_), InstAddr(_)) => Ok(()),
        (InstAddr(_), I32)         => Ok(()),
        (InstAddr(_), NumConst)    => Ok(()),

        // 比較失敗
        _ => Err(TypeOpError::new_mismatch2(lhs, rhs))
    }
}
