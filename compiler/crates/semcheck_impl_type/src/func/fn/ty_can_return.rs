use std::sync::Arc;

use sb_compiler_parse_cst::Spanned;

use crate::r#type::*;
use crate::func::op::ty_equals;

pub fn ty_can_return<'src, A, B>(returnee: &A, ret_ty: &B) -> miette::Result<Arc<Type>>
where
    A: Typed,
    B: Typed + Spanned<'src>,
{
    match returnee.ty().as_ref() {
        // 関数
        Function { ret_ty: req_ret_ty, .. } => {
            ty_equals(req_ret_ty.as_ref().clone(), ret_ty)?;
            Ok(req_ret_ty.ty())
        }

        // return 失敗
        _ => unimplemented!(),
    }
}
