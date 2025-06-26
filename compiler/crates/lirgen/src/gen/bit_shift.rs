use sb_compiler_lirgen_ir::{lir, LirBlock, ShiftL, ShiftR, ShiftRa};
use sb_compiler_semcheck_hir::BitShift;

use crate::GenContext;
use super::lirgen_add;

pub fn lirgen_bit_shift(ctx: &mut GenContext, bit_shift: BitShift) -> LirBlock {
    let (result_reg, lirs) = match bit_shift {
        BitShift::L { lhs, rhs, .. } => {
            let lir_lhs = lirgen_bit_shift(ctx, *lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_add(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(ShiftL reg_result, reg_lhs, reg_rhs),
                ],
            )
        }
        BitShift::R { lhs, rhs, .. } => {
            let lir_lhs = lirgen_bit_shift(ctx, *lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_add(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(ShiftR reg_result, reg_lhs, reg_rhs),
                ],
            )
        }
        BitShift::Ra { lhs, rhs, .. } => {
            let lir_lhs = lirgen_bit_shift(ctx, *lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_add(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(ShiftRa reg_result, reg_lhs, reg_rhs),
                ],
            )
        }
        BitShift::Add { add, .. } => {
            return lirgen_add(ctx, add);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
