use sb_compiler_codegen_asm::inst::{AsmInst, AsmInstValue};
use sb_compiler_codegen_asm::Asm;

pub fn remove_futile_jmp(asm: Asm) -> Asm {
    let mut result = Vec::with_capacity(asm.inst.len() + 20);
    let mut prev_beq_jmp = None;
    for inst in asm.inst {
        if let AsmInst::GLabel { label } = &inst {
            if let Some(prev_beq_label) = prev_beq_jmp {
                if label == &prev_beq_label {
                    result.pop();
                }
            }
        }

        prev_beq_jmp = if let AsmInst::Beq {
            rd: 0,
            rs1: 0,
            rs2: 0,
            value: AsmInstValue::InstLabel(label)
        } = &inst {
            Some(label.clone())
        } else {
            None
        };

        result.push(inst);
    }

    Asm {
        inst: result,
        ..asm
    }
}
