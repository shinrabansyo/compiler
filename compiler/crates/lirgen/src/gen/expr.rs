use sb_compiler_parse_ast::Expr;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::LIR;

use super::lirgen_assign;

pub fn lirgen_expr(lirs: &mut Vec<LIR>, expr: &Expr, analyze_result: &AnalyzeResult) {
    lirgen_assign(lirs, &expr.assign, analyze_result);
}
