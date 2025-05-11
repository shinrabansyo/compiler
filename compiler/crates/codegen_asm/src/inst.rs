use std::fmt::Display;

pub use AsmInst::*;
pub use AsmInstValue::*;

#[macro_export]
macro_rules! asmi {
    // R-形式
    (Add $rd:expr, $rs1:expr, $rs2:expr) => { Add { rd: $rd, rs1: $rs1, rs2: $rs2 } };
    (Sub $rd:expr, $rs1:expr, $rs2:expr) => { Sub { rd: $rd, rs1: $rs1, rs2: $rs2 } };
    (And $rd:expr, $rs1:expr, $rs2:expr) => { And { rd: $rd, rs1: $rs1, rs2: $rs2 } };
    (Or  $rd:expr, $rs1:expr, $rs2:expr) => { Or  { rd: $rd, rs1: $rs1, rs2: $rs2 } };
    (Xor $rd:expr, $rs1:expr, $rs2:expr) => { Xor { rd: $rd, rs1: $rs1, rs2: $rs2 } };
    (Srl $rd:expr, $rs1:expr, $rs2:expr) => { Srl { rd: $rd, rs1: $rs1, rs2: $rs2 } };
    (Sra $rd:expr, $rs1:expr, $rs2:expr) => { Sra { rd: $rd, rs1: $rs1, rs2: $rs2 } };
    (Sll $rd:expr, $rs1:expr, $rs2:expr) => { Sll { rd: $rd, rs1: $rs1, rs2: $rs2 } };

    // I-形式
    (Addi $rd:expr, $rs1:expr, $value:expr) => { Addi { rd: $rd, rs1: $rs1, value: $value } };
    (Subi $rd:expr, $rs1:expr, $value:expr) => { Subi { rd: $rd, rs1: $rs1, value: $value } };
    (Andi $rd:expr, $rs1:expr, $value:expr) => { Andi { rd: $rd, rs1: $rs1, value: $value } };
    (Ori  $rd:expr, $rs1:expr, $value:expr) => { Ori  { rd: $rd, rs1: $rs1, value: $value } };
    (Xori $rd:expr, $rs1:expr, $value:expr) => { Xori { rd: $rd, rs1: $rs1, value: $value } };
    (Srli $rd:expr, $rs1:expr, $value:expr) => { Srli { rd: $rd, rs1: $rs1, value: $value } };
    (Srai $rd:expr, $rs1:expr, $value:expr) => { Srai { rd: $rd, rs1: $rs1, value: $value } };
    (Slli $rd:expr, $rs1:expr, $value:expr) => { Slli { rd: $rd, rs1: $rs1, value: $value } };
    (Lb   $rd:expr, $rs1:expr, $imm:expr) => { Lb  { rd: $rd, rs1: $rs1, imm: $imm } };
    (Lbu  $rd:expr, $rs1:expr, $imm:expr) => { Lbu { rd: $rd, rs1: $rs1, imm: $imm } };
    (Lh   $rd:expr, $rs1:expr, $imm:expr) => { Lh  { rd: $rd, rs1: $rs1, imm: $imm } };
    (Lhu  $rd:expr, $rs1:expr, $imm:expr) => { Lhu { rd: $rd, rs1: $rs1, imm: $imm } };
    (Lw   $rd:expr, $rs1:expr, $imm:expr) => { Lw  { rd: $rd, rs1: $rs1, imm: $imm } };
    (Jal  $rd:expr, $rs1:expr, $imm:expr) => { Jal { rd: $rd, rs1: $rs1, imm: $imm } };
    (In   $rd:expr, $rs1:expr, $imm:expr) => { In  { rd: $rd, rs1: $rs1, imm: $imm } };

    // B-形式
    (Beq $rd:expr, $rs1:expr, $rs2:expr, $value:expr) => { Beq { rd: $rd, rs1: $rs1, rs2: $rs2, value: $value } };
    (Bne $rd:expr, $rs1:expr, $rs2:expr, $value:expr) => { Bne { rd: $rd, rs1: $rs1, rs2: $rs2, value: $value } };
    (Blt $rd:expr, $rs1:expr, $rs2:expr, $value:expr) => { Blt { rd: $rd, rs1: $rs1, rs2: $rs2, value: $value } };
    (Ble $rd:expr, $rs1:expr, $rs2:expr, $value:expr) => { Ble { rd: $rd, rs1: $rs1, rs2: $rs2, value: $value } };

    // S-形式
    (Sb  $rs1:expr, $rs2:expr, $imm:expr) => { Sb  { rs1: $rs1, rs2: $rs2, imm: $imm } };
    (Sh  $rs1:expr, $rs2:expr, $imm:expr) => { Sh  { rs1: $rs1, rs2: $rs2, imm: $imm } };
    (Sw  $rs1:expr, $rs2:expr, $imm:expr) => { Sw  { rs1: $rs1, rs2: $rs2, imm: $imm } };
    (Out $rs1:expr, $rs2:expr, $imm:expr) => { Out { rs1: $rs1, rs2: $rs2, imm: $imm } };

    // ラベル
    (LLabel $label:expr) => { LLabel { label: $label } };
    (GLabel $label:expr) => { GLabel { label: $label } };
}

#[derive(Debug, Clone)]
pub enum AsmInst {
    // R-形式
    Add { rd: u8, rs1: u8, rs2: u8 },
    Sub { rd: u8, rs1: u8, rs2: u8 },
    And { rd: u8, rs1: u8, rs2: u8 },
    Or  { rd: u8, rs1: u8, rs2: u8 },
    Xor { rd: u8, rs1: u8, rs2: u8 },
    Srl { rd: u8, rs1: u8, rs2: u8 },
    Sra { rd: u8, rs1: u8, rs2: u8 },
    Sll { rd: u8, rs1: u8, rs2: u8 },

    // I-形式
    Addi { rd: u8, rs1: u8, value: AsmInstValue },
    Subi { rd: u8, rs1: u8, value: AsmInstValue },
    Andi { rd: u8, rs1: u8, value: AsmInstValue },
    Ori  { rd: u8, rs1: u8, value: AsmInstValue },
    Xori { rd: u8, rs1: u8, value: AsmInstValue },
    Srli { rd: u8, rs1: u8, value: AsmInstValue },
    Srai { rd: u8, rs1: u8, value: AsmInstValue },
    Slli { rd: u8, rs1: u8, value: AsmInstValue },
    Lb   { rd: u8, rs1: u8, imm: i32 },
    Lbu  { rd: u8, rs1: u8, imm: i32 },
    Lh   { rd: u8, rs1: u8, imm: i32 },
    Lhu  { rd: u8, rs1: u8, imm: i32 },
    Lw   { rd: u8, rs1: u8, imm: i32 },
    Jal  { rd: u8, rs1: u8, imm: i32 },
    In   { rd: u8, rs1: u8, imm: i32 },

    // B-形式
    Beq { rd: u8, rs1: u8, rs2: u8, value: AsmInstValue },
    Bne { rd: u8, rs1: u8, rs2: u8, value: AsmInstValue },
    Blt { rd: u8, rs1: u8, rs2: u8, value: AsmInstValue },
    Ble { rd: u8, rs1: u8, rs2: u8, value: AsmInstValue },

    // S-形式
    Sb  { rs1: u8, rs2: u8, imm: i32 },
    Sh  { rs1: u8, rs2: u8, imm: i32 },
    Sw  { rs1: u8, rs2: u8, imm: i32 },
    Out { rs1: u8, rs2: u8, imm: i32 },

    // ラベル
    LLabel { label: u32 },
    GLabel { label: String },
}

impl Display for AsmInst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // R-形式
            AsmInst::Add { rd, rs1, rs2 } => {
                write!(f, "    add r{} = r{}, r{}", rd, rs1, rs2)
            }
            AsmInst::Sub { rd, rs1, rs2 } => {
                write!(f, "    sub r{} = r{}, r{}", rd, rs1, rs2)
            }
            AsmInst::And { rd, rs1, rs2 } => {
                write!(f, "    and r{} = r{}, r{}", rd, rs1, rs2)
            }
            AsmInst::Or { rd, rs1, rs2 } => {
                write!(f, "    or r{} = r{}, r{}", rd, rs1, rs2)
            }
            AsmInst::Xor { rd, rs1, rs2 } => {
                write!(f, "    xor r{} = r{}, r{}", rd, rs1, rs2)
            }
            AsmInst::Srl { rd, rs1, rs2 } => {
                write!(f, "    srl r{} = r{}, r{}", rd, rs1, rs2)
            }
            AsmInst::Sra { rd, rs1, rs2 } => {
                write!(f, "    sra r{} = r{}, r{}", rd, rs1, rs2)
            }
            AsmInst::Sll { rd, rs1, rs2 } => {
                write!(f, "    sll r{} = r{}, r{}", rd, rs1, rs2)
            }

            // I-形式
            AsmInst::Addi { rd, rs1, value } => {
                write!(f, "    addi r{} = r{}, {}", rd, rs1, value)
            }
            AsmInst::Subi { rd, rs1, value } => {
                write!(f, "    subi r{} = r{}, {}", rd, rs1, value)
            }
            AsmInst::Andi { rd, rs1, value } => {
                write!(f, "    andi r{} = r{}, {}", rd, rs1, value)
            }
            AsmInst::Ori { rd, rs1, value } => {
                write!(f, "    ori r{} = r{}, {}", rd, rs1, value)
            }
            AsmInst::Xori { rd, rs1, value } => {
                write!(f, "    xori r{} = r{}, {}", rd, rs1, value)
            }
            AsmInst::Srli { rd, rs1, value } => {
                write!(f, "    srli r{} = r{}, {}", rd, rs1, value)
            }
            AsmInst::Srai { rd, rs1, value } => {
                write!(f, "    srai r{} = r{}, {}", rd, rs1, value)
            }
            AsmInst::Slli { rd, rs1, value } => {
                write!(f, "    slli r{} = r{}, {}", rd, rs1, value)
            }
            AsmInst::Lb { rd, rs1, imm } => {
                write!(f, "    lb r{} = r{}[{}]", rd, rs1, imm)
            }
            AsmInst::Lbu { rd, rs1, imm } => {
                write!(f, "    lbu r{} = r{}[{}]", rd, rs1, imm)
            }
            AsmInst::Lh { rd, rs1, imm } => {
                write!(f, "    lh r{} = r{}[{}]", rd, rs1, imm)
            }
            AsmInst::Lhu { rd, rs1, imm } => {
                write!(f, "    lhu r{} = r{}[{}]", rd, rs1, imm)
            }
            AsmInst::Lw { rd, rs1, imm } => {
                write!(f, "    lw r{} = r{}[{}]", rd, rs1, imm)
            }
            AsmInst::Jal { rd, rs1, imm } => {
                write!(f, "    jal r{} = r{}[{}]", rd, rs1, imm)
            }
            AsmInst::In { rd, rs1, imm } => {
                write!(f, "    in r{} = r{}[{}]", rd, rs1, imm)
            }

            // B-形式
            AsmInst::Beq { rd, rs1, rs2, value } => {
                write!(f, "    beq r{}, (r{}, r{}) -> {}", rd, rs1, rs2, value)
            }
            AsmInst::Bne { rd, rs1, rs2, value } => {
                write!(f, "    bne r{}, (r{}, r{}) -> {}", rd, rs1, rs2, value)
            }
            AsmInst::Blt { rd, rs1, rs2, value } => {
                write!(f, "    blt r{}, (r{}, r{}) -> {}", rd, rs1, rs2, value)
            }
            AsmInst::Ble { rd, rs1, rs2, value } => {
                write!(f, "    ble r{}, (r{}, r{}) -> {}", rd, rs1, rs2, value)
            }

            // S-形式
            AsmInst::Sb { rs1, rs2, imm } => {
                write!(f, "    sb r{}[{}] = r{}", rs1, imm, rs2)
            }
            AsmInst::Sh { rs1, rs2, imm } => {
                write!(f, "    sh r{}[{}] = r{}", rs1, imm, rs2)
            }
            AsmInst::Sw { rs1, rs2, imm } => {
                write!(f, "    sw r{}[{}] = r{}", rs1, imm, rs2)
            }
            AsmInst::Out { rs1, rs2, imm } => {
                write!(f, "    out r{}[{}] = r{}", rs1, imm, rs2)
            }

            // ラベル
            AsmInst::LLabel { label } => {
                write!(f, "@local.{}", label)
            }
            AsmInst::GLabel { .. } => {
                unimplemented!()
            }
        }
    }

}

#[derive(Debug, Clone)]
pub enum AsmInstValue {
    DataLabel(String),
    InstLabel(String),
    Imm(i32),
}

impl Display for AsmInstValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsmInstValue::DataLabel(label) => write!(f, "${}", label),
            AsmInstValue::InstLabel(label) => write!(f, "@{}", label),
            AsmInstValue::Imm(imm) => write!(f, "{}", imm),
        }
    }
}
