use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_async_macros::communicable;

use crate::var::Var;
use crate::func::VarContext;
use super::error::VarDeclError;

#[communicable]
pub async fn var_find<'a, 'src>(
    ctx: &'a VarContext<'src>,
    span: &'a Span<'src>,
) -> miette::Result<Var<'src>> {
    ctx.graph
        .lock()
        .unwrap()
        .find(ctx.cur, |var| span.as_str() == var.span.as_str())
        .map(|var| var.clone())
        .ok_or_else(|| VarDeclError::new_not_declared(*span))
}
