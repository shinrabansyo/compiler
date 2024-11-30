use sb_compiler_parse_ast::BitShift;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, ShiftL, ShiftR, ShiftRa, Push, Pop};

use super::{lirgen_add, TMP_REG_L, TMP_REG_R};

pub fn lirgen_bit_shift(lirs: &mut Vec<LIR>, bit_shift: &BitShift, analyze_result: &AnalyzeResult) {
    match bit_shift {
        BitShift::L { lhs, rhs, .. }=> {
            lirgen_bit_shift(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(ShiftL TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Push TMP_REG_L));
        }
        BitShift::R { lhs, rhs, .. } => {
            lirgen_bit_shift(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(ShiftR TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Push TMP_REG_L));
        }
        BitShift::Ra { lhs, rhs, .. } => {
            lirgen_bit_shift(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(ShiftRa TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Push TMP_REG_L));
        }
        BitShift::Add { add, .. } => {
            lirgen_add(lirs, add,  analyze_result);
        }
    }
}
