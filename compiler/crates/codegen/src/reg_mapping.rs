mod coloring;
mod deps_graph;
mod lifetime;

use std::collections::HashMap;

use sb_compiler_lirgen_ir::LirTree;

use coloring::coloring;
use deps_graph::build_deps_graph;
use lifetime::analyze_lifetime;

pub fn mapping(lir_tree: &LirTree) -> HashMap<u32, u8> {
    // 1. 寿命解析
    let lifetime_tracker = analyze_lifetime(&lir_tree);

    // 2. レジスタ依存グラフの構築
    let deps_graph = build_deps_graph(lifetime_tracker);

    // 3. グラフ彩色問題として解く
    let reg_map = coloring(deps_graph);

    reg_map
}
