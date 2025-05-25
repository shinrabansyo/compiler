mod r#gen;
mod reg_mapping;

use sb_linker::obj::Object;

use sb_compiler_lirgen_ir::LirTopElem;

use r#gen::gen_inst;
use reg_mapping::mapping;

pub fn codegen(lir: LirTopElem) -> Object {
    // 1. レジスタ割り付け (LirBlock -> RegMap)
    let reg_map = mapping(&lir, &[20, 21, 22, 23, 24, 25, 26, 27, 28, 29]);

    // 2. コード生成 (LirBlock + RegMap -> Object)
    let asm = gen_inst(lir, reg_map);

    asm
}
