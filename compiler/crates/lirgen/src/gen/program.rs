use sb_compiler_parse_ast::Program;
use sb_compiler_lirgen_ir::LirTree;

use crate::GenContext;
use super::lirgen_top;

pub fn lirgen_program(program: &Program) -> Vec<LirTree> {
    let mut ctx = GenContext::default();
    program.top_elems
        .iter()
        .map(|ast| lirgen_top(&mut ctx, ast))
        .collect()
}
