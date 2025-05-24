use std::collections::HashMap;
use std::u32;

use sb_linker::obj::inst::*;
use sb_linker::obj::{inst, Object};

use sb_compiler_lirgen_ir::{LirTopElem, LirBlock, LirInst};

pub fn gen_inst(lir: LirTopElem, reg_map: HashMap<u32, u8>) -> Object {
    match lir {
        LirTopElem::Function { namespace, name, body, .. } => {
            let name = format!("{}.{}", namespace, name);
            let inst = InstGenerator::gen(body, reg_map);
            Object::new(name, inst)
        },
    }
}

struct InstGenerator {
    reg_map: HashMap<u32, u8>,
    asm_inst: Vec<Inst>,
}

impl InstGenerator {
    fn gen(lir_block: LirBlock, reg_map: HashMap<u32, u8>) -> Vec<Inst> {
        let mut generator = InstGenerator {
            reg_map,
            asm_inst: Vec::new(),
        };
        generator.gen_recursive(lir_block);
        generator.asm_inst
    }

    fn gen_recursive(&mut self, lir_block: LirBlock) {
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
                    inst!(Subi 2, 2, Imm(4)),   // subi r2 = r2, 4
                    inst!(Sw 2, 3, 0),          // sw r2[0] = r3
                    inst!(Addi 3, 2, Imm(0)),   // addi r3 = r2, 0
                    inst!(Sw 3, 1, -4),         // sw r3[-4] = r1
                    inst!(Sw 3, 20, -8),        // sw r3[-8] = r20
                    inst!(Sw 3, 21, -12),       // sw r3[-12] = r21
                    inst!(Sw 3, 22, -16),       // sw r3[-16] = r22
                    inst!(Sw 3, 23, -20),       // sw r3[-20] = r23
                    inst!(Sw 3, 24, -24),       // sw r3[-24] = r24
                    inst!(Sw 3, 25, -28),       // sw r3[-28] = r25
                    inst!(Sw 3, 26, -32),       // sw r3[-32] = r26
                    inst!(Sw 3, 27, -36),       // sw r3[-36] = r27
                    inst!(Sw 3, 28, -40),       // sw r3[-40] = r28
                    inst!(Sw 3, 29, -44),       // sw r3[-44] = r29
                    inst!(Subi 2, 2, Imm(44)),  // subi r2 = r2, 99
                ]);
            }
            LirBlock::Inst { inst: LirInst::FnEpilogue, .. } => {
                self.asm_inst.extend([
                    inst!(Label u32::MAX),
                    inst!(Lw 1, 3, -4),         // lw r1 = r3[-4]
                    inst!(Lw 20, 3, -8),        // lw r20 = r3[-8]
                    inst!(Lw 21, 3, -12),       // lw r21 = r3[-12]
                    inst!(Lw 22, 3, -16),       // lw r22 = r3[-16]
                    inst!(Lw 23, 3, -20),       // lw r23 = r3[-20]
                    inst!(Lw 24, 3, -24),       // lw r24 = r3[-24]
                    inst!(Lw 25, 3, -28),       // lw r25 = r3[-28]
                    inst!(Lw 26, 3, -32),       // lw r26 = r3[-32]
                    inst!(Lw 27, 3, -36),       // lw r27 = r3[-36]
                    inst!(Lw 28, 3, -40),       // lw r28 = r3[-40]
                    inst!(Lw 29, 3, -44),       // lw r29 = r3[-44]
                    inst!(Lw 3, 3, 0),          // lw r3 = r3[0]
                    inst!(Addi 2, 2, Imm(48)),  // addi r2 = r2, 4
                    inst!(Jal 0, 1, 0),         // jal r0, r1[0]
                ]);
            }
            LirBlock::Inst { inst, dst, src1, src2 } => {
                let dst = *self.reg_map.get(&dst).unwrap_or(&(dst as u8));
                let src1 = *self.reg_map.get(&src1).unwrap_or(&(src1 as u8));
                let src2 = *self.reg_map.get(&src2).unwrap_or(&(src2 as u8));

                let asm = match inst {
                    // Nop
                    LirInst::Nop => inst!(Add 0, 12, 4),

                    // 整数演算 (imm 使用)
                    LirInst::Li(imm) => inst!(Addi dst, 0, Imm(imm)),
                    LirInst::Addi(imm) => inst!(Addi dst, src1, Imm(imm)),
                    LirInst::Subi(imm) => inst!(Subi dst, src1, Imm(imm)),
                    LirInst::Andi(imm) => inst!(Andi dst, src1, Imm(imm)),
                    LirInst::Ori(imm) => inst!(Ori dst, src1, Imm(imm)),
                    LirInst::Xori(imm) => inst!(Xori dst, src1, Imm(imm)),
                    LirInst::ShiftLi(imm) => inst!(Slli dst, src1, Imm(imm)),
                    LirInst::ShiftRi(imm) => inst!(Srli dst, src1, Imm(imm)),
                    LirInst::ShiftRai(imm) => inst!(Srai dst, src1, Imm(imm)),

                    // 整数演算 (imm 不使用)
                    LirInst::Add => inst!(Add dst, src1, src2),
                    LirInst::Sub => inst!(Sub dst, src1, src2),
                    LirInst::And => inst!(And dst, src1, src2),
                    LirInst::Or  => inst!(Or  dst, src1, src2),
                    LirInst::Xor => inst!(Xor dst, src1, src2),
                    LirInst::ShiftL => inst!(Sll dst, src1, src2),
                    LirInst::ShiftR => inst!(Srl dst, src1, src2),
                    LirInst::ShiftRa => inst!(Sra dst, src1, src2),

                    // 分岐
                    LirInst::Beq(imm) => inst!(Beq dst, src1, src2, Imm(imm)),
                    LirInst::Bne(imm) => inst!(Bne dst, src1, src2, Imm(imm)),
                    LirInst::Blt(imm) => inst!(Blt dst, src1, src2, Imm(imm)),
                    LirInst::Ble(imm) => inst!(Ble dst, src1, src2, Imm(imm)),
                    LirInst::Jmp(imm) => inst!(Beq dst, 0, 0, Imm(imm)),
                    LirInst::JmpLabel(label) => inst!(Beq dst, 0, 0, InstLabel(label)),
                    LirInst::Call(label) => inst!(Beq 1, 0, 0, Function(label)),

                    // 関数
                    LirInst::FnPrologue => unreachable!(),
                    LirInst::FnEpilogue => unreachable!(),
                    LirInst::FnReturn => inst!(Beq 0, 0, 0, InstLabel(u32::MAX)),
                };
                self.asm_inst.push(asm);
            }
            LirBlock::Label { label } => {
                self.asm_inst.push(inst!(Label label));
            }
        }
    }
}
