mod gen;

use sb_compiler_parse_ast::Program;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::LIR;

pub fn lirgen<'ast>(program: &'ast Program, analyze_result: AnalyzeResult<'ast>) -> Vec<LIR> {
    let mut lirs = vec![];
    gen::lirgen_program(&mut lirs, program, &analyze_result);
    lirs
}
