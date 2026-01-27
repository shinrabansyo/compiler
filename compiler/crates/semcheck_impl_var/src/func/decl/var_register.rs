use std::sync::Arc;

use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_impl_type::Type;

use crate::var::Var;
use crate::func::VarContext;

pub async fn var_register<'src>(
    ctx: &mut VarContext<'src>,
    span: &Span<'src>,
    ty: Arc<Type>,
) -> miette::Result<Var<'src>> {
    let var = Var {
        span: *span,
        ty,
    };

    ctx.cur = ctx
        .graph
        .lock()
        .unwrap()
        .add_node(ctx.cur, var.clone());

    Ok(var)
}
