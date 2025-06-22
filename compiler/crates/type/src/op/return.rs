use std::sync::Arc;

use crate::r#type::*;
use super::ty_equals;

pub fn ty_can_return(namespace: &Arc<Type>, ret_ty: &Arc<Type>) -> anyhow::Result<Arc<Type>> {
    match namespace.as_ref() {
        // 関数
        Function { ret_ty: req_ret_ty, .. } => {
            ty_equals(req_ret_ty, ret_ty)?;
            Ok(Arc::clone(req_ret_ty))
        }

        // return 失敗
        _ => unimplemented!(),
    }
}
