use sb_compiler_parse_ast::LogicAnd;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Li, Beq, Push, Pop};

use super::{lirgen_bit_or, TMP_REG, TMP_REG_L, TMP_REG_R, ZERO_REG};

pub fn lirgen_logic_and(lirs: &mut Vec<LIR>, logic_and: &LogicAnd, analyze_result: &AnalyzeResult) {
    match logic_and {
        LogicAnd::And { lhs, rhs, .. } => {
            lirgen_logic_and(lirs, lhs, analyze_result);
            lirgen_bit_or(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Beq ZERO_REG, TMP_REG_L, 24));
            lirs.push(lir!(Beq ZERO_REG, TMP_REG_R, 18));
            lirs.push(lir!(Li TMP_REG, 1));
            lirs.push(lir!(Beq ZERO_REG, ZERO_REG, 12));
            lirs.push(lir!(Li TMP_REG, 0));
            lirs.push(lir!(Push TMP_REG));
        }
        LogicAnd::BitOr{ or, .. } => {
            lirgen_bit_or(lirs, or, analyze_result);
        }
    }
}
