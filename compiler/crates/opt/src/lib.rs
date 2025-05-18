mod rule;

use sb_compiler_codegen_asm::Asm;

use rule::*;

pub fn optimize(asm: Asm) -> Asm {
    // 1. 冗長命令削除
    let asm = remove_futile_inst(asm);

    // 2. 冗長なジャンプ命令削除
    let asm = remove_futile_jmp(asm);

    // 3. 各処理の結果として不正な配置となったラベルを修正
    let asm = fix_incomplete_label(asm);

    asm
}
