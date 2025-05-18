use sb_compiler_codegen_asm::inst::AsmInst;
use sb_compiler_codegen_asm::Asm;

pub fn remove_futile_inst(mut asm: Asm) -> Asm {
    fn filter(inst: &AsmInst) -> bool {
        match inst {
            // R-形式 (rs1 = r0)
            AsmInst::Add { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::Sub { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::And { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::Or  { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::Xor { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::Sll { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::Srl { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::Sra { rd, rs1: 0, rs2 } if rd == rs2 => false,

            // R-形式 (rs2 = r0)s
            AsmInst::Add { rd, rs1, rs2: 0 } if rd == rs1 => false,
            AsmInst::Sub { rd, rs1, rs2: 0 } if rd == rs1 => false,
            AsmInst::And { rd, rs1, rs2: 0 } if rd == rs1 => false,
            AsmInst::Or  { rd, rs1, rs2: 0 } if rd == rs1 => false,
            AsmInst::Xor { rd, rs1, rs2: 0 } if rd == rs1 => false,
            AsmInst::Sll { rd, rs1, rs2: 0 } if rd == rs1 => false,
            AsmInst::Srl { rd, rs1, rs2: 0 } if rd == rs1 => false,
            AsmInst::Sra { rd, rs1, rs2: 0 } if rd == rs1 => false,

            _ => true,
        }
    }

    asm.inst = asm.inst
        .into_iter()
        .filter(filter)
        .collect();

    asm
}
