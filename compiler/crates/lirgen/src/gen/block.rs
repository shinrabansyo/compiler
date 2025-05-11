use sb_compiler_parse_ast::Block;
use sb_compiler_lirgen_ir::LirTree;

use crate::{GenContext, ZERO_REG};
use super::lirgen_stmt;

pub fn lirgen_block(ctx: &mut GenContext, block: &Block) -> LirTree {
    let reserved_reg_range_start = ctx.reserved_regs;
    let reserved_label_range_start = ctx.reserved_labels;

    let lirs = block.stmts
        .iter()
        .map(|ast| lirgen_stmt(ctx, ast))
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
