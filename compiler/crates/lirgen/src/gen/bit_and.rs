use sb_compiler_parse_ast::BitAnd;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, And, Push, Pop};

use super::{lirgen_cond, TMP_REG_L, TMP_REG_R};

pub fn lirgen_bit_and(lirs: &mut Vec<LIR>, bit_and: &BitAnd, analyze_result: &AnalyzeResult) {
    match bit_and {
        BitAnd::And { lhs, rhs, .. } => {
            lirgen_bit_and(lirs, lhs, analyze_result);
            lirgen_cond(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(And TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Push TMP_REG_L));
        }
        BitAnd::Cond { cond, .. } => {
            lirgen_cond(lirs, cond, analyze_result);
        }
    }
}
