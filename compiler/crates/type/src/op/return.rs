use std::sync::Arc;

use crate::r#type::*;
use crate::Typed;
use super::ty_equals;

pub fn ty_can_return<A, B>(namespace: &A, ret_ty: &B) -> miette::Result<Arc<Type>>
where
    A: Typed,
    B: Typed,
{
    match namespace.ty().as_ref() {
        // 関数
        Function { ret_ty: req_ret_ty, .. } => {
            ty_equals(req_ret_ty, ret_ty)?;
            Ok(Arc::clone(req_ret_ty))
        }

        // return 失敗
        _ => unimplemented!(),
    }
}
