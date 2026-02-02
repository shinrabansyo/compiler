use crate::func::VarContext;

pub async fn var_new_space<'src>(
    ctx: &mut VarContext<'src>,
) -> miette::Result<()> {
    // 新しい変数空間 (順序付きレイヤ) を作成
    ctx.cur = ctx
        .graph
        .lock()
        .unwrap()
        .add_directed_layer(ctx.cur);

    Ok(())
}
