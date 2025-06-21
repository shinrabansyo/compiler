use sb_compiler_lirgen_ir::{lir, LirBlock, Beq, Bne, Blt, Ble, JmpLabel, Li};
use sb_compiler_semcheck_hir::Cond;

use crate::{GenContext, ZERO_REG};
use super::lirgen_bit_shift;

pub fn lirgen_cond(ctx: &mut GenContext, cond: &Cond) -> LirBlock {
    let (result_reg, lirs) = match cond {
        Cond::Eq { lhs, rhs, .. } => {
            let lir_lhs = lirgen_cond(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_bit_shift(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            let label_false = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Beq(12) ZERO_REG, reg_lhs, reg_rhs),
                    lir!(JmpLabel(label_false)),
                    lir!(Li(1) reg_result),
                    lir!(JmpLabel(label_end)),
                    lir!(Label label_false),
                    lir!(Li(0) reg_result),
                    lir!(Label label_end),
                ],
            )
        }
        Cond::Neq { lhs, rhs, .. } => {
            let lir_lhs = lirgen_cond(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_bit_shift(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            let label_false = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Bne(12) ZERO_REG, reg_lhs, reg_rhs),
                    lir!(JmpLabel(label_false)),
                    lir!(Li(1) reg_result),
                    lir!(JmpLabel(label_end)),
                    lir!(Label label_false),
                    lir!(Li(0) reg_result),
                    lir!(Label label_end),
                ],
            )
        }
        Cond::Lt { lhs, rhs, .. } => {
            let lir_lhs = lirgen_cond(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_bit_shift(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            let label_false = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Blt(12) ZERO_REG, reg_lhs, reg_rhs),
                    lir!(JmpLabel(label_false)),
                    lir!(Li(1) reg_result),
                    lir!(JmpLabel(label_end)),
                    lir!(Label label_false),
                    lir!(Li(0) reg_result),
                    lir!(Label label_end),
                ],
            )
        }
        Cond::Lte { lhs, rhs, .. } => {
            let lir_lhs = lirgen_cond(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_bit_shift(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            let label_false = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Ble(12) ZERO_REG, reg_lhs, reg_rhs),
                    lir!(JmpLabel(label_false)),
                    lir!(Li(1) reg_result),
                    lir!(JmpLabel(label_end)),
                    lir!(Label label_false),
                    lir!(Li(0) reg_result),
                    lir!(Label label_end),
                ],
            )
        }
        Cond::Gt { lhs, rhs, .. } => {
            let lir_lhs = lirgen_cond(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_bit_shift(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            let label_false = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Blt(12) ZERO_REG, reg_rhs, reg_lhs),
                    lir!(JmpLabel(label_false)),
                    lir!(Li(1) reg_result),
                    lir!(JmpLabel(label_end)),
                    lir!(Label label_false),
                    lir!(Li(0) reg_result),
                    lir!(Label label_end),
                ],
            )
        }
        Cond::Gte { lhs, rhs, .. } => {
            let lir_lhs = lirgen_cond(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_bit_shift(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            let label_false = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Ble(12) ZERO_REG, reg_rhs, reg_lhs),
                    lir!(JmpLabel(label_false)),
                    lir!(Li(1) reg_result),
                    lir!(JmpLabel(label_end)),
                    lir!(Label label_false),
                    lir!(Li(0) reg_result),
                    lir!(Label label_end),
                ],
            )
        }
        Cond::BitShift { bit_shift, .. } => {
            return lirgen_bit_shift(ctx, bit_shift);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
