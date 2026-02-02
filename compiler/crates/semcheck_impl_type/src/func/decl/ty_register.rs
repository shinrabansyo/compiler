use std::sync::Arc;

use sb_compiler_parse_cst::Span;

use crate::func::TypeContext;
use crate::r#type::Type;

pub async fn ty_register<'src>(
    ctx: &mut TypeContext<'src>,
    span: Span<'src>,
    ty: Arc<Type>,
) -> miette::Result<()> {
    // 既に登録されている型名の場合，エラー
    let already_declared = ctx.graph
        .lock()
        .unwrap()
        .find(ctx.cur, |(name, _)| name == &span.as_str())
        .is_some();
    if already_declared {
        return Err(super::error::TypeDeclError::new_already_declared(span));
    }

    // 登録作業
    ctx.cur = ctx.graph
        .lock()
        .unwrap()
        .add_node(ctx.cur, (span.as_str(), ty));

    Ok(())
}
