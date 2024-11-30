use sb_compiler_parse_ast::Cond;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Li, Beq, Bne, Blt, Ble, Push, Pop};

use super::{lirgen_bit_shift, TMP_REG, TMP_REG_L, TMP_REG_R, ZERO_REG};

pub fn lirgen_cond(lirs: &mut Vec<LIR>, cond: &Cond, analyze_result: &AnalyzeResult) {
    match cond {
        Cond::Eq { lhs, rhs, .. }=> {
            lirgen_cond(lirs, lhs, analyze_result);
            lirgen_bit_shift(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Bne TMP_REG_L, TMP_REG_R, 18));  // !(TMP_REG_L == TMP_REG_R) = TMP_REG_L != TMP_REG_R
            lirs.push(lir!(Li TMP_REG, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG, 0));
            lirs.push(lir!(Push TMP_REG));
        }
        Cond::Neq { lhs, rhs, .. } => {
            lirgen_cond(lirs, lhs, analyze_result);
            lirgen_bit_shift(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Beq TMP_REG_L, TMP_REG_R, 18));  // !(TMP_REG_L != TMP_REG_R) = TMP_REG_L == TMP_REG_R
            lirs.push(lir!(Li TMP_REG, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG, 0));
            lirs.push(lir!(Push TMP_REG));
        }
        Cond::Lt { lhs, rhs, .. } => {
            lirgen_cond(lirs, lhs, analyze_result);
            lirgen_bit_shift(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Ble TMP_REG_R, TMP_REG_L, 18));  // !(TMP_REG_L <= TMP_REG_R) = TMP_REG_R < TMP_REG_L
            lirs.push(lir!(Li TMP_REG, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG, 0));
            lirs.push(lir!(Push TMP_REG));
        }
        Cond::Lte { lhs, rhs, .. } => {
            lirgen_cond(lirs, lhs, analyze_result);
            lirgen_bit_shift(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Blt TMP_REG_R, TMP_REG_L, 18));  // !(TMP_REG_L <= TMP_REG_R) = TMP_REG_R < TMP_REG_L
            lirs.push(lir!(Li TMP_REG, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG, 0));
            lirs.push(lir!(Push TMP_REG));
        }
        Cond::Gt { lhs, rhs, .. } => {
            lirgen_cond(lirs, lhs, analyze_result);
            lirgen_bit_shift(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Ble TMP_REG_L, TMP_REG_R, 18));  // !(TMP_REG_L > TMP_REG_R) = TMP_REG_L <= TMP_REG_R
            lirs.push(lir!(Li TMP_REG, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG, 0));
            lirs.push(lir!(Push TMP_REG));
        }
        Cond::Gte { lhs, rhs, .. } => {
            lirgen_cond(lirs, lhs, analyze_result);
            lirgen_bit_shift(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Blt TMP_REG_L, TMP_REG_R, 18));  // !(TMP_REG_L >= TMP_REG_R) = TMP_REG_L < TMP_REG_R
            lirs.push(lir!(Li TMP_REG, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG, 0));
            lirs.push(lir!(Push TMP_REG));
        }
        Cond::BitShift { bit_shift, .. } => {
            lirgen_bit_shift(lirs, bit_shift,  analyze_result);
        }
    }
}
