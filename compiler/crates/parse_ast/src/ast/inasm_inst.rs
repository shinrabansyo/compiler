use super::Visitor;

#[derive(Debug)]
pub enum InlineAsmInst {
    // I-形式
    Addi { rd: String, rs1: String, imm: i32 },
    Subi { rd: String, rs1: String, imm: i32 },
    Jal  { rd: String, rs1: String, imm: i32 },
    Lw   { rd: String, rs1: String, imm: i32 },
    Lh   { rd: String, rs1: String, imm: i32 },
    Lb   { rd: String, rs1: String, imm: i32 },
    Lhu  { rd: String, rs1: String, imm: i32 },
    Lbu  { rd: String, rs1: String, imm: i32 },
    In   { rd: String, rs1: String, imm: i32 },
    Andi { rd: String, rs1: String, imm: i32 },
    Ori  { rd: String, rs1: String, imm: i32 },
    Xori { rd: String, rs1: String, imm: i32 },
    Srli { rd: String, rs1: String, imm: i32 },
    Srai { rd: String, rs1: String, imm: i32 },
    Slli { rd: String, rs1: String, imm: i32 },

    // S-形式
    Sw   { rs1: String, rs2: String, imm: i32 },
    Sh   { rs1: String, rs2: String, imm: i32 },
    Sb   { rs1: String, rs2: String, imm: i32 },
    Isb  { rs1: String, rs2: String, imm: i32 },
    Out  { rs1: String, rs2: String, imm: i32 },

    // R-形式
    Add  { rd: String, rs1: String, rs2: String },
    Sub  { rd: String, rs1: String, rs2: String },
    And  { rd: String, rs1: String, rs2: String },
    Or   { rd: String, rs1: String, rs2: String },
    Xor  { rd: String, rs1: String, rs2: String },
    Srl  { rd: String, rs1: String, rs2: String },
    Sra  { rd: String, rs1: String, rs2: String },
    Sll  { rd: String, rs1: String, rs2: String },

    // B-形式
    Beq  { rd: String, rs1: String, rs2: String, imm: i32 },
    Bne  { rd: String, rs1: String, rs2: String, imm: i32 },
    Blt  { rd: String, rs1: String, rs2: String, imm: i32 },
    Ble  { rd: String, rs1: String, rs2: String, imm: i32 },
}

impl From<Visitor<'_>> for InlineAsmInst {
    fn from(mut visitor: Visitor<'_>) -> Self {
        macro_rules! parse_i {
            (Jal $visitor:ident) => {{
                InlineAsmInst::Jal {
                    rd: $visitor.expect_leaf().1.to_string(),
                    rs1: $visitor.expect_leaf().1.to_string(),
                    imm: $visitor.expect_leaf().1.parse().unwrap(),
                }
            }};

            ($inst:ident $visitor:ident) => {{
                let rd = $visitor.expect_leaf().1.to_string();
                let _ = $visitor.expect_leaf();     // '='
                let rs1 = $visitor.expect_leaf().1.to_string();
                let imm = $visitor.expect_leaf().1.parse().unwrap();
                InlineAsmInst::$inst { rd, rs1, imm }
            }};
        }

        macro_rules! parse_s {
            ($inst:ident $visitor:ident) => {{
                let rs1 = $visitor.expect_leaf().1.to_string();
                let imm = $visitor.expect_leaf().1.parse().unwrap();
                let _ = $visitor.expect_leaf();     // '='
                let rs2 = $visitor.expect_leaf().1.to_string();
                InlineAsmInst::$inst { rs1, rs2, imm }
            }};
        }

        macro_rules! parse_r {
            ($inst:ident $visitor:ident) => {{
                let rd = $visitor.expect_leaf().1.to_string();
                let _ = $visitor.expect_leaf();     // '='
                let rs1 = $visitor.expect_leaf().1.to_string();
                let rs2 = $visitor.expect_leaf().1.to_string();
                InlineAsmInst::$inst { rd, rs1, rs2 }
            }};
        }

        macro_rules! parse_b {
            ($inst:ident $visitor:ident) => {{
                InlineAsmInst::$inst {
                    rd: $visitor.expect_leaf().1.to_string(),
                    rs1: $visitor.expect_leaf().1.to_string(),
                    rs2: $visitor.expect_leaf().1.to_string(),
                    imm: $visitor.expect_leaf().1.parse().unwrap(),
                }
            }};
        }

        match visitor.expect_leaf().1.to_lowercase().as_str() {
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
