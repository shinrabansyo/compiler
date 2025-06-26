use sb_compiler_lirgen_ir::{lir, LirBlock, And};
use sb_compiler_semcheck_hir::BitAnd;

use crate::GenContext;
use super::lirgen_cond;

pub fn lirgen_bit_and(ctx: &mut GenContext, bit_and: BitAnd) -> LirBlock {
    let (result_reg, lirs) = match bit_and {
        BitAnd::And { lhs, rhs, .. } => {
            let lir_lhs = lirgen_bit_and(ctx, *lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_cond(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(And reg_result, reg_lhs, reg_rhs),
                ],
            )
        }
        BitAnd::Cond{ cond, .. } => {
            return lirgen_cond(ctx, cond);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
