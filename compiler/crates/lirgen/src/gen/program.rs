use sb_compiler_parse_ast::Program;
use sb_compiler_lirgen_ir::LirTree;

use crate::{GenContext, ZERO_REG};
use super::lirgen_top;

pub fn lirgen_program(ctx: &mut GenContext, program: &Program) -> LirTree {
    let reserved_reg_range_start = ctx.reserved_regs;
    let reserved_label_range_start = ctx.reserved_labels;

    let lirs = program.top_elems
        .iter()
        .map(|ast| lirgen_top(ctx, ast))
        .collect();

    let reserved_reg_range = (reserved_reg_range_start, ctx.reserved_regs);
    let reserved_label_range = (reserved_label_range_start, ctx.reserved_labels);

    LirTree::Node {
        reserved_reg_range,
        reserved_label_range,
        result_reg: ZERO_REG,
        lirs,
    }
}
