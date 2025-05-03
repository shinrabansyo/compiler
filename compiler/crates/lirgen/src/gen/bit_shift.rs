use sb_compiler_parse_ast::BitShift;
use sb_compiler_lirgen_ir::{lir, LirTree, ShiftL, ShiftR, ShiftRa};

use crate::GenContext;
use super::lirgen_add;

pub fn lirgen_bit_shift(ctx: &mut GenContext, bit_shift: &BitShift) -> LirTree {
    let reserved_reg_range_start = ctx.reserved_regs;
    let reserved_label_range_start = ctx.reserved_labels;

    let lirs = match bit_shift {
        BitShift::L { lhs, rhs, .. } => {
            let lir_lhs = lirgen_bit_shift(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_add(ctx, rhs);
            let reg_rhs = lir_rhs.reserved_reg_range().1 - 1;

            vec![
                lir_lhs,
                lir_rhs,
                lir!(ShiftL ctx.alloc_reg(), reg_lhs, reg_rhs),
            ]
        }
        BitShift::R { lhs, rhs, .. } => {
            let lir_lhs = lirgen_bit_shift(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_add(ctx, rhs);
            let reg_rhs = lir_rhs.reserved_reg_range().1 - 1;

            vec![
                lir_lhs,
                lir_rhs,
                lir!(ShiftR ctx.alloc_reg(), reg_lhs, reg_rhs),
            ]
        }
        BitShift::Ra { lhs, rhs, .. } => {
            let lir_lhs = lirgen_bit_shift(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_add(ctx, rhs);
            let reg_rhs = lir_rhs.reserved_reg_range().1 - 1;

            vec![
                lir_lhs,
                lir_rhs,
                lir!(ShiftRa ctx.alloc_reg(), reg_lhs, reg_rhs),
            ]
        }
        BitShift::Add { add, .. } => {
            return lirgen_add(ctx, add);
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
