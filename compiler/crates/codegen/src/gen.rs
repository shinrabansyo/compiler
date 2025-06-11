use std::u32;

use sb_linker::obj::inst::*;
use sb_linker::obj::{inst, Object};

use sb_compiler_lirgen_ir::{LirTopElem, LirBlock, LirInst};

use crate::reg_mapping::{RegMap, MapTo};

pub fn gen_inst(lir: LirTopElem, reg_map: RegMap) -> Object {
    match lir {
        LirTopElem::Function { namespace, name, body, .. } => {
            let name = format!("{}.{}", namespace, name);
            let inst = InstGenerator::gen(body, reg_map);
            Object::new(name, inst)
        },
    }
}

struct InstGenerator {
    reg_map: RegMap,
    asm_inst: Vec<Inst>,
}

impl InstGenerator {
    fn gen(lir_block: LirBlock, reg_map: RegMap) -> Vec<Inst> {
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
                let pro = (11 + self.reg_map.spilled_regs()) * 4;
                self.asm_inst.extend([
                    inst!(Subi 2, 2, Imm(4)),   // subi r2 = r2, 4     : r2 = フレームの先頭を指すアドレス(仮)
                    inst!(Sw 2, 3, 0),          // sw r2[0] = r3       : スタックポインタの退避
                    inst!(Addi 3, 2, Imm(0)),   // addi r3 = r2, 0     : r3 = 退避領域のベースアドレス
                    inst!(Sw 3, 1, -4),         // sw r3[-4] = r1      : 規約で定められているレジスタの退避
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
                    inst!(Subi 4, 3, Imm(48)),  // subi r4 = r3, 48    : r4 = 溢れ領域のベースアドレス
                    inst!(Subi 2, 2, Imm(pro)), // subi r2 = r2, pro   : r2 = フレームの先頭を指すアドレス
                ]);
            }
            LirBlock::Inst { inst: LirInst::FnEpilogue, .. } => {
                let epi = (1 + 11 + self.reg_map.spilled_regs()) * 4;
                self.asm_inst.extend([
                    inst!(Label u32::MAX),
                    inst!(Lw 1, 3, -4),         // lw r1 = r3[-4]      : 規約で定められているレジスタの復元
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
                    inst!(Lw 3, 3, 0),          // lw r3 = r3[0]       : r3 の復元
                    inst!(Addi 2, 2, Imm(epi)), // addi r2 = r2, epi   : r2 の復元
                    inst!(Jal 0, 1, 0),         // jal r0, r1[0]       : リターン
                ]);
            }
            LirBlock::Inst { inst, dst, src1, src2 } => {
                // 計算に使用するレジスタを取得
                let (dst, dst_needs_wback) = match self.reg_map.get(&dst) {
                    MapTo::Reg(reg) => (reg, None),
                    MapTo::Stack(addr) => (5, Some(addr)),
                };
                let src1 = match self.reg_map.get(&src1) {
                    MapTo::Reg(reg) => reg,
                    MapTo::Stack(addr) => {
                        self.asm_inst.push(inst!(Lw 6, 4, -(addr as i32) * 4));
                        6
                    }
                };
                let src2 = match self.reg_map.get(&src2) {
                    MapTo::Reg(reg) => reg,
                    MapTo::Stack(addr) => {
                        self.asm_inst.push(inst!(Lw 7, 4, -(addr as i32) * 4));
                        7
                    }
                };

                // 命令変換
                let inst = match inst {
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

                    // インラインアセンブリ (I-形式)
                    LirInst::RawAddi(imm) => inst!(Addi dst, src1, Imm(imm)),
                    LirInst::RawSubi(imm) => inst!(Subi dst, src1, Imm(imm)),
                    LirInst::RawJal(imm) => inst!(Jal dst, src1, imm),
                    LirInst::RawLw(imm) => inst!(Lw dst, src1, imm),
                    LirInst::RawLh(imm) => inst!(Lh dst, src1, imm),
                    LirInst::RawLb(imm) => inst!(Lb dst, src1, imm),
                    LirInst::RawLhu(imm) => inst!(Lhu dst, src1, imm),
                    LirInst::RawLbu(imm) => inst!(Lbu dst, src1, imm),
                    LirInst::RawAndi(imm) => inst!(Andi dst, src1, Imm(imm)),
                    LirInst::RawOri(imm) => inst!(Ori dst, src1, Imm(imm)),
                    LirInst::RawXori(imm) => inst!(Xori dst, src1, Imm(imm)),
                    LirInst::RawSrli(imm) => inst!(Srli dst, src1, Imm(imm)),
                    LirInst::RawSrai(imm) => inst!(Srai dst, src1, Imm(imm)),
                    LirInst::RawSlli(imm) => inst!(Slli dst, src1, Imm(imm)),

                    // インラインアセンブリ (S-形式)
                    LirInst::RawSw(imm) => inst!(Sw dst, src1, imm),
                    LirInst::RawSh(imm) => inst!(Sh dst, src1, imm),
                    LirInst::RawSb(imm) => inst!(Sb dst, src1, imm),
                    LirInst::RawIsb(_) => todo!(),

                    // インラインアセンブリ (R-形式)
                    LirInst::RawAdd => inst!(Add dst, src1, src2),
                    LirInst::RawSub => inst!(Sub dst, src1, src2),
                    LirInst::RawAnd => inst!(And dst, src1, src2),
                    LirInst::RawOr  => inst!(Or  dst, src1, src2),
                    LirInst::RawXor => inst!(Xor dst, src1, src2),
                    LirInst::RawSrl => inst!(Srl dst, src1, src2),
                    LirInst::RawSra => inst!(Sra dst, src1, src2),
                    LirInst::RawSll => inst!(Sll dst, src1, src2),

                    // インラインアセンブリ (B-形式)
                    LirInst::RawBeq(imm) => inst!(Beq dst, src1, src2, Imm(imm)),
                    LirInst::RawBne(imm) => inst!(Bne dst, src1, src2, Imm(imm)),
                    LirInst::RawBlt(imm) => inst!(Blt dst, src1, src2, Imm(imm)),
                    LirInst::RawBle(imm) => inst!(Ble dst, src1, src2, Imm(imm)),
                };
                self.asm_inst.push(inst);

                // スタックへの書き戻し
                if let Some(addr) = dst_needs_wback {
                    self.asm_inst.push(inst!(Sw 4, 5, -(addr as i32) * 4));
                }
            }
            LirBlock::Label { label } => {
                self.asm_inst.push(inst!(Label label));
            }
        }
    }
}
