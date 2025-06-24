use std::sync::Arc;

use sb_compiler_parse_cst::Spanned;

use crate::r#type::*;
use crate::Typed;
use super::ty_equals;

pub fn ty_can_call<'src, C, A>(callee: &C, args: &[A]) -> miette::Result<Arc<Type>>
where
    C: Typed,
    A: Typed + Spanned<'src>,
{
    match callee.ty().as_ref() {
        // 関数
        Function { args: req_args, ret_ty } => {
            // 引数の数が一致しない場合エラー
            if req_args.len() != args.len() {
                let expected = req_args.len();
                let got = args.len();
                panic!("expected {} arguments, but got {}", expected, got);
            }

            // 引数の型が一致しない場合エラー
            for (req_arg, arg) in req_args.iter().zip(args) {
                ty_equals(req_arg.as_ref().clone(), arg)?;
            }

            Ok(Arc::clone(ret_ty))
        }

        // 呼び出し失敗
        _ => panic!(""),
    }
}
