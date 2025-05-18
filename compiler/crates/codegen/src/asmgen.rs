use std::collections::HashMap;

use sb_compiler_lirgen_ir::{LirBlock, LirInst};
use sb_compiler_codegen_asm::inst::*;
use sb_compiler_codegen_asm::{asmi, Asm};

pub fn asmgen(lir_block: &LirBlock, reg_map: HashMap<u32, u8>) -> Asm {
    let asm_inst = AsmInstGenerator::gen(lir_block, reg_map);
    Asm::from(asm_inst)
}

struct AsmInstGenerator {
    reg_map: HashMap<u32, u8>,
    asm_inst: Vec<AsmInst>,
}

impl AsmInstGenerator {
    fn gen(lir_block: &LirBlock, reg_map: HashMap<u32, u8>) -> Vec<AsmInst> {
        let mut generator = AsmInstGenerator {
            reg_map,
            asm_inst: Vec::new(),
        };
        generator.gen_recursive(lir_block);
        generator.asm_inst.push(asmi!(Add 0, 12, 4));

        generator.asm_inst
    }

    fn gen_recursive(&mut self, lir_block: &LirBlock) {
        match lir_block {
            LirBlock::Single { lirs, .. } => {
                for lir in lirs {
                    self.gen_recursive(lir);
                }
            }
            LirBlock::Multiple { lirs } => {
                for lir in lirs {
                    self.gen_recursive(lir);
                }
            }
            LirBlock::Inst { inst: LirInst::FnPrologue, .. } => {
                self.asm_inst.extend([
                    asmi!(Subi 2, 2, Imm(4)),   // subi r2 = r2, 4
                    asmi!(Sw 2, 3, 0),          // sw r2[0] = r3
                    asmi!(Addi 3, 2, Imm(0)),   // addi r3 = r2, 0
                    asmi!(Subi 2, 2, Imm(44)),  // subi r2 = r2, 99
                    asmi!(Sw 3, 1, -4),         // sw r3[-4] = r1
                    asmi!(Sw 3, 20, -8),        // sw r3[-8] = r20
                    asmi!(Sw 3, 21, -12),       // sw r3[-12] = r21
                    asmi!(Sw 3, 22, -16),       // sw r3[-16] = r22
                    asmi!(Sw 3, 23, -20),       // sw r3[-20] = r23
                    asmi!(Sw 3, 24, -24),       // sw r3[-24] = r24
                    asmi!(Sw 3, 25, -28),       // sw r3[-28] = r25
                    asmi!(Sw 3, 26, -32),       // sw r3[-32] = r26
                    asmi!(Sw 3, 27, -36),       // sw r3[-36] = r27
                    asmi!(Sw 3, 28, -40),       // sw r3[-40] = r28
                    asmi!(Sw 3, 29, -44),       // sw r3[-44] = r29
                ]);
            }
            LirBlock::Inst { inst: LirInst::FnEpilogue, .. } => {
                self.asm_inst.extend([
                    asmi!(Lw 1, 3, -4),         // lw r1 = r3[-4]
                    asmi!(Lw 20, 3, -8),        // lw r20 = r3[-8]
                    asmi!(Lw 21, 3, -12),       // lw r21 = r3[-12]
                    asmi!(Lw 22, 3, -16),       // lw r22 = r3[-16]
                    asmi!(Lw 23, 3, -20),       // lw r23 = r3[-20]
                    asmi!(Lw 24, 3, -24),       // lw r24 = r3[-24]
                    asmi!(Lw 25, 3, -28),       // lw r25 = r3[-28]
                    asmi!(Lw 26, 3, -32),       // lw r26 = r3[-32]
                    asmi!(Lw 27, 3, -36),       // lw r27 = r3[-36]
                    asmi!(Lw 28, 3, -40),       // lw r28 = r3[-40]
                    asmi!(Lw 29, 3, -44),       // lw r29 = r3[-44]
                    asmi!(Lw 3, 3, 0),          // lw r3 = r3[0]
                    asmi!(Addi 2, 2, Imm(48)),  // addi r2 = r2, 4
                    asmi!(Beq 0, 0, 0, Imm(0)), // beq r0, (r0, r0) -> 0
                ]);
            }
            LirBlock::Inst { inst, dst, src1, src2 } => {
                let dst = *self.reg_map.get(&dst).unwrap_or(&0);
                let src1 = *self.reg_map.get(&src1).unwrap_or(&0);
                let src2 = *self.reg_map.get(&src2).unwrap_or(&0);

                let asm = match inst {
                    // Nop
                    LirInst::Nop => asmi!(Add 0, 0, 0),

                    // 整数演算 (imm 使用)
                    LirInst::Li(imm) => asmi!(Addi dst, 0, Imm(*imm)),
                    LirInst::Addi(imm) => asmi!(Addi dst, src1, Imm(*imm)),
                    LirInst::Subi(imm) => asmi!(Subi dst, src1, Imm(*imm)),
                    LirInst::Andi(imm) => asmi!(Andi dst, src1, Imm(*imm)),
                    LirInst::Ori(imm) => asmi!(Ori dst, src1, Imm(*imm)),
                    LirInst::Xori(imm) => asmi!(Xori dst, src1, Imm(*imm)),
                    LirInst::ShiftLi(imm) => asmi!(Slli dst, src1, Imm(*imm)),
                    LirInst::ShiftRi(imm) => asmi!(Srli dst, src1, Imm(*imm)),
                    LirInst::ShiftRai(imm) => asmi!(Srai dst, src1, Imm(*imm)),

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
                    LirInst::Beq(imm) => asmi!(Beq dst, src1, src2, Imm(*imm)),
                    LirInst::Bne(imm) => asmi!(Bne dst, src1, src2, Imm(*imm)),
                    LirInst::Blt(imm) => asmi!(Blt dst, src1, src2, Imm(*imm)),
                    LirInst::Ble(imm) => asmi!(Ble dst, src1, src2, Imm(*imm)),
                    LirInst::Jmp(imm) => asmi!(Beq dst, 0, 0, Imm(*imm)),
                    LirInst::JmpLabel(label) => {
                        let label = format!("local.{}", label);
                        asmi!(Beq dst, 0, 0, InstLabel(label))
                    }
                    LirInst::Call(label) => {
                        let label = format!("{}", label);
                        asmi!(Beq 1, 0, 0, InstLabel(label))
                    }

                    _ => unreachable!(),
                };
                self.asm_inst.push(asm);
            }
            LirBlock::LLabel { label } => {
                self.asm_inst.push(asmi!(LLabel *label));
            }
            LirBlock::GLabel { label } => {
                self.asm_inst.push(asmi!(GLabel label.clone()));
            }
        }
    }
}
