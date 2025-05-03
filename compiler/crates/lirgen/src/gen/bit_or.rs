use sb_compiler_parse_ast::BitOr;
use sb_compiler_lirgen_ir::{lir, LirTree, Or};

use crate::GenContext;
use super::lirgen_bit_xor;

pub fn lirgen_bit_or(ctx: &mut GenContext, bit_or: &BitOr) -> LirTree {
    let reserved_reg_range_start = ctx.reserved_regs;
    let reserved_label_range_start = ctx.reserved_labels;

    let lirs = match bit_or {
        BitOr::Or { lhs, rhs, .. } => {
            let lir_lhs = lirgen_bit_or(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_bit_xor(ctx, rhs);
            let reg_rhs = lir_lhs.reserved_reg_range().1 - 1;

            vec![
                lir_lhs,
                lir_rhs,
                lir!(Or ctx.alloc_reg(), reg_lhs, reg_rhs),
            ]
        }
        BitOr::BitXor { xor, .. } => {
            return lirgen_bit_xor(ctx, xor);
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
