use sb_compiler_parse_cst::Spanned;

use crate::error::TypeError;
use crate::r#type::*;
use crate::Typed;

pub fn ty_cast<'src, F, T>(from: &F, to: &T) -> miette::Result<()>
where
    F: Typed + Spanned<'src>,
    T: Typed,
{
    match (from.ty().as_ref(), to.ty().as_ref()) {
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
        _ => Err(TypeError::new_cast_failed(from, to)),
    }
}
