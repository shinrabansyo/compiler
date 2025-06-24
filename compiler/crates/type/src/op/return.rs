use std::sync::Arc;

use crate::r#type::*;
use crate::Typed;
use super::ty_equals2;

pub fn ty_can_return<A, B>(namespace: &A, ret_ty: &B) -> miette::Result<Arc<Type>>
where
    A: Typed,
    B: Typed,
{
    match namespace.ty().as_ref() {
        // 関数
        Function { ret_ty: req_ret_ty, .. } => {
            ty_equals2(req_ret_ty, ret_ty)?;
            Ok(req_ret_ty.ty())
        }

        // return 失敗
        _ => unimplemented!(),
    }
}
