use std::sync::Arc;

use petgraph::prelude::NodeIndex;

use sb_compiler_parse_cst::Span;
use sb_compiler_type::r#type::Type;

use crate::data::Var;
use crate::func::Context;

pub async fn var_register<'src>(
    ctx: &mut Context<'src>,
    span: &Span<'src>,
    ty: Arc<Type>,
) -> miette::Result<Var<'src>> {
    let mut checker = ctx.checker.lock().unwrap();

    // 1. 変数名を登録
    let mut var = Var {
        id: NodeIndex::new(0),
        span: *span,
        ty,
    };

    // 2. 参照グラフにノードを追加
    let from = checker.graph.add_node(var.clone());
    var.id = from;
    *checker.graph.node_weight_mut(from).unwrap() = var.clone();

    // 3. あああ
    let to = ctx.node;
    checker.graph.add_edge(from, to, ());
    checker.nodes.insert(var.clone(), from);
    ctx.node = from;

    Ok(var)
}
