use sb_compiler_parse_ast::Expr;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Add, Sub, Push, Pop};

use super::{lirgen_value, TMP_REG_L, TMP_REG_R};

pub fn lirgen_expr(lirs: &mut Vec<LIR>, expr: &Expr, analyze_result: &AnalyzeResult) {
    match expr {
        Expr::Plus { lhs, rhs, .. }=> {
            lirgen_expr(lirs, lhs, analyze_result);
            lirgen_value(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop: TMP_REG_R));
            lirs.push(lir!(Pop: TMP_REG_L));
            lirs.push(lir!(Add: TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Push: TMP_REG_L));
        }
        Expr::Minus { lhs, rhs, .. } => {
            lirgen_expr(lirs, lhs, analyze_result);
            lirgen_value(lirs, rhs, analyze_result);
            lirs.push(lir!(Pop: TMP_REG_R));
            lirs.push(lir!(Pop: TMP_REG_L));
            lirs.push(lir!(Sub: TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Push: TMP_REG_L));
        }
        Expr::Value { value, .. } => {
            lirgen_value(lirs, value,  analyze_result);
        }
    }
}
