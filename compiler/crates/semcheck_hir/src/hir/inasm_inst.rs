use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::{Typed, Type, I32};

use super::{InlineAsmOperand, SemCheck, Dep};

type Operand<'a> = InlineAsmOperand<'a>;

#[derive(Debug)]
pub enum InlineAsmInst<'src> {
    // I-形式
    Addi { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Subi { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Jal  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Lw   { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Lh   { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Lb   { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Lhu  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Lbu  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    In   { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Andi { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Ori  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Xori { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Srli { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Srai { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },
    Slli { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, imm: i32 },

    // S-形式
    Sw   { span: Span<'src>, rs1: Operand<'src>, rs2: Operand<'src>, imm: i32 },
    Sh   { span: Span<'src>, rs1: Operand<'src>, rs2: Operand<'src>, imm: i32 },
    Sb   { span: Span<'src>, rs1: Operand<'src>, rs2: Operand<'src>, imm: i32 },
    Isb  { span: Span<'src>, rs1: Operand<'src>, rs2: Operand<'src>, imm: i32 },
    Out  { span: Span<'src>, rs1: Operand<'src>, rs2: Operand<'src>, imm: i32 },

    // R-形式
    Add  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, rs2: Operand<'src> },
    Sub  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, rs2: Operand<'src> },
    And  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, rs2: Operand<'src> },
    Or   { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, rs2: Operand<'src> },
    Xor  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, rs2: Operand<'src> },
    Srl  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, rs2: Operand<'src> },
    Sra  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, rs2: Operand<'src> },
    Sll  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, rs2: Operand<'src> },

    // B-形式
    Beq  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, rs2: Operand<'src>, imm: i32 },
    Bne  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, rs2: Operand<'src>, imm: i32 },
    Blt  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, rs2: Operand<'src>, imm: i32 },
    Ble  { span: Span<'src>, rd: Operand<'src>, rs1: Operand<'src>, rs2: Operand<'src>, imm: i32 },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::InlineAsmInst<'src>> for InlineAsmInst<'src> {
    async fn check0(ctx: Dep<'_, 'src>, inst: ast::InlineAsmInst<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        macro_rules! check_reg_i {
            ($span:ident : $inst:ident $rd:expr, $rs1:expr, $imm:expr) => {
                Ok(InlineAsmInst::$inst {
                    span: $span,
                    rd: InlineAsmOperand::check(&mut *ctx, $rd).await?,
                    rs1: InlineAsmOperand::check(&mut *ctx, $rs1).await?,
                    imm: $imm,
                })
            };
        }

        macro_rules! check_reg_s {
            ($span:ident : $inst:ident $rs1:expr, $rs2:expr, $imm:expr) => {
                Ok(InlineAsmInst::$inst {
                    span: $span,
                    rs1: InlineAsmOperand::check(&mut *ctx, $rs1).await?,
                    rs2: InlineAsmOperand::check(&mut *ctx, $rs2).await?,
                    imm: $imm,
                })
            };
        }

        macro_rules! check_reg_r {
            ($span:ident : $inst:ident $rd:expr, $rs1:expr, $rs2:expr) => {
                Ok(InlineAsmInst::$inst {
                    span: $span,
                    rd: InlineAsmOperand::check(&mut *ctx, $rd).await?,
                    rs1: InlineAsmOperand::check(&mut *ctx, $rs1).await?,
                    rs2: InlineAsmOperand::check(&mut *ctx, $rs2).await?,
                })
            };
        }

        macro_rules! check_reg_b {
            ($span:ident : $inst:ident $rd:expr, $rs1:expr, $rs2:expr, $imm:expr) => {
                Ok(InlineAsmInst::$inst {
                    span: $span,
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
            Addi { span, rd, rs1, imm } => check_reg_i!(span : Addi rd, rs1, imm),
            Subi { span, rd, rs1, imm } => check_reg_i!(span : Subi rd, rs1, imm),
            Jal { span, rd, rs1, imm } => check_reg_i!(span : Jal rd, rs1, imm),
            Lw { span, rd, rs1, imm } => check_reg_i!(span : Lw rd, rs1, imm),
            Lh { span, rd, rs1, imm } => check_reg_i!(span : Lh rd, rs1, imm),
            Lb { span, rd, rs1, imm } => check_reg_i!(span : Lb rd, rs1, imm),
            Lhu { span, rd, rs1, imm } => check_reg_i!(span : Lhu rd, rs1, imm),
            Lbu { span, rd, rs1, imm } => check_reg_i!(span : Lbu rd, rs1, imm),
            In { span, rd, rs1, imm } => check_reg_i!(span : In rd, rs1, imm),
            Andi { span, rd, rs1, imm } => check_reg_i!(span : Andi rd, rs1, imm),
            Ori { span, rd, rs1, imm } => check_reg_i!(span : Ori rd, rs1, imm),
            Xori { span, rd, rs1, imm } => check_reg_i!(span : Xori rd, rs1, imm),
            Srli { span, rd, rs1, imm } => check_reg_i!(span : Srli rd, rs1, imm),
            Srai { span, rd, rs1, imm } => check_reg_i!(span : Srai rd, rs1, imm),
            Slli { span, rd, rs1, imm } => check_reg_i!(span : Slli rd, rs1, imm),

            // S-形式
            Sw { span, rs1, rs2, imm } => check_reg_s!(span : Sw rs1, rs2, imm),
            Sh { span, rs1, rs2, imm } => check_reg_s!(span : Sh rs1, rs2, imm),
            Sb { span, rs1, rs2, imm } => check_reg_s!(span : Sb rs1, rs2, imm),
            Isb { span, rs1, rs2, imm } => check_reg_s!(span : Isb rs1, rs2, imm),
            Out { span, rs1, rs2, imm } => check_reg_s!(span : Out rs1, rs2, imm),

            // R-形式
            Add { span, rd, rs1, rs2 } => check_reg_r!(span : Add rd, rs1, rs2),
            Sub { span, rd, rs1, rs2 } => check_reg_r!(span : Sub rd, rs1, rs2),
            And { span, rd, rs1, rs2 } => check_reg_r!(span : And rd, rs1, rs2),
            Or { span, rd, rs1, rs2 } => check_reg_r!(span : Or rd, rs1, rs2),
            Xor { span, rd, rs1, rs2 } => check_reg_r!(span : Xor rd, rs1, rs2),
            Srl { span, rd, rs1, rs2 } => check_reg_r!(span : Srl rd, rs1, rs2),
            Sra { span, rd, rs1, rs2 } => check_reg_r!(span : Sra rd, rs1, rs2),
            Sll { span, rd, rs1, rs2 } => check_reg_r!(span : Sll rd, rs1, rs2),

            // B-形式
            Beq { span, rd, rs1, rs2, imm } => check_reg_b!(span : Beq rd, rs1, rs2, imm),
            Bne { span, rd, rs1, rs2, imm } => check_reg_b!(span : Bne rd, rs1, rs2, imm),
            Blt { span, rd, rs1, rs2, imm } => check_reg_b!(span : Blt rd, rs1, rs2, imm),
            Ble { span, rd, rs1, rs2, imm } => check_reg_b!(span : Ble rd, rs1, rs2, imm),
        }
    }
}

impl<'src> Spanned<'src> for InlineAsmInst<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            InlineAsmInst::Addi { span, .. } => *span,
            InlineAsmInst::Subi { span, .. } => *span,
            InlineAsmInst::Jal { span, .. } => *span,
            InlineAsmInst::Lw { span, .. } => *span,
            InlineAsmInst::Lh { span, .. } => *span,
            InlineAsmInst::Lb { span, .. } => *span,
            InlineAsmInst::Lhu { span, .. } => *span,
            InlineAsmInst::Lbu { span, .. } => *span,
            InlineAsmInst::In { span, .. } => *span,
            InlineAsmInst::Andi { span, .. } => *span,
            InlineAsmInst::Ori { span, .. } => *span,
            InlineAsmInst::Xori { span, .. } => *span,
            InlineAsmInst::Srli { span, .. } => *span,
            InlineAsmInst::Srai { span, .. } => *span,
            InlineAsmInst::Slli { span, .. } => *span,
            InlineAsmInst::Sw { span, .. } => *span,
            InlineAsmInst::Sh { span, .. } => *span,
            InlineAsmInst::Sb { span, .. } => *span,
            InlineAsmInst::Isb { span, .. } => *span,
            InlineAsmInst::Out { span, .. } => *span,
            InlineAsmInst::Add { span, .. } => *span,
            InlineAsmInst::Sub { span, .. } => *span,
            InlineAsmInst::And { span, .. } => *span,
            InlineAsmInst::Or  { span, .. } => *span,
            InlineAsmInst::Xor { span, .. } => *span,
            InlineAsmInst::Srl { span, .. } => *span,
            InlineAsmInst::Sra { span, .. } => *span,
            InlineAsmInst::Sll { span, .. } => *span,
            InlineAsmInst::Beq { span, .. } => *span,
            InlineAsmInst::Bne { span, .. } => *span,
            InlineAsmInst::Blt { span, .. } => *span,
            InlineAsmInst::Ble { span, .. } => *span,
        }
    }
}

impl Typed for InlineAsmInst<'_> {
    fn ty(&self) -> Arc<Type> {
        I32.ty()
    }
}