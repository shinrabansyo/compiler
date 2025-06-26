use sb_linker::obj::inst::Inst;
use sb_linker::obj::{inst, Object};

pub fn fix_incomplete_label(obj: Object) -> Object {
    let mut result = Vec::with_capacity(obj.code.len() + 20);
    let mut prev_is_label = false;
    for inst in obj.code {
        if prev_is_label && is_label(&inst) {
            result.push(inst!(Add 0, 12, 4));
        }
        prev_is_label = is_label(&inst);
        result.push(inst);
    }

    Object {
        code: result,
        ..obj
    }
}

fn is_label(inst: &Inst) -> bool {
    match inst {
        Inst::Label { .. } => true,
        _ => false,
    }
}
