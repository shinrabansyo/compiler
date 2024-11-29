use sb_compiler_parse_ast::Block;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::LIR;

use super::lirgen_stmt;

pub fn lirgen_block(lirs: &mut Vec<LIR>, block: &Block, analyze_result: &AnalyzeResult) {
    for stmt in &block.stmts {
        lirgen_stmt(lirs, stmt, analyze_result);
    }
}
