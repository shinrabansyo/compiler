use std::sync::Arc;

use crate::error::TypeError;
use crate::r#type::*;
use super::ty_equals;

pub fn ty_can_call(callee: &Arc<Type>, args: &[&Arc<Type>]) -> anyhow::Result<Arc<Type>> {
    match callee.as_ref() {
        // 関数
        Function { args: req_args, ret_ty } => {
            // 引数の数が一致しない場合エラー
            if req_args.len() != args.len() {
                let expected = req_args.len();
                let got = args.len();
                return Err(TypeError::new_callee_args_mismatch(expected, got).into());
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
