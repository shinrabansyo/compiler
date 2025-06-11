use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::{unwrap_node, unwrap_leaf};

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

impl From<(String, Tree<'_, SBLangDef>)> for InlineAsmInst {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        macro_rules! parse_i {
            (Jal $args:ident) => {{
                let (_, rd) = unwrap_leaf($args.pop_front().unwrap());
                let (_, rs1) = unwrap_leaf($args.pop_front().unwrap());
                let (_, imm) = unwrap_leaf($args.pop_front().unwrap());

                InlineAsmInst::Jal {
                    namespace: namespace.clone(),
                    rd: rd.to_string(),
                    rs1: rs1.to_string(),
                    imm: imm.parse().unwrap(),
                }
            }};

            ($inst:ident $args:ident) => {{
                let (_, rd) = unwrap_leaf($args.pop_front().unwrap());
                let _ = $args.pop_front().unwrap();
                let (_, rs1) = unwrap_leaf($args.pop_front().unwrap());
                let (_, imm) = unwrap_leaf($args.pop_front().unwrap());

                InlineAsmInst::$inst {
                    namespace: namespace.clone(),
                    rd: rd.to_string(),
                    rs1: rs1.to_string(),
                    imm: imm.parse().unwrap(),
                }
            }};
        }

        macro_rules! parse_s {
            ($inst:ident $args:ident) => {{
                let (_, rs1) = unwrap_leaf($args.pop_front().unwrap());
                let (_, imm) = unwrap_leaf($args.pop_front().unwrap());
                let _ = $args.pop_front().unwrap();
                let (_, rs2) = unwrap_leaf($args.pop_front().unwrap());

                InlineAsmInst::$inst {
                    namespace: namespace.clone(),
                    rs1: rs1.to_string(),
                    rs2: rs2.to_string(),
                    imm: imm.parse().unwrap(),
                }
            }};
        }

        macro_rules! parse_r {
            ($inst:ident $args:ident) => {{
                let (_, rd) = unwrap_leaf($args.pop_front().unwrap());
                let _ = $args.pop_front().unwrap();
                let (_, rs1) = unwrap_leaf($args.pop_front().unwrap());
                let (_, rs2) = unwrap_leaf($args.pop_front().unwrap());

                InlineAsmInst::$inst {
                    namespace: namespace.clone(),
                    rd: rd.to_string(),
                    rs1: rs1.to_string(),
                    rs2: rs2.to_string(),
                }
            }};
        }

        macro_rules! parse_b {
            ($inst:ident $args:ident) => {{
                let (_, rd) = unwrap_leaf($args.pop_front().unwrap());
                let (_, rs1) = unwrap_leaf($args.pop_front().unwrap());
                let (_, rs2) = unwrap_leaf($args.pop_front().unwrap());
                let (_, imm) = unwrap_leaf($args.pop_front().unwrap());

                InlineAsmInst::$inst {
                    namespace: namespace.clone(),
                    rd: rd.to_string(),
                    rs1: rs1.to_string(),
                    rs2: rs2.to_string(),
                    imm: imm.parse().unwrap(),
                }
            }};
        }

        let (_, mut children) = unwrap_node(tree);

        let (_, inst) = unwrap_leaf(children.pop_front().unwrap());
        match inst.to_lowercase().as_str() {
            // I-形式
            "addi" => parse_i!(Addi children),
            "subi" => parse_i!(Subi children),
            "jal"  => parse_i!(Jal children),
            "lw"   => parse_i!(Lw children),
            "lh"   => parse_i!(Lh children),
            "lb"   => parse_i!(Lb children),
            "lhu"  => parse_i!(Lhu children),
            "lbu"  => parse_i!(Lbu children),
            "andi" => parse_i!(Andi children),
            "ori"  => parse_i!(Ori children),
            "xori" => parse_i!(Xori children),
            "srli" => parse_i!(Srli children),
            "srai" => parse_i!(Srai children),
            "slli" => parse_i!(Slli children),

            // S-形式
            "sw"   => parse_s!(Sw children),
            "sh"   => parse_s!(Sh children),
            "sb"   => parse_s!(Sb children),
            "isb"  => parse_s!(Isb children),

            // R-形式
            "add"  => parse_r!(Add children),
            "sub"  => parse_r!(Sub children),
            "and"  => parse_r!(And children),
            "or"   => parse_r!(Or children),
            "xor"  => parse_r!(Xor children),
            "srl"  => parse_r!(Srl children),
            "sra"  => parse_r!(Sra children),
            "sll"  => parse_r!(Sll children),

            // B-形式
            "beq"  => parse_b!(Beq children),
            "bne"  => parse_b!(Bne children),
            "blt"  => parse_b!(Blt children),
            "ble"  => parse_b!(Ble children),

            _ => panic!("Unexpected inline assembly instruction: {:?}", inst)
        }
    }
}
