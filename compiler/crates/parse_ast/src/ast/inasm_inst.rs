use super::{InlineAsmOperandL, InlineAsmOperandR, Visitor};


#[derive(Debug)]
pub enum InlineAsmInst<'src> {
    // I-形式
    Addi { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Subi { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Jal  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Lw   { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Lh   { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Lb   { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Lhu  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Lbu  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    In   { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Andi { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Ori  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Xori { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Srli { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Srai { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },
    Slli { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, imm: i32 },

    // S-形式
    Sw   { rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src>, imm: i32 },
    Sh   { rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src>, imm: i32 },
    Sb   { rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src>, imm: i32 },
    Isb  { rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src>, imm: i32 },
    Out  { rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src>, imm: i32 },

    // R-形式
    Add  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src> },
    Sub  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src> },
    And  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src> },
    Or   { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src> },
    Xor  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src> },
    Srl  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src> },
    Sra  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src> },
    Sll  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src> },

    // B-形式
    Beq  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src>, imm: i32 },
    Bne  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src>, imm: i32 },
    Blt  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src>, imm: i32 },
    Ble  { rd: InlineAsmOperandL<'src>, rs1: InlineAsmOperandR<'src>, rs2: InlineAsmOperandR<'src>, imm: i32 },
}

impl<'src> From<Visitor<'src>> for InlineAsmInst<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        macro_rules! parse_i {
            (Jal $visitor:ident) => {{
                InlineAsmInst::Jal {
                    rd: $visitor.expect_node::<InlineAsmOperandL>(),
                    rs1: $visitor.expect_node::<InlineAsmOperandR>(),
                    imm: $visitor.expect_leaf().1.as_str().parse().unwrap(),
                }
            }};

            ($inst:ident $visitor:ident) => {{
                let rd = $visitor.expect_node::<InlineAsmOperandL>();
                let _ = $visitor.expect_leaf();  // '='
                let rs1 = $visitor.expect_node::<InlineAsmOperandR>();
                let imm = $visitor.expect_leaf().1.as_str().parse().unwrap();
                InlineAsmInst::$inst { rd, rs1, imm }
            }};
        }

        macro_rules! parse_s {
            ($inst:ident $visitor:ident) => {{
                let rs1 = $visitor.expect_node::<InlineAsmOperandR>();
                let imm = $visitor.expect_leaf().1.as_str().parse().unwrap();
                let _ = $visitor.expect_leaf();     // '='
                let rs2 = $visitor.expect_node::<InlineAsmOperandR>();
                InlineAsmInst::$inst { rs1, rs2, imm }
            }};
        }

        macro_rules! parse_r {
            ($inst:ident $visitor:ident) => {{
                let rd = $visitor.expect_node::<InlineAsmOperandL>();
                let _ = $visitor.expect_leaf();     // '='
                let rs1 = $visitor.expect_node::<InlineAsmOperandR>();
                let rs2 = $visitor.expect_node::<InlineAsmOperandR>();
                InlineAsmInst::$inst { rd, rs1, rs2 }
            }};
        }

        macro_rules! parse_b {
            ($inst:ident $visitor:ident) => {{
                InlineAsmInst::$inst {
                    rd: $visitor.expect_node::<InlineAsmOperandL>(),
                    rs1: $visitor.expect_node::<InlineAsmOperandR>(),
                    rs2: $visitor.expect_node::<InlineAsmOperandR>(),
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
