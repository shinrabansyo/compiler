use sb_compiler_parse_ast::Program;
use sb_compiler_lirgen_ir::LirBlock;

use crate::{GenContext, ZERO_REG};
use super::lirgen_top;

pub fn lirgen_program(ctx: &mut GenContext, program: &Program) -> LirBlock {
    let lirs = program.top_elems
        .iter()
        .map(|ast| lirgen_top(ctx, ast))
        .collect();

    LirBlock::Single {
        result_reg: ZERO_REG,
        lirs,
    }
}
