use std::sync::Arc;

use sb_compiler_parse_ast as ast;

use crate::func::TypeContext;
use crate::r#type::{Type, Typed, Function, Void};
use super::ty_parse_type;

pub async fn ty_parse_func<'a, 'src>(
    ctx: &'a TypeContext,
    ast: &'a ast::FuncDef<'src>,
) -> miette::Result<Arc<Type>> {
    // 引数
    let mut arg_tys = vec![];
    for arg in &ast.args {
        let ty = ty_parse_type(ctx, &arg.ty).await?;
        arg_tys.push(ty);
    }

    // 返り値
    let ret_ty = match &ast.ret_ty {
        Some(ty) => ty_parse_type(ctx, ty).await?,
        None => Void.ty(),
    };

    Ok(Arc::new(Function {
        args: arg_tys,
        ret_ty: ret_ty.ty(),
    }))
}
