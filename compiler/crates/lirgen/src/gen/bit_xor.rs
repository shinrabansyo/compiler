use sb_compiler_parse_ast::BitXor;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Xor, Push, Pop};

use super::{lirgen_bit_and, TMP_REG_L, TMP_REG_R};

pub fn lirgen_bit_xor(lirs: &mut Vec<LIR>, bit_xor: &BitXor, analyze_result: &AnalyzeResult) {
    match bit_xor {
        BitXor::Xor { lhs, rhs, .. } => {
            lirgen_bit_xor(lirs, lhs, analyze_result);
            lirgen_bit_and(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Xor TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Push TMP_REG_L));
        }
        BitXor::BitAnd { and, .. } => {
            lirgen_bit_and(lirs, and, analyze_result);
        }
    }
}
