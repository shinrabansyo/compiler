mod asmgen;
mod cleaning;
mod reg_mapping;

use sb_compiler_lirgen_ir::LirTree;
use sb_compiler_codegen_asm::Asm;

use asmgen::asmgen;
use cleaning::cleaning;
use reg_mapping::mapping;

pub fn codegen(lir_tree: LirTree) -> Asm {
    // 1. レジスタ割り付け (LirTree -> RegMap)
    let reg_map = mapping(&lir_tree);

    // 2. コード生成 (LirTree + RegMap -> Asm)
    let asm = asmgen(lir_tree, reg_map);

    // 3. 冗長コード削減 (Asm -> Asm)
    let asm = cleaning(asm);

    asm
}
