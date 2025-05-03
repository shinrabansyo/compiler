use sb_compiler_parse_ast::Cond;
use sb_compiler_lirgen_ir::{lir, LirTree, Beq, Bne, Blt, Ble, Jmp, Li};

use crate::{GenContext, ZERO_REG};
use super::lirgen_bit_shift;

pub fn lirgen_cond(ctx: &mut GenContext, cond: &Cond) -> LirTree {
    let reserved_reg_range_start = ctx.reserved_regs;
    let reserved_label_range_start = ctx.reserved_labels;

    let lirs = match cond {
        Cond::Eq { lhs, rhs, .. } => {
            let lir_lhs = lirgen_cond(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_bit_shift(ctx, rhs);
            let reg_rhs = lir_rhs.reserved_reg_range().1 - 1;

            let reg_result = ctx.alloc_reg();

            vec![
                lir_lhs,
                lir_rhs,
                lir!(Beq(18) ZERO_REG, reg_lhs, reg_rhs),
                lir!(Li(0) reg_result),
                lir!(Jmp(12)),
                lir!(Li(1) reg_result),
            ]
        }
        Cond::Neq { lhs, rhs, .. } => {
            let lir_lhs = lirgen_cond(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_bit_shift(ctx, rhs);
            let reg_rhs = lir_rhs.reserved_reg_range().1 - 1;

            let reg_result = ctx.alloc_reg();

            vec![
                lir_lhs,
                lir_rhs,
                lir!(Bne(18) ZERO_REG, reg_lhs, reg_rhs),
                lir!(Li(0) reg_result),
                lir!(Jmp(12)),
                lir!(Li(1) reg_result),
            ]
        }
        Cond::Lt { lhs, rhs, .. } => {
            let lir_lhs = lirgen_cond(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_bit_shift(ctx, rhs);
            let reg_rhs = lir_rhs.reserved_reg_range().1 - 1;

            let reg_result = ctx.alloc_reg();

            vec![
                lir_lhs,
                lir_rhs,
                lir!(Blt(18) ZERO_REG, reg_lhs, reg_rhs),
                lir!(Li(0) reg_result),
                lir!(Jmp(12)),
                lir!(Li(1) reg_result),
            ]
        }
        Cond::Lte { lhs, rhs, .. } => {
            let lir_lhs = lirgen_cond(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_bit_shift(ctx, rhs);
            let reg_rhs = lir_rhs.reserved_reg_range().1 - 1;

            let reg_result = ctx.alloc_reg();

            vec![
                lir_lhs,
                lir_rhs,
                lir!(Ble(18) ZERO_REG, reg_lhs, reg_rhs),
                lir!(Li(0) reg_result),
                lir!(Jmp(12)),
                lir!(Li(1) reg_result),
            ]
        }
        Cond::Gt { lhs, rhs, .. } => {
            let lir_lhs = lirgen_cond(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_bit_shift(ctx, rhs);
            let reg_rhs = lir_rhs.reserved_reg_range().1 - 1;

            let reg_result = ctx.alloc_reg();

            vec![
                lir_lhs,
                lir_rhs,
                lir!(Blt(18) ZERO_REG, reg_rhs, reg_lhs),
                lir!(Li(0) reg_result),
                lir!(Jmp(12)),
                lir!(Li(1) reg_result),
            ]
        }
        Cond::Gte { lhs, rhs, .. } => {
            let lir_lhs = lirgen_cond(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_bit_shift(ctx, rhs);
            let reg_rhs = lir_rhs.reserved_reg_range().1 - 1;

            let reg_result = ctx.alloc_reg();

            vec![
                lir_lhs,
                lir_rhs,
                lir!(Ble(18) ZERO_REG, reg_rhs, reg_lhs),
                lir!(Li(0) reg_result),
                lir!(Jmp(12)),
                lir!(Li(1) reg_result),
            ]
        }
        Cond::BitShift { bit_shift, .. } => {
            return lirgen_bit_shift(ctx, bit_shift);
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
