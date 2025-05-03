use sb_compiler_parse_ast::BitAnd;
use sb_compiler_lirgen_ir::{lir, LirTree, And};

use crate::GenContext;
use super::lirgen_cond;

pub fn lirgen_bit_and(ctx: &mut GenContext, bit_and: &BitAnd) -> LirTree {
    let reserved_reg_range_start = ctx.reserved_regs;
    let reserved_label_range_start = ctx.reserved_labels;

    let lirs = match bit_and {
        BitAnd::And { lhs, rhs, .. } => {
            let lir_lhs = lirgen_bit_and(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_cond(ctx, rhs);
            let reg_rhs = lir_rhs.reserved_reg_range().1 - 1;

            vec![
                lir_lhs,
                lir_rhs,
                lir!(And ctx.alloc_reg(), reg_lhs, reg_rhs),
            ]
        }
        BitAnd::Cond{ cond, .. } => {
            return lirgen_cond(ctx, cond);
        }
    };

    let reserved_reg_range = (reserved_reg_range_start, ctx.reserved_regs);
    let reserved_label_range = (reserved_label_range_start, ctx.reserved_labels);

    LirTree::Node {
        reserved_reg_range,
        reserved_label_range,
        lirs,
    }
}
