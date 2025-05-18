use std::collections::HashMap;

use sb_compiler_lirgen_ir::{LirTree, LirInst};
use sb_compiler_codegen_asm::inst::*;
use sb_compiler_codegen_asm::{asmi, Asm};

pub fn asmgen(lir_tree: LirTree, reg_map: HashMap<u32, u8>) -> Asm {
    let asm_inst = AsmInstGenerator::gen(lir_tree, reg_map);
    Asm::from(asm_inst)
}

struct AsmInstGenerator {
    reg_map: HashMap<u32, u8>,
    asm_inst: Vec<AsmInst>,
}

impl AsmInstGenerator {
    fn gen(lir_tree: LirTree, reg_map: HashMap<u32, u8>) -> Vec<AsmInst> {
        let mut generator = AsmInstGenerator {
            reg_map,
            asm_inst: Vec::new(),
        };
        generator.gen_recursive(lir_tree);
        generator.asm_inst.push(asmi!(Add 0, 12, 4));

        generator.asm_inst
    }

    fn gen_recursive(&mut self, lir_tree: LirTree) {
        match lir_tree {
            LirTree::Single { lirs, .. } => {
                for lir in lirs {
                    self.gen_recursive(lir);
                }
            }
            LirTree::Multiple { lirs } => {
                for lir in lirs {
                    self.gen_recursive(lir);
                }
            }
            LirTree::Inst { inst, dst, src1, src2 } => {
                let dst = *self.reg_map.get(&dst).unwrap_or(&0);
                let src1 = *self.reg_map.get(&src1).unwrap_or(&0);
                let src2 = *self.reg_map.get(&src2).unwrap_or(&0);

                let asm = match inst {
                    // Nop
                    LirInst::Nop => asmi!(Add 0, 0, 0),

                    // 整数演算 (imm 使用)
                    LirInst::Li(imm) => asmi!(Addi dst, 0, Imm(imm)),
                    LirInst::Addi(imm) => asmi!(Addi dst, src1, Imm(imm)),
                    LirInst::Subi(imm) => asmi!(Subi dst, src1, Imm(imm)),
                    LirInst::Andi(imm) => asmi!(Andi dst, src1, Imm(imm)),
                    LirInst::Ori(imm) => asmi!(Ori dst, src1, Imm(imm)),
                    LirInst::Xori(imm) => asmi!(Xori dst, src1, Imm(imm)),
                    LirInst::ShiftLi(imm) => asmi!(Slli dst, src1, Imm(imm)),
                    LirInst::ShiftRi(imm) => asmi!(Srli dst, src1, Imm(imm)),
                    LirInst::ShiftRai(imm) => asmi!(Srai dst, src1, Imm(imm)),

                    // 整数演算 (imm 不使用)
                    LirInst::Add => asmi!(Add dst, src1, src2),
                    LirInst::Sub => asmi!(Sub dst, src1, src2),
                    LirInst::And => asmi!(And dst, src1, src2),
                    LirInst::Or  => asmi!(Or  dst, src1, src2),
                    LirInst::Xor => asmi!(Xor dst, src1, src2),
                    LirInst::ShiftL => asmi!(Sll dst, src1, src2),
                    LirInst::ShiftR => asmi!(Srl dst, src1, src2),
                    LirInst::ShiftRa => asmi!(Sra dst, src1, src2),

                    // 分岐
                    LirInst::Beq(imm) => asmi!(Beq dst, src1, src2, Imm(imm)),
                    LirInst::Bne(imm) => asmi!(Bne dst, src1, src2, Imm(imm)),
                    LirInst::Blt(imm) => asmi!(Blt dst, src1, src2, Imm(imm)),
                    LirInst::Ble(imm) => asmi!(Ble dst, src1, src2, Imm(imm)),
                    LirInst::Jmp(imm) => asmi!(Beq dst, 0, 0, Imm(imm)),
                    LirInst::JmpLabel(label) => {
                        let label = format!("local.{}", label);
                        asmi!(Beq dst, 0, 0, InstLabel(label))
                    }
                    LirInst::Call(label) => {
                        let label = format!("{}", label);
                        asmi!(Beq 1, 0, 0, InstLabel(label))
                    }
                };
                self.asm_inst.push(asm);
            }
            LirTree::Label { label } => {
                self.asm_inst.push(asmi!(LLabel label));
            }
        }
    }
}
