use sb_linker::obj::inst::{Inst, InstValue};
use sb_linker::obj::Object;

pub fn remove_futile_jmp(obj: Object) -> Object {
    let mut result = Vec::with_capacity(obj.code.len() + 20);
    let mut prev_beq_jmp = None;
    for inst in obj.code {
        // Beq -> Label の並びを検出した場合，Beq を削除する
        if let Inst::Label { label } = &inst {
            if let Some(prev_beq_label) = prev_beq_jmp {
                if label == &prev_beq_label {
                    result.pop();
                }
            }
        }

        // Beq で飛ぶ先のラベルを保存
        prev_beq_jmp = if let Inst::Beq {
            rd: 0,
            rs1: 0,
            rs2: 0,
            value: InstValue::InstLabel(label)
        } = &inst {
            Some(*label)
        } else {
            None
        };

        result.push(inst);
    }

    Object {
        code: result,
        ..obj
    }
}
