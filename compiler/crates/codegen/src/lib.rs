mod reg_mapping;

use sb_compiler_lirgen_ir::LirTree;
use sb_compiler_codegen_asm::Asm;

pub fn codegen(lir_tree: LirTree) -> Asm {
    // 1. レジスタ割り付け (LirTree -> RegMap)
    let reg_map = reg_mapping::mapping(&lir_tree);

    // 2. コード生成 (LirTree + RegMap -> Asm)
    let asm = todo!();

    asm
}
