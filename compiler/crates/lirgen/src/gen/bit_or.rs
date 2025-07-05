use sb_compiler_lirgen_ir::{lir, LirBlock, Or};
use sb_compiler_semcheck_hir::BitOr;

use crate::GenContext;
use super::lirgen_bit_xor;

pub fn lirgen_bit_or<'src>(ctx: &mut GenContext<'src>, bit_or: BitOr<'src>) -> LirBlock {
    let (result_reg, lirs) = match bit_or {
        BitOr::Or { lhs, rhs, .. } => {
            let lir_lhs = lirgen_bit_or(ctx, *lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_bit_xor(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Or reg_result, reg_lhs, reg_rhs),
                ],
            )
        }
        BitOr::BitXor { xor, .. } => {
            return lirgen_bit_xor(ctx, xor);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
