use std::sync::Arc;

use petgraph::prelude::NodeIndex;

use sb_compiler_parse_cst::Span;
use sb_compiler_type::r#type::Type;

use crate::data::Var;
use crate::func::VarContext;

pub async fn var_register<'src>(
    ctx: &mut VarContext<'src>,
    span: &Span<'src>,
    ty: Arc<Type>,
) -> miette::Result<Var<'src>> {
    let mut graph = ctx.graph.lock().unwrap();

    // 1. 変数名を登録
    let mut var = Var {
        id: NodeIndex::new(0),
        span: *span,
        ty,
    };

    // 2. 参照グラフにノードを追加
    let from = graph.graph.add_node(var.clone());
    var.id = from;
    *graph.graph.node_weight_mut(from).unwrap() = var.clone();

    // 3. 追加したノードを追跡可能にする
    let to = ctx.node;
    graph.graph.add_edge(from, to, ());
    graph.nodes.insert(var.clone(), from);
    ctx.node = from;

    Ok(var)
}
