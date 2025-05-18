mod asmgen;
mod reg_mapping;

use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_codegen_asm::Asm;

use asmgen::asmgen;
use reg_mapping::mapping;

pub fn codegen(lir_top_elem: LirTopElem) -> Asm {
    // 1. レジスタ割り付け (LirBlock -> RegMap)
    let lir_block = lir_top_elem.block();
    let reg_map = mapping(&lir_block);

    // 2. コード生成 (LirBlock + RegMap -> Asm)
    let asm = asmgen(&lir_block, reg_map);

    asm
}
