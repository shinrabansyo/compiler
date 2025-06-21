use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_semcheck_hir::Program;

use crate::GenContext;
use super::lirgen_top;

pub fn lirgen_program(ctx: &mut GenContext, program: &Program) -> Vec<LirTopElem> {
    program.top_elems
        .iter()
        .map(|ast| lirgen_top(ctx, ast))
        .collect()
}
