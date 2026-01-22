use std::sync::Arc;

use sb_compiler_parse_cst::{Span, Spanned};

use crate::r#type::*;
use crate::func::decl::ty_find;
use crate::func::op::ty_equals;
use crate::func::TypeContext;
use super::error::TypeFnError;

pub async fn ty_can_return<'src, A>(
    ctx: &TypeContext,
    span: Span<'src>,
    ret_ty: &A,
) -> miette::Result<Arc<Type>>
where
    A: Typed + Spanned<'src>,
{
    let returnee = ty_find(ctx, span).await?;
    match returnee.as_ref() {
        // 関数
        Function { ret_ty: req_ret_ty, .. } => {
            if ty_equals(req_ret_ty.as_ref().clone(), ret_ty).is_err() {
                return Err(TypeFnError::new_return_failed(req_ret_ty.as_ref(), ret_ty));
            }
            Ok(req_ret_ty.ty())
        }

        // return 失敗
        _ => unimplemented!(),
    }
}
