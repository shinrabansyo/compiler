use crate::func::TypeContext;

pub async fn ty_new_space<'src>(
    ctx: &mut TypeContext<'src>,
) -> miette::Result<()> {
    // 新しい型空間 (順序無しレイヤ) を作成
    ctx.cur = ctx
        .graph
        .lock()
        .unwrap()
        .add_undirected_layer(ctx.cur);

    Ok(())
}
