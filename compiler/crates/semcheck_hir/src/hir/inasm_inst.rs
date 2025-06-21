use sb_compiler_parse_ast as ast;

use super::{InlineAsmOperand, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum InlineAsmInst {
    // I-形式
    Addi { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Subi { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Jal  { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Lw   { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Lh   { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Lb   { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Lhu  { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Lbu  { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    In   { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Andi { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Ori  { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Xori { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Srli { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Srai { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },
    Slli { rd: InlineAsmOperand, rs1: InlineAsmOperand, imm: i32 },

    // S-形式
    Sw   { rs1: InlineAsmOperand, rs2: InlineAsmOperand, imm: i32 },
    Sh   { rs1: InlineAsmOperand, rs2: InlineAsmOperand, imm: i32 },
    Sb   { rs1: InlineAsmOperand, rs2: InlineAsmOperand, imm: i32 },
    Isb  { rs1: InlineAsmOperand, rs2: InlineAsmOperand, imm: i32 },
    Out  { rs1: InlineAsmOperand, rs2: InlineAsmOperand, imm: i32 },

    // R-形式
    Add  { rd: InlineAsmOperand, rs1: InlineAsmOperand, rs2: InlineAsmOperand },
    Sub  { rd: InlineAsmOperand, rs1: InlineAsmOperand, rs2: InlineAsmOperand },
    And  { rd: InlineAsmOperand, rs1: InlineAsmOperand, rs2: InlineAsmOperand },
    Or   { rd: InlineAsmOperand, rs1: InlineAsmOperand, rs2: InlineAsmOperand },
    Xor  { rd: InlineAsmOperand, rs1: InlineAsmOperand, rs2: InlineAsmOperand },
    Srl  { rd: InlineAsmOperand, rs1: InlineAsmOperand, rs2: InlineAsmOperand },
    Sra  { rd: InlineAsmOperand, rs1: InlineAsmOperand, rs2: InlineAsmOperand },
    Sll  { rd: InlineAsmOperand, rs1: InlineAsmOperand, rs2: InlineAsmOperand },

    // B-形式
    Beq  { rd: InlineAsmOperand, rs1: InlineAsmOperand, rs2: InlineAsmOperand, imm: i32 },
    Bne  { rd: InlineAsmOperand, rs1: InlineAsmOperand, rs2: InlineAsmOperand, imm: i32 },
    Blt  { rd: InlineAsmOperand, rs1: InlineAsmOperand, rs2: InlineAsmOperand, imm: i32 },
    Ble  { rd: InlineAsmOperand, rs1: InlineAsmOperand, rs2: InlineAsmOperand, imm: i32 },
}

impl<'input> SemCheckFrom<Dep<'_>, ast::InlineAsmInst<'input>> for InlineAsmInst {
    async fn check0(ctx: Dep<'_>, inst: ast::InlineAsmInst<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        macro_rules! check_reg_i {
            ($inst:ident $rd:expr, $rs1:expr, $imm:expr) => {
                Ok(InlineAsmInst::$inst {
                    rd: InlineAsmOperand::check(&mut *ctx, $rd).await?,
                    rs1: InlineAsmOperand::check(&mut *ctx, $rs1).await?,
                    imm: $imm,
                })
            };
        }

        macro_rules! check_reg_s {
            ($inst:ident $rs1:expr, $rs2:expr, $imm:expr) => {
                Ok(InlineAsmInst::$inst {
                    rs1: InlineAsmOperand::check(&mut *ctx, $rs1).await?,
                    rs2: InlineAsmOperand::check(&mut *ctx, $rs2).await?,
                    imm: $imm,
                })
            };
        }

        macro_rules! check_reg_r {
            ($inst:ident $rd:expr, $rs1:expr, $rs2:expr) => {
                Ok(InlineAsmInst::$inst {
                    rd: InlineAsmOperand::check(&mut *ctx, $rd).await?,
                    rs1: InlineAsmOperand::check(&mut *ctx, $rs1).await?,
                    rs2: InlineAsmOperand::check(&mut *ctx, $rs2).await?,
                })
            };
        }

        macro_rules! check_reg_b {
            ($inst:ident $rd:expr, $rs1:expr, $rs2:expr, $imm:expr) => {
                Ok(InlineAsmInst::$inst {
                    rd: InlineAsmOperand::check(&mut *ctx, $rd).await?,
                    rs1: InlineAsmOperand::check(&mut *ctx, $rs1).await?,
                    rs2: InlineAsmOperand::check(&mut *ctx, $rs2).await?,
                    imm: $imm,
                })
            };
        }

        use ast::InlineAsmInst::*;
        match inst {
            // I-形式
            Addi { rd, rs1, imm } => check_reg_i!(Addi rd, rs1, imm),
            Subi { rd, rs1, imm } => check_reg_i!(Subi rd, rs1, imm),
            Jal { rd, rs1, imm } => check_reg_i!(Jal rd, rs1, imm),
            Lw { rd, rs1, imm } => check_reg_i!(Lw rd, rs1, imm),
            Lh { rd, rs1, imm } => check_reg_i!(Lh rd, rs1, imm),
            Lb { rd, rs1, imm } => check_reg_i!(Lb rd, rs1, imm),
            Lhu { rd, rs1, imm } => check_reg_i!(Lhu rd, rs1, imm),
            Lbu { rd, rs1, imm } => check_reg_i!(Lbu rd, rs1, imm),
            In { rd, rs1, imm } => check_reg_i!(In rd, rs1, imm),
            Andi { rd, rs1, imm } => check_reg_i!(Andi rd, rs1, imm),
            Ori { rd, rs1, imm } => check_reg_i!(Ori rd, rs1, imm),
            Xori { rd, rs1, imm } => check_reg_i!(Xori rd, rs1, imm),
            Srli { rd, rs1, imm } => check_reg_i!(Srli rd, rs1, imm),
            Srai { rd, rs1, imm } => check_reg_i!(Srai rd, rs1, imm),
            Slli { rd, rs1, imm } => check_reg_i!(Slli rd, rs1, imm),

            // S-形式
            Sw { rs1, rs2, imm } => check_reg_s!(Sw rs1, rs2, imm),
            Sh { rs1, rs2, imm } => check_reg_s!(Sh rs1, rs2, imm),
            Sb { rs1, rs2, imm } => check_reg_s!(Sb rs1, rs2, imm),
            Isb { rs1, rs2, imm } => check_reg_s!(Isb rs1, rs2, imm),
            Out { rs1, rs2, imm } => check_reg_s!(Out rs1, rs2, imm),

            // R-形式
            Add { rd, rs1, rs2 } => check_reg_r!(Add rd, rs1, rs2),
            Sub { rd, rs1, rs2 } => check_reg_r!(Sub rd, rs1, rs2),
            And { rd, rs1, rs2 } => check_reg_r!(And rd, rs1, rs2),
            Or { rd, rs1, rs2 } => check_reg_r!(Or rd, rs1, rs2),
            Xor { rd, rs1, rs2 } => check_reg_r!(Xor rd, rs1, rs2),
            Srl { rd, rs1, rs2 } => check_reg_r!(Srl rd, rs1, rs2),
            Sra { rd, rs1, rs2 } => check_reg_r!(Sra rd, rs1, rs2),
            Sll { rd, rs1, rs2 } => check_reg_r!(Sll rd, rs1, rs2),

            // B-形式
            Beq { rd, rs1, rs2, imm } => check_reg_b!(Beq rd, rs1, rs2, imm),
            Bne { rd, rs1, rs2, imm } => check_reg_b!(Bne rd, rs1, rs2, imm),
            Blt { rd, rs1, rs2, imm } => check_reg_b!(Blt rd, rs1, rs2, imm),
            Ble { rd, rs1, rs2, imm } => check_reg_b!(Ble rd, rs1, rs2, imm),
        }
    }
}
