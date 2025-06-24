use sb_compiler_parse_cst::{Span, Spanned};

use super::{InlineAsmOperandL, InlineAsmOperandR, Visitor};

type OperandL<'a> = InlineAsmOperandL<'a>;
type OperandR<'a> = InlineAsmOperandR<'a>;

#[derive(Debug)]
pub enum InlineAsmInst<'src> {
    // I-形式
    Addi { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Subi { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Jal  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Lw   { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Lh   { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Lb   { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Lhu  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Lbu  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    In   { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Andi { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Ori  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Xori { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Srli { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Srai { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },
    Slli { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, imm: i32 },

    // S-形式
    Sw   { span: Span<'src>, rs1: OperandR<'src>, rs2: OperandR<'src>, imm: i32 },
    Sh   { span: Span<'src>, rs1: OperandR<'src>, rs2: OperandR<'src>, imm: i32 },
    Sb   { span: Span<'src>, rs1: OperandR<'src>, rs2: OperandR<'src>, imm: i32 },
    Isb  { span: Span<'src>, rs1: OperandR<'src>, rs2: OperandR<'src>, imm: i32 },
    Out  { span: Span<'src>, rs1: OperandR<'src>, rs2: OperandR<'src>, imm: i32 },

    // R-形式
    Add  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, rs2: OperandR<'src> },
    Sub  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, rs2: OperandR<'src> },
    And  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, rs2: OperandR<'src> },
    Or   { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, rs2: OperandR<'src> },
    Xor  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, rs2: OperandR<'src> },
    Srl  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, rs2: OperandR<'src> },
    Sra  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, rs2: OperandR<'src> },
    Sll  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, rs2: OperandR<'src> },

    // B-形式
    Beq  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, rs2: OperandR<'src>, imm: i32 },
    Bne  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, rs2: OperandR<'src>, imm: i32 },
    Blt  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, rs2: OperandR<'src>, imm: i32 },
    Ble  { span: Span<'src>, rd: OperandL<'src>, rs1: OperandR<'src>, rs2: OperandR<'src>, imm: i32 },
}

impl<'src> From<Visitor<'src>> for InlineAsmInst<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        macro_rules! parse_i {
            (Jal $visitor:ident) => {{
                InlineAsmInst::Jal {
                    span: $visitor.span(),
                    rd: $visitor.expect_node::<OperandL>(),
                    rs1: $visitor.expect_node::<OperandR>(),
                    imm: $visitor.expect_leaf().1.as_str().parse().unwrap(),
                }
            }};

            ($inst:ident $visitor:ident) => {{
                let span = $visitor.span();
                let rd = $visitor.expect_node::<OperandL>();
                let _ = $visitor.expect_leaf();  // '='
                let rs1 = $visitor.expect_node::<OperandR>();
                let imm = $visitor.expect_leaf().1.as_str().parse().unwrap();
                InlineAsmInst::$inst { span, rd, rs1, imm }
            }};
        }

        macro_rules! parse_s {
            ($inst:ident $visitor:ident) => {{
                let span = $visitor.span();
                let rs1 = $visitor.expect_node::<OperandR>();
                let imm = $visitor.expect_leaf().1.as_str().parse().unwrap();
                let _ = $visitor.expect_leaf();     // '='
                let rs2 = $visitor.expect_node::<OperandR>();
                InlineAsmInst::$inst { span, rs1, rs2, imm }
            }};
        }

        macro_rules! parse_r {
            ($inst:ident $visitor:ident) => {{
                let span = $visitor.span();
                let rd = $visitor.expect_node::<OperandL>();
                let _ = $visitor.expect_leaf();     // '='
                let rs1 = $visitor.expect_node::<OperandR>();
                let rs2 = $visitor.expect_node::<OperandR>();
                InlineAsmInst::$inst { span, rd, rs1, rs2 }
            }};
        }

        macro_rules! parse_b {
            ($inst:ident $visitor:ident) => {{
                InlineAsmInst::$inst {
                    span: $visitor.span(),
                    rd: $visitor.expect_node::<OperandL>(),
                    rs1: $visitor.expect_node::<OperandR>(),
                    rs2: $visitor.expect_node::<OperandR>(),
                    imm: $visitor.expect_leaf().1.as_str().parse().unwrap(),
                }
            }};
        }

        match visitor.expect_leaf().1.as_str().to_lowercase().as_str() {
            // I-形式
            "addi" => parse_i!(Addi visitor),
            "subi" => parse_i!(Subi visitor),
            "jal"  => parse_i!(Jal visitor),
            "lw"   => parse_i!(Lw visitor),
            "lh"   => parse_i!(Lh visitor),
            "lb"   => parse_i!(Lb visitor),
            "lhu"  => parse_i!(Lhu visitor),
            "lbu"  => parse_i!(Lbu visitor),
            "in"   => parse_i!(In visitor),
            "andi" => parse_i!(Andi visitor),
            "ori"  => parse_i!(Ori visitor),
            "xori" => parse_i!(Xori visitor),
            "srli" => parse_i!(Srli visitor),
            "srai" => parse_i!(Srai visitor),
            "slli" => parse_i!(Slli visitor),

            // S-形式
            "sw"   => parse_s!(Sw visitor),
            "sh"   => parse_s!(Sh visitor),
            "sb"   => parse_s!(Sb visitor),
            "isb"  => parse_s!(Isb visitor),
            "out"  => parse_s!(Out visitor),

            // R-形式
            "add"  => parse_r!(Add visitor),
            "sub"  => parse_r!(Sub visitor),
            "and"  => parse_r!(And visitor),
            "or"   => parse_r!(Or visitor),
            "xor"  => parse_r!(Xor visitor),
            "srl"  => parse_r!(Srl visitor),
            "sra"  => parse_r!(Sra visitor),
            "sll"  => parse_r!(Sll visitor),

            // B-形式
            "beq"  => parse_b!(Beq visitor),
            "bne"  => parse_b!(Bne visitor),
            "blt"  => parse_b!(Blt visitor),
            "ble"  => parse_b!(Ble visitor),

            inst => panic!("Unexpected inline assembly instruction: {:?}", inst)
        }
    }
}

impl<'src> Spanned<'src> for InlineAsmInst<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            InlineAsmInst::Addi { span, .. } => *span,
            InlineAsmInst::Subi { span, .. } => *span,
            InlineAsmInst::Jal  { span, .. } => *span,
            InlineAsmInst::Lw   { span, .. } => *span,
            InlineAsmInst::Lh   { span, .. } => *span,
            InlineAsmInst::Lb   { span, .. } => *span,
            InlineAsmInst::Lhu  { span, .. } => *span,
            InlineAsmInst::Lbu  { span, .. } => *span,
            InlineAsmInst::In   { span, .. } => *span,
            InlineAsmInst::Andi { span, .. } => *span,
            InlineAsmInst::Ori  { span, .. } => *span,
            InlineAsmInst::Xori { span, .. } => *span,
            InlineAsmInst::Srli { span, .. } => *span,
            InlineAsmInst::Srai { span, .. } => *span,
            InlineAsmInst::Slli { span, .. } => *span,
            InlineAsmInst::Sw   { span, .. } => *span,
            InlineAsmInst::Sh   { span, .. } => *span,
            InlineAsmInst::Sb   { span, .. } => *span,
            InlineAsmInst::Isb  { span, .. } => *span,
            InlineAsmInst::Out  { span, .. } => *span,
            InlineAsmInst::Add  { span, .. } => *span,
            InlineAsmInst::Sub  { span, .. } => *span,
            InlineAsmInst::And  { span, .. } => *span,
            InlineAsmInst::Or   { span, .. } => *span,
            InlineAsmInst::Xor  { span, .. } => *span,
            InlineAsmInst::Srl  { span, .. } => *span,
            InlineAsmInst::Sra  { span, .. } => *span,
            InlineAsmInst::Sll  { span, .. } => *span,
            InlineAsmInst::Beq  { span, .. } => *span,
            InlineAsmInst::Bne  { span, .. } => *span,
            InlineAsmInst::Blt  { span, .. } => *span,
            InlineAsmInst::Ble  { span, .. } => *span,
        }
    }
}
