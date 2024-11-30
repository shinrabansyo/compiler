use sb_compiler_parse_ast::DevIO;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Push, Pop, In, Out};

use super::{lirgen_expr, TMP_REG_L, TMP_REG_R};

pub fn lirgen_dev_io(lirs: &mut Vec<LIR>, dev_io: &DevIO, analyze_result: &AnalyzeResult) {
    match dev_io {
        DevIO::In { id, .. } => {
            lirgen_expr(lirs, id, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirs.push(lir!(In TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Push TMP_REG_L));
        }
        DevIO::Out { id, expr, .. } => {
            lirgen_expr(lirs, expr, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));
            lirgen_expr(lirs, id, analyze_result);
            lirs.push(lir!(Pop TMP_REG_L));
            lirs.push(lir!(Out TMP_REG_L, TMP_REG_R));
        }
    }
}
