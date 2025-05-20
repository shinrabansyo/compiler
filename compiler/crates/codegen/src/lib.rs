mod asmgen;
mod reg_mapping;

use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_codegen_asm::Asm;

use asmgen::asmgen;
use reg_mapping::mapping;

pub fn codegen(lir_top_elem: LirTopElem) -> Asm {
    let lir_block = match lir_top_elem {
        LirTopElem::Function { body, .. } => body,
    };

    // 1. レジスタ割り付け (LirBlock -> RegMap)
    let reg_map = mapping(&lir_block, &[20, 21, 22, 23, 24, 25, 26, 27, 28, 29]);

    // 2. コード生成 (LirBlock + RegMap -> Asm)
    let asm = asmgen(lir_block, reg_map);

    asm
}
