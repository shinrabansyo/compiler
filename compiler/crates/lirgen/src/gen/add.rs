use sb_compiler_parse_ast::Add as AddAst;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Add, Sub, Push, Pop};

use super::{lirgen_value, TMP_REG_L, TMP_REG_R};

pub fn lirgen_add(lirs: &mut Vec<LIR>, add: &AddAst, analyze_result: &AnalyzeResult) {
    match add {
        AddAst::Plus { lhs, rhs, .. }=> {
            lirgen_add(lirs, lhs, analyze_result);
            lirgen_value(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Add TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Push TMP_REG_L));
        }
        AddAst::Minus { lhs, rhs, .. } => {
            lirgen_add(lirs, lhs, analyze_result);
            lirgen_value(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Sub TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Push TMP_REG_L));
        }
        AddAst::Value { value, .. } => {
            lirgen_value(lirs, value,  analyze_result);
        }
    }
}
