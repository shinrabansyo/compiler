use sb_compiler_codegen_asm::inst::AsmInst;
use sb_compiler_codegen_asm::Asm;

pub fn cleaning(asm: Asm) -> Asm {
    let asm_insts = cleaning_inst(asm.inst);
    Asm::from(asm_insts)
}

pub fn cleaning_inst(insts: Vec<AsmInst>) -> Vec<AsmInst> {
    fn filter(inst: &AsmInst) -> bool {
        match inst {
            // R-形式
            AsmInst::Add { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::Sub { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::And { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::Or  { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::Xor { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::Sll { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::Srl { rd, rs1: 0, rs2 } if rd == rs2 => false,
            AsmInst::Sra { rd, rs1: 0, rs2 } if rd == rs2 => false,
            _ => true,
        }
    }

    insts.into_iter().filter(filter).collect()
}
