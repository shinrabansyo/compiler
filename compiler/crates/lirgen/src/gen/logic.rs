use sb_compiler_parse_ast::Logic;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Li, Beq, Bne, Blt, Ble, Push, Pop};

use super::{lirgen_add, TMP_REG_L, TMP_REG_R, ZERO_REG};

pub fn lirgen_logic(lirs: &mut Vec<LIR>, logic: &Logic, analyze_result: &AnalyzeResult) {
    match logic {
        Logic::Eq { lhs, rhs, .. }=> {
            lirgen_logic(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Bne TMP_REG_L, TMP_REG_R, 18));  // !(TMP_REG_L == TMP_REG_R) = TMP_REG_L != TMP_REG_R
            lirs.push(lir!(Li TMP_REG_L, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG_L, 0));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Logic::Neq { lhs, rhs, .. } => {
            lirgen_logic(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Beq TMP_REG_L, TMP_REG_R, 18));  // !(TMP_REG_L != TMP_REG_R) = TMP_REG_L == TMP_REG_R
            lirs.push(lir!(Li TMP_REG_L, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG_L, 0));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Logic::Lt { lhs, rhs, .. } => {
            lirgen_logic(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Ble TMP_REG_R, TMP_REG_L, 18));  // !(TMP_REG_L <= TMP_REG_R) = TMP_REG_R < TMP_REG_L
            lirs.push(lir!(Li TMP_REG_L, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG_L, 0));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Logic::Lte { lhs, rhs, .. } => {
            lirgen_logic(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Blt TMP_REG_R, TMP_REG_L, 18));  // !(TMP_REG_L <= TMP_REG_R) = TMP_REG_R < TMP_REG_L
            lirs.push(lir!(Li TMP_REG_L, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG_L, 0));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Logic::Gt { lhs, rhs, .. } => {
            lirgen_logic(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Ble TMP_REG_L, TMP_REG_R, 18));  // !(TMP_REG_L > TMP_REG_R) = TMP_REG_L <= TMP_REG_R
            lirs.push(lir!(Li TMP_REG_L, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG_L, 0));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Logic::Gte { lhs, rhs, .. } => {
            lirgen_logic(lirs, lhs, analyze_result);
            lirgen_add(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Blt TMP_REG_L, TMP_REG_R, 18));  // !(TMP_REG_L >= TMP_REG_R) = TMP_REG_L < TMP_REG_R
            lirs.push(lir!(Li TMP_REG_L, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG_L, 0));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Logic::Add { add, .. } => {
            lirgen_add(lirs, add,  analyze_result);
        }
    }
}
