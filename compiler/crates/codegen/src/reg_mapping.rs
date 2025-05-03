mod coloring;
mod deps_graph;

use std::collections::HashMap;

use sb_compiler_lirgen_ir::LirTree;

use coloring::coloring;
use deps_graph::build_deps_graph;

pub fn mapping(lir_tree: &LirTree) -> HashMap<u32, u8> {
    // 1. レジスタ依存グラフの構築
    let deps_graph = build_deps_graph(&lir_tree);

    // 2. グラフ彩色問題として解く
    let reg_map = coloring(deps_graph);

    reg_map
}
