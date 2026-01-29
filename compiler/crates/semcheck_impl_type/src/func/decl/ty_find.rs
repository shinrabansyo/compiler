use std::sync::Arc;

use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_async_macros::communicable;

use crate::func::TypeContext;
use crate::r#type::Type;
use super::error::TypeDeclError;

#[communicable]
pub async fn ty_find<'a, 'b, 'src>(
    ctx: &'a TypeContext<'src>,
    span: Span<'b>,
) -> miette::Result<Arc<Type>> {
    println!("{:#?}", ctx.graph);
    println!("serach for type: {}", span.as_str());
    println!("current cursor: {:?}", ctx.cur);

    ctx.graph
        .lock()
        .unwrap()
        .find(ctx.cur, |(name, _)| name == &span.as_str())
        .map(|(_, ty)| Arc::clone(ty))
        .ok_or_else(|| TypeDeclError::new_not_declared(span))
}
