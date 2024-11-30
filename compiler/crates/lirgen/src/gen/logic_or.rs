use sb_compiler_parse_ast::LogicOr;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Li, Beq, Bne, Push, Pop};

use super::{lirgen_logic_and, TMP_REG, TMP_REG_L, TMP_REG_R, ZERO_REG};

pub fn lirgen_logic_or(lirs: &mut Vec<LIR>, logic_or: &LogicOr, analyze_result: &AnalyzeResult) {
    match logic_or {
        LogicOr::Or { lhs, rhs, .. } => {
            lirgen_logic_or(lirs, lhs, analyze_result);
            lirgen_logic_and(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Bne ZERO_REG, TMP_REG_L, 24));
            lirs.push(lir!(Bne ZERO_REG, TMP_REG_R, 18));
            lirs.push(lir!(Li TMP_REG, 0));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG, 1));
            lirs.push(lir!(Push TMP_REG));
        }
        LogicOr::LogicAnd { and, .. } => {
            lirgen_logic_and(lirs, and, analyze_result);
        }
    }
}
