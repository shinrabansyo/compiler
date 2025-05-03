mod deps_graph;

use sb_compiler_lirgen_ir::LirTree;
use sb_compiler_codegen_asm::Asm;

use deps_graph::build_deps_graph;

pub fn mapping(lir_tree: &LirTree) -> Asm {
    // 1. レジスタ依存グラフの構築
    let deps_graph = build_deps_graph(&lir_tree);

    todo!()
}
