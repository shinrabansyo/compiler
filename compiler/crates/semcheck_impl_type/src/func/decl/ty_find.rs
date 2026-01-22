use std::sync::Arc;

use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_async_macros::communicable;
use sb_compiler_type::r#type::Type;

use crate::func::TypeContext;
use super::error::TypeDeclError;

#[communicable]
pub async fn ty_find<'a, 'src>(
    ctx: &'a TypeContext,
    span: Span<'src>,
) -> miette::Result<Arc<Type>> {
    let mut tree = ctx.tree.lock().unwrap();

    // 1. 型名を検索
    let symbol = match tree.interner.get(span.as_str()) {
        Some(symbol) => symbol,
        None => return Err(TypeDeclError::new_not_declared(span)),
    };

    // 2. 型が可視であるか確認
    let node = *tree.nodes.get(&symbol).unwrap();
    if tree.tree.connected(ctx.current_pos, node) {
        Ok(Arc::clone(&tree.types.get(&symbol).unwrap()))
    } else {
        Err(TypeDeclError::new_not_declared_in_scope(span))
    }
}

#[communicable]
pub async fn ty_find_from_mod<'a, 'b, 'src>(
    ctx: &'a TypeContext,
    r#mod: &'b str,
    span: Span<'src>,
) -> miette::Result<(Arc<Type>, String)> {
    let mut tree = ctx.tree.lock().unwrap();

    // 1. 型名を検索
    let full_name = format!("{}.{}", r#mod, span.as_str());
    let symbol = match tree.interner.get(&full_name) {
        Some(symbol) => symbol,
        None => return Err(TypeDeclError::new_not_declared(span)),
    };

    // 2. 型が可視であるか確認
    let node = *tree.nodes.get(&symbol).unwrap();
    if tree.tree.connected(ctx.current_pos, node) {
        let ty = Arc::clone(&tree.types.get(&symbol).unwrap());
        Ok((ty, full_name))
    } else {
        Err(TypeDeclError::new_not_declared_in_scope(span))
    }
}
