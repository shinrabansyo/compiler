use sb_compiler_parse_ast::Cond;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Li, Beq, Bne, Blt, Ble, Push, Pop};

use super::{lirgen_add, TMP_REG_L, TMP_REG_R, ZERO_REG};

pub fn lirgen_cond(lirs: &mut Vec<LIR>, cond: &Cond, analyze_result: &AnalyzeResult) {
    match cond {
        Cond::Eq { lhs, rhs, .. }=> {
            lirgen_cond(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Bne TMP_REG_L, TMP_REG_R, 18));  // !(TMP_REG_L == TMP_REG_R) = TMP_REG_L != TMP_REG_R
            lirs.push(lir!(Li TMP_REG_L, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG_L, 0));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Cond::Neq { lhs, rhs, .. } => {
            lirgen_cond(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Beq TMP_REG_L, TMP_REG_R, 18));  // !(TMP_REG_L != TMP_REG_R) = TMP_REG_L == TMP_REG_R
            lirs.push(lir!(Li TMP_REG_L, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG_L, 0));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Cond::Lt { lhs, rhs, .. } => {
            lirgen_cond(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Ble TMP_REG_R, TMP_REG_L, 18));  // !(TMP_REG_L <= TMP_REG_R) = TMP_REG_R < TMP_REG_L
            lirs.push(lir!(Li TMP_REG_L, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG_L, 0));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Cond::Lte { lhs, rhs, .. } => {
            lirgen_cond(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Blt TMP_REG_R, TMP_REG_L, 18));  // !(TMP_REG_L <= TMP_REG_R) = TMP_REG_R < TMP_REG_L
            lirs.push(lir!(Li TMP_REG_L, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG_L, 0));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Cond::Gt { lhs, rhs, .. } => {
            lirgen_cond(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Ble TMP_REG_L, TMP_REG_R, 18));  // !(TMP_REG_L > TMP_REG_R) = TMP_REG_L <= TMP_REG_R
            lirs.push(lir!(Li TMP_REG_L, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG_L, 0));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Cond::Gte { lhs, rhs, .. } => {
            lirgen_cond(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Blt TMP_REG_L, TMP_REG_R, 18));  // !(TMP_REG_L >= TMP_REG_R) = TMP_REG_L < TMP_REG_R
            lirs.push(lir!(Li TMP_REG_L, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG_L, 0));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Cond::Add { add, .. } => {
            lirgen_add(lirs, add,  analyze_result);
        }
    }
}
