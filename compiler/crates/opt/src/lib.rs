mod rule;

use sb_linker::obj::Object;

use rule::*;

pub fn optimize(obj: Object) -> Object {
    // 1. 冗長命令削除
    let obj = remove_futile_inst(obj);

    // 2. 冗長なジャンプ命令削除
    let obj = remove_futile_jmp(obj);

    // 3. プロローグ/エピローグのサイズを調整
    let obj = downsize_prologue_epilogue(obj);

    // 4. 各処理の結果として不正な配置となったラベルを修正
    let obj = fix_incomplete_label(obj);

    obj
}
