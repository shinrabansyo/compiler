use sb_compiler_codegen_asm::inst::{Add, AsmInst};
use sb_compiler_codegen_asm::{asmi, Asm};

pub fn fix_incomplete_label(asm: Asm) -> Asm {
    let mut result = Vec::with_capacity(asm.inst.len() + 20);
    let mut prev_is_label = false;
    for inst in asm.inst {
        if prev_is_label && is_label(&inst) {
            result.push(asmi!(Add 0, 12, 4));
        }
        prev_is_label = is_label(&inst);
        result.push(inst);
    }

    Asm {
        inst: result,
        ..asm
    }
}

fn is_label(inst: &AsmInst) -> bool {
    match inst {
        AsmInst::LLabel { .. } | AsmInst::GLabel { .. } => true,
        _ => false,
    }
}
