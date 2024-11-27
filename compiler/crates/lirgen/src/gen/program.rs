use sb_compiler_parse_ast::Program;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::LIR;

use super::lirgen_top;

pub fn lirgen_program(lirs: &mut Vec<LIR>, program: &Program, analyze_result: &AnalyzeResult) {
    for top in &program.top_elems {
        lirgen_top(lirs, top, analyze_result);
    }
}
