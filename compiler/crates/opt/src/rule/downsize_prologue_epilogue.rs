use std::collections::HashSet;

use sb_compiler_codegen_asm::inst::{AsmInst, AsmInstValue, Addi, Lw, Subi, Sw};
use sb_compiler_codegen_asm::{asmi, Asm};

pub fn downsize_prologue_epilogue(asm: Asm) -> Asm {
    let mut insts = asm.inst;
    let mut result = vec![];
    let fn_ranges = detect_fn_range(&insts);
    for idx in 0..fn_ranges.len()-1 {
        let fn_begin = fn_ranges[idx];
        let fn_end = fn_ranges[idx + 1];
        let fn_insts = insts.drain(0..fn_end-fn_begin+1).collect::<Vec<_>>();
        let downsized_insts = downsize(fn_insts);
        result.extend(downsized_insts);
    }

    Asm {
        inst: result,
        .. asm
    }
}

fn detect_fn_range(insts: &[AsmInst]) -> Vec<usize> {
    let mut fn_begin_points = vec![];
    for (idx, inst) in insts.iter().enumerate() {
        if let AsmInst::Subi { rd: 2, rs1: 2, value: AsmInstValue::Imm(4) } = inst {
            fn_begin_points.push(idx);
        }
    }
    fn_begin_points.push(insts.len());
    fn_begin_points
}

fn downsize(insts: Vec<AsmInst>) -> Vec<AsmInst> {
    let reg_using = analyze_reg_using(&insts);

    let mut prologue_evacuation_size = 4;
    let mut epilogue_evacuation_size = 4;
    let mut result = vec![];
    for inst in insts {
        // プロローグでのレジスタ退避を書き換え
        if let (true, reg) = is_prologue_evacuation(&inst) {
            if reg_using.contains(&reg) {
                prologue_evacuation_size += 4;
                result.push(asmi!(Sw 3, reg, -prologue_evacuation_size ));
            }
        }
        // エピローグでのレジスタ復帰を書き換え
        else if let (true, reg) = is_epilogue_evacuation(&inst) {
            if reg_using.contains(&reg) {
                epilogue_evacuation_size += 4;
                result.push(asmi!(Lw reg, 3, -epilogue_evacuation_size ));
            }
        }
        // フレーム確保サイズを書き換え
        else if let AsmInst::Subi { rd: 2, rs1: 2, value: AsmInstValue::Imm(44) } = inst {
            result.push(asmi!(Subi 2, 2, AsmInstValue::Imm(prologue_evacuation_size)));
        }
        // フレーム解放サイズを書き換え
        else if let AsmInst::Addi { rd: 2, rs1: 2, value: AsmInstValue::Imm(48) } = inst {
            result.push(asmi!(Addi 2, 2, AsmInstValue::Imm(epilogue_evacuation_size + 4)));
        }
        // その他の命令
        else {
            result.push(inst);
        }
    }
    result
}

fn analyze_reg_using(insts: &[AsmInst]) -> HashSet<u8> {
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
            AsmInst::Add { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            AsmInst::Sub { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            AsmInst::And { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            AsmInst::Or { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            AsmInst::Xor { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            AsmInst::Srl { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            AsmInst::Sra { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),
            AsmInst::Sll { rd, rs1, rs2 } => record!(used_regs <= rd, rs1, rs2),

            // I-形式
            AsmInst::Addi { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Subi { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Andi { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Ori { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Xori { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Srli { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Srai { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Slli { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Lb { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Lbu { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Lh { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Lhu { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Lw { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::Jal { rd, rs1, .. } => record!(used_regs <= rd, rs1),
            AsmInst::In { rd, rs1, .. } => record!(used_regs <= rd, rs1),

            // B-形式
            AsmInst::Beq { rd, rs1, rs2, .. } => record!(used_regs <= rd, rs1, rs2),
            AsmInst::Bne { rd, rs1, rs2, .. } => record!(used_regs <= rd, rs1, rs2),
            AsmInst::Blt { rd, rs1, rs2, .. } => record!(used_regs <= rd, rs1, rs2),
            AsmInst::Ble { rd, rs1, rs2, .. } => record!(used_regs <= rd, rs1, rs2),

            // S-形式
            AsmInst::Sb { rs1, rs2, .. } => record!(used_regs <= rs1, rs2),
            AsmInst::Sh { rs1, rs2, .. } => record!(used_regs <= rs1, rs2),
            AsmInst::Sw { rs1, rs2, .. } => record!(used_regs <= rs1, rs2),
            AsmInst::Out { rs1, rs2, .. } => record!(used_regs <= rs1, rs2),

            _ => {}
        }
    }

    used_regs
}

fn is_prologue_evacuation(inst: &AsmInst) -> (bool, u8) {
    match inst {
        AsmInst::Sw { rs1: 3, rs2, imm } => {
            (*rs2 >= 20 && *rs2 < 30 && *imm < -4 && *imm % 4 == 0, *rs2)
        }
        _ => (false, 0),
    }
}

fn is_epilogue_evacuation(inst: &AsmInst) -> (bool, u8) {
    match inst {
        AsmInst::Lw { rd, rs1: 3, imm } => {
            (*rd >= 20 && *rd < 30 && *imm < -4 && *imm % 4 == 0, *rd)
        }
        _ => (false, 0),
    }
}
