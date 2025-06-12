use super::Visitor;

#[derive(Debug)]
pub enum InlineAsmInst {
    // I-形式
    Addi { namespace: String, rd: String, rs1: String, imm: i32 },
    Subi { namespace: String, rd: String, rs1: String, imm: i32 },
    Jal  { namespace: String, rd: String, rs1: String, imm: i32 },
    Lw   { namespace: String, rd: String, rs1: String, imm: i32 },
    Lh   { namespace: String, rd: String, rs1: String, imm: i32 },
    Lb   { namespace: String, rd: String, rs1: String, imm: i32 },
    Lhu  { namespace: String, rd: String, rs1: String, imm: i32 },
    Lbu  { namespace: String, rd: String, rs1: String, imm: i32 },
    In   { namespace: String, rd: String, rs1: String, imm: i32 },
    Andi { namespace: String, rd: String, rs1: String, imm: i32 },
    Ori  { namespace: String, rd: String, rs1: String, imm: i32 },
    Xori { namespace: String, rd: String, rs1: String, imm: i32 },
    Srli { namespace: String, rd: String, rs1: String, imm: i32 },
    Srai { namespace: String, rd: String, rs1: String, imm: i32 },
    Slli { namespace: String, rd: String, rs1: String, imm: i32 },

    // S-形式
    Sw   { namespace: String, rs1: String, rs2: String, imm: i32 },
    Sh   { namespace: String, rs1: String, rs2: String, imm: i32 },
    Sb   { namespace: String, rs1: String, rs2: String, imm: i32 },
    Isb  { namespace: String, rs1: String, rs2: String, imm: i32 },
    Out  { namespace: String, rs1: String, rs2: String, imm: i32 },

    // R-形式
    Add  { namespace: String, rd: String, rs1: String, rs2: String },
    Sub  { namespace: String, rd: String, rs1: String, rs2: String },
    And  { namespace: String, rd: String, rs1: String, rs2: String },
    Or   { namespace: String, rd: String, rs1: String, rs2: String },
    Xor  { namespace: String, rd: String, rs1: String, rs2: String },
    Srl  { namespace: String, rd: String, rs1: String, rs2: String },
    Sra  { namespace: String, rd: String, rs1: String, rs2: String },
    Sll  { namespace: String, rd: String, rs1: String, rs2: String },

    // B-形式
    Beq  { namespace: String, rd: String, rs1: String, rs2: String, imm: i32 },
    Bne  { namespace: String, rd: String, rs1: String, rs2: String, imm: i32 },
    Blt  { namespace: String, rd: String, rs1: String, rs2: String, imm: i32 },
    Ble  { namespace: String, rd: String, rs1: String, rs2: String, imm: i32 },
}

impl From<(String, Visitor<'_>)> for InlineAsmInst {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        macro_rules! parse_i {
            (Jal $visitor:ident) => {{
                InlineAsmInst::Jal {
                    namespace: namespace.clone(),
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

                InlineAsmInst::$inst {
                    namespace: namespace.clone(),
                    rd,
                    rs1,
                    imm,
                }
            }};
        }

        macro_rules! parse_s {
            ($inst:ident $visitor:ident) => {{
                let rs1 = $visitor.expect_leaf().1.to_string();
                let imm = $visitor.expect_leaf().1.parse().unwrap();
                let _ = $visitor.expect_leaf();     // '='
                let rs2 = $visitor.expect_leaf().1.to_string();

                InlineAsmInst::$inst {
                    namespace: namespace.clone(),
                    rs1,
                    rs2,
                    imm,
                }
            }};
        }

        macro_rules! parse_r {
            ($inst:ident $visitor:ident) => {{
                let rd = $visitor.expect_leaf().1.to_string();
                let _ = $visitor.expect_leaf();     // '='
                let rs1 = $visitor.expect_leaf().1.to_string();
                let rs2 = $visitor.expect_leaf().1.to_string();

                InlineAsmInst::$inst {
                    namespace: namespace.clone(),
                    rd,
                    rs1,
                    rs2,
                }
            }};
        }

        macro_rules! parse_b {
            ($inst:ident $visitor:ident) => {{
                InlineAsmInst::$inst {
                    namespace: namespace.clone(),
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
