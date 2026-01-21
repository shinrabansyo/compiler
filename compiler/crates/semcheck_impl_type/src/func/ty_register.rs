use std::sync::Arc;

use sb_compiler_type::r#type::Type;

use crate::func::Context;
use crate::error::TypeDeclError;

pub async fn ty_register(ctx: &mut Context, name: &str, ty: Arc<Type>) -> miette::Result<()> {
    let mut tree = ctx.tree.lock().unwrap();

    // 1. 型名を登録
    let symbol = match tree.interner.get(name) {
        Some(_) => {
            let err = TypeDeclError::new_already_declared(name.to_string());
            return Err(err);
        }
        None => tree.interner.get_or_intern(name),
    };
    tree.types.insert(symbol, ty);

    // 2. 参照木に追加
    let parent = ctx.current_pos;
    let child = tree.tree.make_tree(0.);
    tree.tree.link(parent, child);
    tree.nodes.insert(symbol, child);

    // 3. 以降の文脈で新しい型を参照できるように更新
    ctx.current_pos = child;

    Ok(())
}
