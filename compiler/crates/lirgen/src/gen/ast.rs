use sb_compiler_parse_ast::AST;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::LIR;

use super::lirgen_top;

pub fn lirgen_ast(lirs: &mut Vec<LIR>, ast: &AST, analyze_result: &AnalyzeResult) {
    for top in &ast.top_elems {
        lirgen_top(lirs, top, analyze_result);
    }
}
