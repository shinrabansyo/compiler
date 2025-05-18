mod rule;

use sb_compiler_codegen_asm::Asm;

use rule::*;

pub fn optimize(asm: Asm) -> Asm {
    // 1. 冗長命令削除
    let asm = remove_futile_inst(asm);

    asm
}
