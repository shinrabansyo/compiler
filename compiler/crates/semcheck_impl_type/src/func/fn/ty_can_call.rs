use std::sync::Arc;

use sb_compiler_parse_cst::{Span, Spanned};

use crate::r#type::*;
use crate::func::decl::ty_find;
use crate::func::op::ty_equals;
use crate::func::TypeContext;
use super::error::TypeFnError;

pub async fn ty_can_call<'src, A>(
    ctx: &TypeContext<'src>,
    span: Span<'_>,
    args: &[A],
) -> miette::Result<Arc<Type>>
where
    A: Typed + Spanned<'src>,
{
    let callee_ty = ty_find(ctx, span).await?;
    match callee_ty.as_ref() {
        // 関数
        Function { args: req_args, ret_ty, .. } => {
            // 引数の数が一致しない場合エラー
            if req_args.len() != args.len() {
                let expected = req_args.len();
                let got = args.len();
                return Err(TypeFnError::new_argument_nums_mis_match(span, expected, got));
            }

            // 引数の型が一致しない場合エラー
            for (req_arg, arg) in req_args.iter().zip(args) {
                ty_equals(req_arg, arg)?;
            }

            Ok(Arc::clone(ret_ty))
        }

        // 呼び出し失敗
        _ => panic!(""),
    }
}
