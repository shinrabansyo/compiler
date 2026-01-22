use std::sync::Arc;

use sb_compiler_parse_cst::Spanned;

use crate::r#type::*;
use super::error::TypeOpError;

pub fn ty_det_arith2<'src, L, R>(lhs: &L, rhs: &R) -> miette::Result<Arc<Type>>
where
    L: Typed + Spanned<'src>,
    R: Typed + Spanned<'src>,
{
    let lhs_ty = lhs.ty();
    let rhs_ty = rhs.ty();
    match (lhs_ty.as_ref(), rhs_ty.as_ref()) {
        // プリミティブ
        (Void,     Void)     => Ok(lhs_ty),
        (Bool,     Bool)     => Ok(lhs_ty),
        (Char,     Char)     => Ok(lhs_ty),
        (I8,       I8)       => Ok(lhs_ty),
        (I8,       NumConst) => Ok(lhs_ty),
        (I16,      I16)      => Ok(lhs_ty),
        (I16,      NumConst) => Ok(lhs_ty),
        (I32,      I32)      => Ok(lhs_ty),
        (I32,      NumConst) => Ok(lhs_ty),
        (NumConst, I8)       => Ok(rhs_ty),
        (NumConst, I16)      => Ok(rhs_ty),
        (NumConst, I32)      => Ok(rhs_ty),
        (NumConst, NumConst) => Ok(lhs_ty),

        // アドレス
        (Addr(_),     Addr(_))     => Ok(lhs_ty),
        (Addr(_),     I32)         => Ok(lhs_ty),
        (Addr(_),     NumConst)    => Ok(lhs_ty),
        (Addr(_),     DataAddr(_)) => Ok(rhs_ty),
        (Addr(_),     InstAddr(_)) => Ok(rhs_ty),
        (DataAddr(_), Addr(_))     => Ok(lhs_ty),
        (DataAddr(_), I32)         => Ok(lhs_ty),
        (DataAddr(_), NumConst)    => Ok(lhs_ty),
        (DataAddr(_), DataAddr(_)) => Ok(lhs_ty),
        (InstAddr(_), Addr(_))     => Ok(lhs_ty),
        (InstAddr(_), I32)         => Ok(lhs_ty),
        (InstAddr(_), NumConst)    => Ok(lhs_ty),
        (InstAddr(_), InstAddr(_)) => Ok(lhs_ty),

        // 推論失敗
        _ => Err(TypeOpError::new_det2_failed(lhs, rhs)),
    }
}
