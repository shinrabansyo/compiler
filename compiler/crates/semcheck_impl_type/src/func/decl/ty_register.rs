use std::sync::Arc;

use sb_compiler_parse_cst::Span;

use crate::func::TypeContext;
use crate::r#type::Type;
use super::error::TypeDeclError;

pub async fn ty_register<'src>(
    ctx: &mut TypeContext,
    span: Span<'src>,
    ty: Arc<Type>,
) -> miette::Result<()> {
    let mut tree = ctx.tree.lock().unwrap();

    // 1. 型名を登録
    let symbol = match tree.interner.get(span.as_str()) {
        Some(_) => return Err(TypeDeclError::new_already_declared(span)),
        None => tree.interner.get_or_intern(span.as_str()),
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

pub async fn ty_register_in_mod<'src>(
    ctx: &mut TypeContext,
    r#mod: &str,
    span: Span<'src>,
    ty: Arc<Type>,
) -> miette::Result<String> {
    let mut tree = ctx.tree.lock().unwrap();

    // 1. 型名を登録
    let full_name = format!("{}.{}", r#mod, span.as_str());
    let symbol = match tree.interner.get(&full_name) {
        Some(_) => return Err(TypeDeclError::new_already_declared(span)),
        None => tree.interner.get_or_intern(&full_name),
    };
    tree.types.insert(symbol, ty);

    // 2. 参照木に追加
    let parent = ctx.current_pos;
    let child = tree.tree.make_tree(0.);
    tree.tree.link(parent, child);
    tree.nodes.insert(symbol, child);

    // 3. 以降の文脈で新しい型を参照できるように更新
    ctx.current_pos = child;

    Ok(full_name)
}
