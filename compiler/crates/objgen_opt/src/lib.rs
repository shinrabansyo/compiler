mod rule;

use sb_linker::obj::Object;

use rule::*;

pub fn optimize(obj: Object) -> Object {
    // 冗長命令削除
    let obj = remove_futile_inst(obj);

    // 冗長なジャンプ命令削除
    let obj = remove_futile_jmp(obj);

    // プロローグ/エピローグのサイズを調整
    let obj = downsize_prologue_epilogue(obj);

    // 各処理の結果として不正な配置となったラベルを修正
    let obj = fix_incomplete_label(obj);

    obj
}
