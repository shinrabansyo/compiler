use std::collections::HashSet;

use sb_linker::obj::inst::{Inst, InstValue};
use sb_linker::obj::{inst, Object};

pub fn downsize_prologue_epilogue(obj: Object) -> Object {
    Object {
        code: downsize(obj.code),
        ..obj
    }
}

fn downsize(insts: Vec<Inst>) -> Vec<Inst> {
    let reg_using = analyze_reg_using(&insts);

    let mut prologue_evacuation_size = 4;
    let mut epilogue_evacuation_size = 4;
    let mut result = vec![];
    for inst in insts {
        // プロローグでのレジスタ退避を書き換え
        if let (true, reg) = is_prologue_evacuation(&inst) {
            if reg_using.contains(&reg) {
                prologue_evacuation_size += 4;
                result.push(inst!(Sw 3, reg, -prologue_evacuation_size ));
            }
        }
        // エピローグでのレジスタ復帰を書き換え
        else if let (true, reg) = is_epilogue_evacuation(&inst) {
            if reg_using.contains(&reg) {
                epilogue_evacuation_size += 4;
                result.push(inst!(Lw reg, 3, -epilogue_evacuation_size ));
            }
        }
        // フレーム確保サイズを書き換え
        else if let Inst::Subi { rd: 2, rs1: 2, value: InstValue::Imm(44) } = inst {
            result.push(inst!(Subi 2, 2, InstValue::Imm(prologue_evacuation_size)));
        }
        // フレーム解放サイズを書き換え
        else if let Inst::Addi { rd: 2, rs1: 2, value: InstValue::Imm(48) } = inst {
            result.push(inst!(Addi 2, 2, InstValue::Imm(epilogue_evacuation_size + 4)));
        }
        // その他の命令
        else {
            result.push(inst);
        }
    }

    result
}

fn analyze_reg_using(insts: &[Inst]) -> HashSet<u8> {
    macro_rules! record {
        ($hashset:ident <= $r1:expr, $r2:expr, $r3:expr) => {{
            $hashset.insert(*$r1);
            $hashset.insert(*$r2);
            $hashset.insert(*$r3);
        }};

        ($hashset:ident <= $r1:expr, $r2:expr) => {{
            $hashset.insert(*$r1);
            $hashset.insert(*$r2);
        }};
    }

    let mut used_regs = HashSet::new();
    for inst in insts {
        if let (true, _) = is_prologue_evacuation(inst) {
            continue;
        }
        if let (true, _) = is_epilogue_evacuation(inst) {
            continue;
        }
        match inst {
            // R-形式
            Inst::Add { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            Inst::Sub { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            Inst::And { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            Inst::Or { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            Inst::Xor { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            Inst::Srl { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            Inst::Sra { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            Inst::Sll { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),

            // I-形式
            Inst::Addi { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Subi { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Andi { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Ori { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Xori { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Srli { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Srai { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Slli { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Lb { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Lbu { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Lh { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Lhu { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Lw { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::Jal { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            Inst::In { rd, rs1, .. } => record!(used_regs <= rd, rs1),

            // B-形式
            Inst::Beq { rd, rs1, rs2, .. } => record!(used_regs <= rd, rs1, rs2),
            Inst::Bne { rd, rs1, rs2, .. } => record!(used_regs <= rd, rs1, rs2),
            Inst::Blt { rd, rs1, rs2, .. } => record!(used_regs <= rd, rs1, rs2),
            Inst::Ble { rd, rs1, rs2, .. } => record!(used_regs <= rd, rs1, rs2),

            // S-形式
            Inst::Sb { rs1, rs2, .. } => record!(used_regs <= rs1, rs2),
            Inst::Sh { rs1, rs2, .. } => record!(used_regs <= rs1, rs2),
            Inst::Sw { rs1, rs2, .. } => record!(used_regs <= rs1, rs2),
            Inst::Out { rs1, rs2, .. } => record!(used_regs <= rs1, rs2),

            _ => {}
        }
    }

    used_regs
}

fn is_prologue_evacuation(inst: &Inst) -> (bool, u8) {
    match inst {
        Inst::Sw { rs1: 3, rs2, imm } => {
            (*rs2 >= 20 && *rs2 < 30 && *imm < -4 && *imm % 4 == 0, *rs2)
        }
        _ => (false, 0),
    }
}

fn is_epilogue_evacuation(inst: &Inst) -> (bool, u8) {
    match inst {
        Inst::Lw { rd, rs1: 3, imm } => {
            (*rd >= 20 && *rd < 30 && *imm < -4 && *imm % 4 == 0, *rd)
        }
        _ => (false, 0),
    }
}
