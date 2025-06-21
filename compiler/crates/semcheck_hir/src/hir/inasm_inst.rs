use sb_compiler_parse_ast as ast;

use super::{InlineAsmReg, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum InlineAsmInst {
    // I-形式
    Addi { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Subi { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Jal  { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Lw   { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Lh   { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Lb   { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Lhu  { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Lbu  { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    In   { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Andi { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Ori  { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Xori { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Srli { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Srai { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },
    Slli { rd: InlineAsmReg, rs1: InlineAsmReg, imm: i32 },

    // S-形式
    Sw   { rs1: InlineAsmReg, rs2: InlineAsmReg, imm: i32 },
    Sh   { rs1: InlineAsmReg, rs2: InlineAsmReg, imm: i32 },
    Sb   { rs1: InlineAsmReg, rs2: InlineAsmReg, imm: i32 },
    Isb  { rs1: InlineAsmReg, rs2: InlineAsmReg, imm: i32 },
    Out  { rs1: InlineAsmReg, rs2: InlineAsmReg, imm: i32 },

    // R-形式
    Add  { rd: InlineAsmReg, rs1: InlineAsmReg, rs2: InlineAsmReg },
    Sub  { rd: InlineAsmReg, rs1: InlineAsmReg, rs2: InlineAsmReg },
    And  { rd: InlineAsmReg, rs1: InlineAsmReg, rs2: InlineAsmReg },
    Or   { rd: InlineAsmReg, rs1: InlineAsmReg, rs2: InlineAsmReg },
    Xor  { rd: InlineAsmReg, rs1: InlineAsmReg, rs2: InlineAsmReg },
    Srl  { rd: InlineAsmReg, rs1: InlineAsmReg, rs2: InlineAsmReg },
    Sra  { rd: InlineAsmReg, rs1: InlineAsmReg, rs2: InlineAsmReg },
    Sll  { rd: InlineAsmReg, rs1: InlineAsmReg, rs2: InlineAsmReg },

    // B-形式
    Beq  { rd: InlineAsmReg, rs1: InlineAsmReg, rs2: InlineAsmReg, imm: i32 },
    Bne  { rd: InlineAsmReg, rs1: InlineAsmReg, rs2: InlineAsmReg, imm: i32 },
    Blt  { rd: InlineAsmReg, rs1: InlineAsmReg, rs2: InlineAsmReg, imm: i32 },
    Ble  { rd: InlineAsmReg, rs1: InlineAsmReg, rs2: InlineAsmReg, imm: i32 },
}

impl<'input> SemCheckFrom<Dep<'_>, ast::InlineAsmInst<'input>> for InlineAsmInst {
    async fn check0(ctx: Dep<'_>, inst: ast::InlineAsmInst<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        macro_rules! check_reg_i {
            ($inst:ident $rd:expr, $rs1:expr, $imm:expr) => {
                Ok(InlineAsmInst::$inst {
                    rd: InlineAsmReg::check(ctx, $rd).await?,
                    rs1: InlineAsmReg::check(ctx, $rs1).await?,
                    imm: $imm,
                })
            };
        }

        macro_rules! check_reg_s {
            ($inst:ident $rs1:expr, $rs2:expr, $imm:expr) => {
                Ok(InlineAsmInst::$inst {
                    rs1: InlineAsmReg::check(ctx, $rs1).await?,
                    rs2: InlineAsmReg::check(ctx, $rs2).await?,
                    imm: $imm,
                })
            };
        }

        macro_rules! check_reg_r {
            ($inst:ident $rd:expr, $rs1:expr, $rs2:expr) => {
                Ok(InlineAsmInst::$inst {
                    rd: InlineAsmReg::check(ctx, $rd).await?,
                    rs1: InlineAsmReg::check(ctx, $rs1).await?,
                    rs2: InlineAsmReg::check(ctx, $rs2).await?,
                })
            };
        }

        macro_rules! check_reg_b {
            ($inst:ident $rd:expr, $rs1:expr, $rs2:expr, $imm:expr) => {
                Ok(InlineAsmInst::$inst {
                    rd: InlineAsmReg::check(ctx, $rd).await?,
                    rs1: InlineAsmReg::check(ctx, $rs1).await?,
                    rs2: InlineAsmReg::check(ctx, $rs2).await?,
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
