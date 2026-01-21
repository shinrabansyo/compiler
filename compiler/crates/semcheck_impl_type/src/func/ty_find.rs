use std::sync::Arc;

use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_async_macros::failable_as_async;
use sb_compiler_type::r#type::*;

use crate::func::Context;
use crate::error::TypeDeclError;

#[failable_as_async]
pub async fn ty_find<'a>(ctx: &'a Context, name: &'a str) -> miette::Result<Arc<Type>> {
    let mut tree = ctx.tree.lock().unwrap();

    // 1. 型名を検索
    let symbol = match tree.interner.get(name) {
        Some(symbol) => symbol,
        None => {
            let err = TypeDeclError::new_not_declared(name.to_string());
            return Err(err);
        }
    };

    // 2. 型が可視であるか確認
    let node = *tree.nodes.get(&symbol).unwrap();
    if tree.tree.connected(ctx.current_pos, node) {
        Ok(Arc::clone(&tree.types.get(&symbol).unwrap()))
    } else {
        let err = TypeDeclError::new_not_declared_in_scope(name.to_string());
        Err(err)
    }
}
