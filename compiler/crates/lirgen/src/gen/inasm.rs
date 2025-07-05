use sb_compiler_semcheck_hir::{InlineAsm, InlineAsmInst, InlineAsmOperand};
use sb_compiler_lirgen_ir::*;

use super::{GenContext, ZERO_REG};

pub fn lirgen_inline_asm<'src>(ctx: &mut GenContext<'src>, inline_asm: InlineAsm<'src>) -> LirBlock {
    let mut use_reg = |operand: &InlineAsmOperand<'src>| {
        match operand {
            InlineAsmOperand::Reg { num, .. } => *num as u32,
            InlineAsmOperand::Var { var } => {
                if let Some(reg) = ctx.ref_var_reg(&var) {
                    reg
                } else {
                    let reg = ctx.alloc_reg();
                    ctx.set_var_reg(var.clone(), reg);
                    reg
                }
            }
        }
    };

    let mut lirs = vec![];
    for inst in &inline_asm.insts {
        let lir = match inst {
            // I-形式
            InlineAsmInst::Addi { rd, rs1, imm, .. } => {
                lir!(RawAddi(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Subi { rd, rs1, imm, .. } => {
                lir!(RawSubi(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Jal { rd, rs1, imm, .. } => {
                lir!(RawJal(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Lw { rd, rs1, imm, .. } => {
                lir!(RawLw(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Lh { rd, rs1, imm, .. } => {
                lir!(RawLh(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Lb { rd, rs1, imm, .. } => {
                lir!(RawLb(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Lhu { rd, rs1, imm, .. } => {
                lir!(RawLhu(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Lbu { rd, rs1, imm, .. } => {
                lir!(RawLbu(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::In { rd, rs1, imm, .. } => {
                lir!(RawIn(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Andi { rd, rs1, imm, .. } => {
                lir!(RawAndi(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Ori { rd, rs1, imm, .. } => {
                lir!(RawOri(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Xori { rd, rs1, imm, .. } => {
                lir!(RawXori(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Srli { rd, rs1, imm, .. } => {
                lir!(RawSrli(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Srai { rd, rs1, imm, .. } => {
                lir!(RawSrai(*imm) use_reg(rd), use_reg(rs1))
            }
            InlineAsmInst::Slli { rd, rs1, imm, .. } => {
                lir!(RawSlli(*imm) use_reg(rd), use_reg(rs1))
            }

            // S-形式
            InlineAsmInst::Sw { rs1, rs2, imm, .. } => {
                lir!(RawSw(*imm) use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Sh { rs1, rs2, imm, .. } => {
                lir!(RawSh(*imm) use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Sb { rs1, rs2, imm, .. } => {
                lir!(RawSb(*imm) use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Isb { rs1, rs2, imm, .. } => {
                lir!(RawIsb(*imm) use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Out { rs1, rs2, imm, .. } => {
                lir!(RawOut(*imm) use_reg(rs1), use_reg(rs2))
            }

            // R-形式
            InlineAsmInst::Add { rd, rs1, rs2, .. } => {
                lir!(RawAdd use_reg(rd), use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Sub { rd, rs1, rs2, .. } => {
                lir!(RawSub use_reg(rd), use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::And { rd, rs1, rs2, .. } => {
                lir!(RawAnd use_reg(rd), use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Or { rd, rs1, rs2, .. } => {
                lir!(RawOr use_reg(rd), use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Xor { rd, rs1, rs2, .. } => {
                lir!(RawXor use_reg(rd), use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Srl { rd, rs1, rs2, .. } => {
                lir!(RawSrl use_reg(rd), use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Sra { rd, rs1, rs2, .. } => {
                lir!(RawSra use_reg(rd), use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Sll { rd, rs1, rs2, .. } => {
                lir!(RawSll use_reg(rd), use_reg(rs1), use_reg(rs2))
            }

            // B-形式
            InlineAsmInst::Beq { rd, rs1, rs2, imm, .. } => {
                lir!(RawBeq(*imm) use_reg(rd), use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Bne { rd, rs1, rs2, imm, .. } => {
                lir!(RawBne(*imm) use_reg(rd), use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Blt { rd, rs1, rs2, imm, .. } => {
                lir!(RawBlt(*imm) use_reg(rd), use_reg(rs1), use_reg(rs2))
            }
            InlineAsmInst::Ble { rd, rs1, rs2, imm, .. } => {
                lir!(RawBle(*imm) use_reg(rd), use_reg(rs1), use_reg(rs2))
            }
        };
        lirs.push(lir);
    }

    LirBlock::Single {
        result_reg: ZERO_REG,
        lirs,
    }
}
