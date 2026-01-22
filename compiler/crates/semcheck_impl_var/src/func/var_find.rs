use petgraph::algo::astar;

use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_async_macros::communicable;

use crate::data::Var;
use crate::func::VarContext;
use crate::error::VarDeclError;

#[communicable]
pub async fn var_find<'a, 'src>(
    ctx: &'a VarContext<'src>,
    span: &'a Span<'src>,
) -> miette::Result<Var<'src>> {
    let graph = ctx.graph.lock().unwrap();

    // 1. 可視変数の洗い出し
    let from = ctx.node;
    let to = graph.root_node;
    let waypoints = astar(
        &graph.graph,
        from,
        |n| n == to,
        |_| 0,      // 連結を確認するだけなので辺の重みは無視
        |_| 0,      // 連結を確認するだけなので辺の重みは無視
    ).unwrap().1;

    // 2. 経路を順に見て，初めて見つけた変数を検索結果とする
    for waypoint in waypoints {
        let waypoint_var = graph.graph.node_weight(waypoint).unwrap();
        if span.as_str() == waypoint_var.span.as_str() {
            return Ok(waypoint_var.clone());
        }
    }

    Err(VarDeclError::new_not_declared(*span))
}
