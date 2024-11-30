use sb_compiler_parse_ast::BitOr;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Or, Push, Pop};

use super::{lirgen_bit_xor, TMP_REG_L, TMP_REG_R};

pub fn lirgen_bit_or(lirs: &mut Vec<LIR>, bit_or: &BitOr, analyze_result: &AnalyzeResult) {
    match bit_or {
        BitOr::Or { lhs, rhs, .. } => {
            lirgen_bit_or(lirs, lhs, analyze_result);
            lirgen_bit_xor(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Or TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Push TMP_REG_L));
        }
        BitOr::BitXor { xor, .. } => {
            lirgen_bit_xor(lirs, xor, analyze_result);
        }
    }
}
