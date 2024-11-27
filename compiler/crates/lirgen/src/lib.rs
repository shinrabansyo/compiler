mod gen;

use sb_compiler_parse_ast::AST;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::LIR;

pub fn lirgen<'ast>(ast: &'ast AST, analyze_result: AnalyzeResult<'ast>) -> Vec<LIR> {
    let mut lirs = vec![];
    gen::lirgen_ast(&mut lirs, ast,  &analyze_result);
    lirs
}
