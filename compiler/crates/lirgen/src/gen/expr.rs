use sb_compiler_parse_ast::Expr;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::LIR;

use super::lirgen_logic_or;

pub fn lirgen_expr(lirs: &mut Vec<LIR>, expr: &Expr, analyze_result: &AnalyzeResult) {
    lirgen_logic_or(lirs, &expr.or, analyze_result);
}
