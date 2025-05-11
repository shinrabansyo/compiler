use sb_compiler_parse_ast::Program;
use sb_compiler_lirgen_ir::LirTree;

use crate::{GenContext, ZERO_REG};
use super::lirgen_top;

pub fn lirgen_program(ctx: &mut GenContext, program: &Program) -> LirTree {
    let lirs = program.top_elems
        .iter()
        .map(|ast| lirgen_top(ctx, ast))
        .collect();

    LirTree::Single {
        result_reg: ZERO_REG,
        lirs,
    }
}
