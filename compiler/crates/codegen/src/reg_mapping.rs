mod coloring;
mod deps_graph;
mod lifetime;

use std::collections::HashMap;

use sb_compiler_lirgen_ir::LirTopElem;

use coloring::coloring;
use deps_graph::build_deps_graph;
use lifetime::analyze_lifetime;

pub fn mapping(lir: &LirTopElem, usable_regs: &[u8]) -> HashMap<u32, u8> {
    // 1. 寿命解析
    let lir_block = match lir {
        LirTopElem::Function { body, .. } => body,
    };
    let lifetime_tracker = analyze_lifetime(lir_block);

    // 2. レジスタ依存グラフの構築
    let deps_graph = build_deps_graph(lifetime_tracker);

    // 3. グラフ彩色問題として解く
    let reg_map = coloring(deps_graph, usable_regs);

    reg_map
}
